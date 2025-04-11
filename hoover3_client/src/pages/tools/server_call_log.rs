use crate::time::current_time;
use crate::{
    api::ServerCallEvent,
    app::read_server_call_history,
    components::table::{DataRowDisplay, HtmlTable},
};
use dioxus::prelude::*;

impl DataRowDisplay for ServerCallEvent {
    fn get_headers() -> Vec<&'static str> {
        vec![
            "age",
            "function",
            "arg",
            "is_finished",
            "is_successful",
            "ret",
            "duration",
        ]
    }

    fn render_cell(&self, header_name: &str) -> Element {
        let x = match header_name {
            "age" => {
                format!("{}s", (current_time() - self.ts).round())
            }
            "function" => self.function.clone(),
            "arg" => self.arg.clone(),
            "is_finished" => self.is_finished.to_string(),
            "is_successful" => self.is_successful.to_string(),
            "ret" => self.ret.clone(),
            "duration" => {
                format!("{}ms", (self.duration * 1000.0).round())
            }
            _ => unreachable!(),
        };
        rsx! {pre{"{x}"}}
    }
}

/// Page that displays the server call log table.
#[component]
pub fn ServerCallLogPage() -> Element {
    let hist = read_server_call_history();
    let sorted_data = use_memo(move || {
        let mut v = vec![];
        for (_k, vd) in hist.read().iter() {
            for x in vd.iter() {
                v.push(x.clone())
            }
        }
        v.sort_by(|a, b| a.ts.partial_cmp(&b.ts).unwrap());
        v.reverse();
        v
    });
    rsx! {
        HtmlTable {
            title: "Server Call Log",
            data: sorted_data,
        }
    }
}
