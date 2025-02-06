(function() {
    var type_impls = Object.fromEntries([["hoover3_database",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-Mutex%3CT%3E\" class=\"impl\"><a href=\"#impl-Debug-for-Mutex%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for Mutex&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.84.1/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","hoover3_database::db_management::nebula::NebulaDatabaseHandle"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Default-for-Mutex%3CT%3E\" class=\"impl\"><a href=\"#impl-Default-for-Mutex%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for Mutex&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.default\" class=\"method trait-impl\"><a href=\"#method.default\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/default/trait.Default.html#tymethod.default\" class=\"fn\">default</a>() -&gt; Mutex&lt;T&gt;</h4></section></summary><div class='docblock'>Returns the “default value” for a type. <a href=\"https://doc.rust-lang.org/1.84.1/core/default/trait.Default.html#tymethod.default\">Read more</a></div></details></div></details>","Default","hoover3_database::db_management::nebula::NebulaDatabaseHandle"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-From%3CT%3E-for-Mutex%3CT%3E\" class=\"impl\"><a href=\"#impl-From%3CT%3E-for-Mutex%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for Mutex&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.from\" class=\"method trait-impl\"><a href=\"#method.from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.84.1/core/convert/trait.From.html#tymethod.from\" class=\"fn\">from</a>(s: T) -&gt; Mutex&lt;T&gt;</h4></section></summary><div class='docblock'>Converts to this type from the input type.</div></details></div></details>","From<T>","hoover3_database::db_management::nebula::NebulaDatabaseHandle"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Mutex%3CT%3E\" class=\"impl\"><a href=\"#impl-Mutex%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; Mutex&lt;T&gt;<div class=\"where\">where\n    T: ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">new</a>(t: T) -&gt; Mutex&lt;T&gt;</h4></section></summary><div class=\"docblock\"><p>Creates a new lock in an unlocked state ready for use.</p>\n<h5 id=\"examples\"><a class=\"doc-anchor\" href=\"#examples\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>tokio::sync::Mutex;\n\n<span class=\"kw\">let </span>lock = Mutex::new(<span class=\"number\">5</span>);</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.const_new\" class=\"method\"><h4 class=\"code-header\">pub const fn <a class=\"fn\">const_new</a>(t: T) -&gt; Mutex&lt;T&gt;</h4></section></summary><div class=\"docblock\"><p>Creates a new lock in an unlocked state ready for use.</p>\n<p>When using the <code>tracing</code> <a href=\"crate#unstable-features\">unstable feature</a>, a <code>Mutex</code> created with\n<code>const_new</code> will not be instrumented. As such, it will not be visible\nin <a href=\"https://github.com/tokio-rs/console\"><code>tokio-console</code></a>. Instead, [<code>Mutex::new</code>] should be used to create\nan instrumented object if that is needed.</p>\n<h5 id=\"examples-1\"><a class=\"doc-anchor\" href=\"#examples-1\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>tokio::sync::Mutex;\n\n<span class=\"kw\">static </span>LOCK: Mutex&lt;i32&gt; = Mutex::const_new(<span class=\"number\">5</span>);</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.lock\" class=\"method\"><h4 class=\"code-header\">pub async fn <a class=\"fn\">lock</a>(&amp;self) -&gt; MutexGuard&lt;'_, T&gt;</h4></section></summary><div class=\"docblock\"><p>Locks this mutex, causing the current task to yield until the lock has\nbeen acquired.  When the lock has been acquired, function returns a\n[<code>MutexGuard</code>].</p>\n<p>If the mutex is available to be acquired immediately, then this call\nwill typically not yield to the runtime. However, this is not guaranteed\nunder all circumstances.</p>\n<h5 id=\"cancel-safety\"><a class=\"doc-anchor\" href=\"#cancel-safety\">§</a>Cancel safety</h5>\n<p>This method uses a queue to fairly distribute locks in the order they\nwere requested. Cancelling a call to <code>lock</code> makes you lose your place in\nthe queue.</p>\n<h5 id=\"examples-2\"><a class=\"doc-anchor\" href=\"#examples-2\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>tokio::sync::Mutex;\n\n<span class=\"attr\">#[tokio::main]\n</span><span class=\"kw\">async fn </span>main() {\n    <span class=\"kw\">let </span>mutex = Mutex::new(<span class=\"number\">1</span>);\n\n    <span class=\"kw\">let </span><span class=\"kw-2\">mut </span>n = mutex.lock().<span class=\"kw\">await</span>;\n    <span class=\"kw-2\">*</span>n = <span class=\"number\">2</span>;\n}</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.blocking_lock\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">blocking_lock</a>(&amp;self) -&gt; MutexGuard&lt;'_, T&gt;</h4></section></summary><div class=\"docblock\"><p>Blockingly locks this <code>Mutex</code>. When the lock has been acquired, function returns a\n[<code>MutexGuard</code>].</p>\n<p>This method is intended for use cases where you\nneed to use this mutex in asynchronous code as well as in synchronous code.</p>\n<h5 id=\"panics\"><a class=\"doc-anchor\" href=\"#panics\">§</a>Panics</h5>\n<p>This function panics if called within an asynchronous execution context.</p>\n<ul>\n<li>If you find yourself in an asynchronous execution context and needing\nto call some (synchronous) function which performs one of these\n<code>blocking_</code> operations, then consider wrapping that call inside\n[<code>spawn_blocking()</code>][crate::runtime::Handle::spawn_blocking]\n(or [<code>block_in_place()</code>][crate::task::block_in_place]).</li>\n</ul>\n<h5 id=\"examples-3\"><a class=\"doc-anchor\" href=\"#examples-3\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>std::sync::Arc;\n<span class=\"kw\">use </span>tokio::sync::Mutex;\n\n<span class=\"attr\">#[tokio::main]\n</span><span class=\"kw\">async fn </span>main() {\n    <span class=\"kw\">let </span>mutex =  Arc::new(Mutex::new(<span class=\"number\">1</span>));\n    <span class=\"kw\">let </span>lock = mutex.lock().<span class=\"kw\">await</span>;\n\n    <span class=\"kw\">let </span>mutex1 = Arc::clone(<span class=\"kw-2\">&amp;</span>mutex);\n    <span class=\"kw\">let </span>blocking_task = tokio::task::spawn_blocking(<span class=\"kw\">move </span>|| {\n        <span class=\"comment\">// This shall block until the `lock` is released.\n        </span><span class=\"kw\">let </span><span class=\"kw-2\">mut </span>n = mutex1.blocking_lock();\n        <span class=\"kw-2\">*</span>n = <span class=\"number\">2</span>;\n    });\n\n    <span class=\"macro\">assert_eq!</span>(<span class=\"kw-2\">*</span>lock, <span class=\"number\">1</span>);\n    <span class=\"comment\">// Release the lock.\n    </span>drop(lock);\n\n    <span class=\"comment\">// Await the completion of the blocking task.\n    </span>blocking_task.<span class=\"kw\">await</span>.unwrap();\n\n    <span class=\"comment\">// Assert uncontended.\n    </span><span class=\"kw\">let </span>n = mutex.try_lock().unwrap();\n    <span class=\"macro\">assert_eq!</span>(<span class=\"kw-2\">*</span>n, <span class=\"number\">2</span>);\n}\n</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.blocking_lock_owned\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">blocking_lock_owned</a>(self: <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;Mutex&lt;T&gt;&gt;) -&gt; OwnedMutexGuard&lt;T&gt;</h4></section></summary><div class=\"docblock\"><p>Blockingly locks this <code>Mutex</code>. When the lock has been acquired, function returns an\n[<code>OwnedMutexGuard</code>].</p>\n<p>This method is identical to [<code>Mutex::blocking_lock</code>], except that the returned\nguard references the <code>Mutex</code> with an <a href=\"https://doc.rust-lang.org/1.84.1/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\"><code>Arc</code></a> rather than by borrowing\nit. Therefore, the <code>Mutex</code> must be wrapped in an <code>Arc</code> to call this\nmethod, and the guard will live for the <code>'static</code> lifetime, as it keeps\nthe <code>Mutex</code> alive by holding an <code>Arc</code>.</p>\n<h5 id=\"panics-1\"><a class=\"doc-anchor\" href=\"#panics-1\">§</a>Panics</h5>\n<p>This function panics if called within an asynchronous execution context.</p>\n<ul>\n<li>If you find yourself in an asynchronous execution context and needing\nto call some (synchronous) function which performs one of these\n<code>blocking_</code> operations, then consider wrapping that call inside\n[<code>spawn_blocking()</code>][crate::runtime::Handle::spawn_blocking]\n(or [<code>block_in_place()</code>][crate::task::block_in_place]).</li>\n</ul>\n<h5 id=\"examples-4\"><a class=\"doc-anchor\" href=\"#examples-4\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>std::sync::Arc;\n<span class=\"kw\">use </span>tokio::sync::Mutex;\n\n<span class=\"attr\">#[tokio::main]\n</span><span class=\"kw\">async fn </span>main() {\n    <span class=\"kw\">let </span>mutex =  Arc::new(Mutex::new(<span class=\"number\">1</span>));\n    <span class=\"kw\">let </span>lock = mutex.lock().<span class=\"kw\">await</span>;\n\n    <span class=\"kw\">let </span>mutex1 = Arc::clone(<span class=\"kw-2\">&amp;</span>mutex);\n    <span class=\"kw\">let </span>blocking_task = tokio::task::spawn_blocking(<span class=\"kw\">move </span>|| {\n        <span class=\"comment\">// This shall block until the `lock` is released.\n        </span><span class=\"kw\">let </span><span class=\"kw-2\">mut </span>n = mutex1.blocking_lock_owned();\n        <span class=\"kw-2\">*</span>n = <span class=\"number\">2</span>;\n    });\n\n    <span class=\"macro\">assert_eq!</span>(<span class=\"kw-2\">*</span>lock, <span class=\"number\">1</span>);\n    <span class=\"comment\">// Release the lock.\n    </span>drop(lock);\n\n    <span class=\"comment\">// Await the completion of the blocking task.\n    </span>blocking_task.<span class=\"kw\">await</span>.unwrap();\n\n    <span class=\"comment\">// Assert uncontended.\n    </span><span class=\"kw\">let </span>n = mutex.try_lock().unwrap();\n    <span class=\"macro\">assert_eq!</span>(<span class=\"kw-2\">*</span>n, <span class=\"number\">2</span>);\n}\n</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.lock_owned\" class=\"method\"><h4 class=\"code-header\">pub async fn <a class=\"fn\">lock_owned</a>(self: <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;Mutex&lt;T&gt;&gt;) -&gt; OwnedMutexGuard&lt;T&gt;</h4></section></summary><div class=\"docblock\"><p>Locks this mutex, causing the current task to yield until the lock has\nbeen acquired. When the lock has been acquired, this returns an\n[<code>OwnedMutexGuard</code>].</p>\n<p>If the mutex is available to be acquired immediately, then this call\nwill typically not yield to the runtime. However, this is not guaranteed\nunder all circumstances.</p>\n<p>This method is identical to [<code>Mutex::lock</code>], except that the returned\nguard references the <code>Mutex</code> with an <a href=\"https://doc.rust-lang.org/1.84.1/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\"><code>Arc</code></a> rather than by borrowing\nit. Therefore, the <code>Mutex</code> must be wrapped in an <code>Arc</code> to call this\nmethod, and the guard will live for the <code>'static</code> lifetime, as it keeps\nthe <code>Mutex</code> alive by holding an <code>Arc</code>.</p>\n<h5 id=\"cancel-safety-1\"><a class=\"doc-anchor\" href=\"#cancel-safety-1\">§</a>Cancel safety</h5>\n<p>This method uses a queue to fairly distribute locks in the order they\nwere requested. Cancelling a call to <code>lock_owned</code> makes you lose your\nplace in the queue.</p>\n<h5 id=\"examples-5\"><a class=\"doc-anchor\" href=\"#examples-5\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>tokio::sync::Mutex;\n<span class=\"kw\">use </span>std::sync::Arc;\n\n<span class=\"attr\">#[tokio::main]\n</span><span class=\"kw\">async fn </span>main() {\n    <span class=\"kw\">let </span>mutex = Arc::new(Mutex::new(<span class=\"number\">1</span>));\n\n    <span class=\"kw\">let </span><span class=\"kw-2\">mut </span>n = mutex.clone().lock_owned().<span class=\"kw\">await</span>;\n    <span class=\"kw-2\">*</span>n = <span class=\"number\">2</span>;\n}</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_lock\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">try_lock</a>(&amp;self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;MutexGuard&lt;'_, T&gt;, TryLockError&gt;</h4></section></summary><div class=\"docblock\"><p>Attempts to acquire the lock, and returns <a href=\"TryLockError\"><code>TryLockError</code></a> if the\nlock is currently held somewhere else.</p>\n<h5 id=\"examples-6\"><a class=\"doc-anchor\" href=\"#examples-6\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>tokio::sync::Mutex;\n\n<span class=\"kw\">let </span>mutex = Mutex::new(<span class=\"number\">1</span>);\n\n<span class=\"kw\">let </span>n = mutex.try_lock()<span class=\"question-mark\">?</span>;\n<span class=\"macro\">assert_eq!</span>(<span class=\"kw-2\">*</span>n, <span class=\"number\">1</span>);</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.get_mut\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">get_mut</a>(&amp;mut self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.84.1/std/primitive.reference.html\">&amp;mut T</a></h4></section></summary><div class=\"docblock\"><p>Returns a mutable reference to the underlying data.</p>\n<p>Since this call borrows the <code>Mutex</code> mutably, no actual locking needs to\ntake place – the mutable borrow statically guarantees no locks exist.</p>\n<h5 id=\"examples-7\"><a class=\"doc-anchor\" href=\"#examples-7\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>tokio::sync::Mutex;\n\n<span class=\"kw\">fn </span>main() {\n    <span class=\"kw\">let </span><span class=\"kw-2\">mut </span>mutex = Mutex::new(<span class=\"number\">1</span>);\n\n    <span class=\"kw\">let </span>n = mutex.get_mut();\n    <span class=\"kw-2\">*</span>n = <span class=\"number\">2</span>;\n}</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_lock_owned\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">try_lock_owned</a>(\n    self: <a class=\"struct\" href=\"https://doc.rust-lang.org/1.84.1/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\">Arc</a>&lt;Mutex&lt;T&gt;&gt;,\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.84.1/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;OwnedMutexGuard&lt;T&gt;, TryLockError&gt;</h4></section></summary><div class=\"docblock\"><p>Attempts to acquire the lock, and returns <a href=\"TryLockError\"><code>TryLockError</code></a> if the lock\nis currently held somewhere else.</p>\n<p>This method is identical to [<code>Mutex::try_lock</code>], except that the\nreturned  guard references the <code>Mutex</code> with an <a href=\"https://doc.rust-lang.org/1.84.1/alloc/sync/struct.Arc.html\" title=\"struct alloc::sync::Arc\"><code>Arc</code></a> rather than by\nborrowing it. Therefore, the <code>Mutex</code> must be wrapped in an <code>Arc</code> to call\nthis method, and the guard will live for the <code>'static</code> lifetime, as it\nkeeps the <code>Mutex</code> alive by holding an <code>Arc</code>.</p>\n<h5 id=\"examples-8\"><a class=\"doc-anchor\" href=\"#examples-8\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>tokio::sync::Mutex;\n<span class=\"kw\">use </span>std::sync::Arc;\n\n<span class=\"kw\">let </span>mutex = Arc::new(Mutex::new(<span class=\"number\">1</span>));\n\n<span class=\"kw\">let </span>n = mutex.clone().try_lock_owned()<span class=\"question-mark\">?</span>;\n<span class=\"macro\">assert_eq!</span>(<span class=\"kw-2\">*</span>n, <span class=\"number\">1</span>);</code></pre></div>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.into_inner\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">into_inner</a>(self) -&gt; T</h4></section></summary><div class=\"docblock\"><p>Consumes the mutex, returning the underlying data.</p>\n<h5 id=\"examples-9\"><a class=\"doc-anchor\" href=\"#examples-9\">§</a>Examples</h5>\n<div class=\"example-wrap\"><pre class=\"rust rust-example-rendered\"><code><span class=\"kw\">use </span>tokio::sync::Mutex;\n\n<span class=\"attr\">#[tokio::main]\n</span><span class=\"kw\">async fn </span>main() {\n    <span class=\"kw\">let </span>mutex = Mutex::new(<span class=\"number\">1</span>);\n\n    <span class=\"kw\">let </span>n = mutex.into_inner();\n    <span class=\"macro\">assert_eq!</span>(n, <span class=\"number\">1</span>);\n}</code></pre></div>\n</div></details></div></details>",0,"hoover3_database::db_management::nebula::NebulaDatabaseHandle"],["<section id=\"impl-Send-for-Mutex%3CT%3E\" class=\"impl\"><a href=\"#impl-Send-for-Mutex%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for Mutex&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h3></section>","Send","hoover3_database::db_management::nebula::NebulaDatabaseHandle"],["<section id=\"impl-Sync-for-Mutex%3CT%3E\" class=\"impl\"><a href=\"#impl-Sync-for-Mutex%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for Mutex&lt;T&gt;<div class=\"where\">where\n    T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + ?<a class=\"trait\" href=\"https://doc.rust-lang.org/1.84.1/core/marker/trait.Sized.html\" title=\"trait core::marker::Sized\">Sized</a>,</div></h3></section>","Sync","hoover3_database::db_management::nebula::NebulaDatabaseHandle"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[22504]}