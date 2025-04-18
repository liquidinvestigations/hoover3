//! CLI tool to run a worker;

use std::path::PathBuf;

use hoover3_taskdef::{
    task_inventory::TaskQueue, WORKER_TEMPDIR_ENV_VAR_BIG, WORKER_TEMPDIR_ENV_VAR_SMALL,
};
use hoover3_tracing::tracing::{error, info, warn};

fn main() -> anyhow::Result<()> {
    hoover3_tracing::init_tracing();
    hoover3_server::init_server_plugins()?;
    let arg = std::env::args().nth(1).unwrap_or_default();
    if arg.is_empty() {
        info!("\n\nRunning all workers as subprocesses\n\n");
        main_run_all_workers()?;
        // hoover3_taskdef::tasks::run_workers(hoover3_filesystem_scanner::tasks::FilesystemScannerQueue)?;
    } else {
        info!("\n\nRunning single worker in current process:  {}\n\n", arg);
        run_worker_directly(arg)?;
    }
    Ok(())
}

fn main_run_all_workers() -> anyhow::Result<()> {
    let (quit_tx, quit_rx) = std::sync::mpsc::channel();

    ctrlc::set_handler(move || {
        quit_tx.send("ctrl-c").unwrap();
    })
    .expect("Error setting Ctrl-C handler");

    let queues = hoover3_taskdef::tasks::list_task_queues()
        .map(|q| q.queue_name().to_string())
        .collect::<Vec<_>>();
    info!(
        "main_run_all_workers(): Running workers for task queues: {:#?}",
        queues
    );
    let mut children = Vec::new();
    for q in queues {
        children.push((q.clone(), run_worker_in_subprocess(q)?));
    }

    // wait until one exits
    let (dead_queue, exit_status) = 'outer: loop {
        for (queue, proc) in children.iter_mut() {
            let status = proc.try_wait()?;
            let Some(status) = status else {
                continue;
            };
            error!("Worker {} exited with status {}", queue, status);
            break 'outer (Some(queue.clone()), Some(status));
        }

        if let Ok(msg) = quit_rx.recv_timeout(std::time::Duration::from_millis(2000)) {
            warn!("Quit: Received {}", msg);
            break 'outer (None, None);
        }
    };

    for (queue, mut proc) in children {
        if Some(queue.clone()) != dead_queue {
            warn!("Killing worker {}", queue);
            let _ = proc.kill();
        }
    }
    warn!("All workers killed.");
    warn!("Removing temporary dirs...");
    rm_temp_dirs();

    if let Some(queue) = dead_queue {
        error!("Worker {} exited with status {:?}", queue, exit_status);
    } else {
        info!("Ctrl-C received, exiting.");
    }

    Ok(())
}

fn rm_temp_dirs() {
    let big_tmp = std::env::var(WORKER_TEMPDIR_ENV_VAR_BIG).ok();
    let small_tmp = std::env::var(WORKER_TEMPDIR_ENV_VAR_SMALL).ok();
    for item in vec![big_tmp, small_tmp].iter().flatten() {
        if std::fs::exists(item).unwrap_or(false) {
            let _ = remove_tmp_dir(item);
        }
    }
}

fn remove_tmp_dir(item: &str) -> anyhow::Result<()> {
    let path = PathBuf::from(item).canonicalize()?;
    for item in std::fs::read_dir(path)? {
        let item = item?;
        let path = item.path();
        if path.is_dir() {
            info!("Removing tempdir: {}", path.display());
            std::fs::remove_dir_all(path)?;
        } else if path.is_file() {
            info!("Removing tempfile: {}", path.display());
            std::fs::remove_file(path)?;
        } else {
            warn!("Tempdir: Unknown file type: {}", path.display());
        }
    }
    Ok(())
}
fn run_worker_in_subprocess(queue: String) -> anyhow::Result<std::process::Child> {
    let exe = std::env::current_exe()?;
    let subprocess = std::process::Command::new(exe).arg(queue).spawn()?;
    Ok(subprocess)
}

fn run_worker_directly(arg: String) -> anyhow::Result<()> {
    let matching_queues = hoover3_taskdef::tasks::list_task_queues()
        .filter(|q| q.queue_name() == arg)
        .collect::<Vec<_>>();
    if matching_queues.is_empty() {
        return Err(anyhow::anyhow!("No matching queue found"));
    }
    if matching_queues.len() > 1 {
        return Err(anyhow::anyhow!("Multiple matching queues found"));
    }
    let queue = matching_queues.first().cloned().unwrap();
    hoover3_taskdef::tasks::run_worker(queue)?;
    Ok(())
}
