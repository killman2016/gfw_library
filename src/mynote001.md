further read:

[part 4](https://crates.io/crates/tcp-relay-rust)

[part 3](https://github.com/icodesign/proxy-relay/blob/tokio0.2/src/lib.rs)

[pls read this part 2](https://tokio.rs/tokio/tutorial/io)

[pls read this part 1](https://v0-1--tokio.netlify.app/docs/io/reading_writing_data/)


decrypt to: <912>: [23, 3, 3, 3, 139, 106, 226, 229] ... [52, 119, 188, 128, 144, 121, 241, 76]
thread 'tokio-runtime-worker' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', /home/a14248/project/rust/gfw.press.rust/gfw_library/src/gfw_proxy.rs:180:47
stack backtrace:
   0:     0x560f5e8d91da - std::backtrace_rs::backtrace::libunwind::trace::h9a6b80bbf328ba5d
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x560f5e8d91da - std::backtrace_rs::backtrace::trace_unsynchronized::hd162ec543a11886b
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x560f5e8d91da - std::sys_common::backtrace::_print_fmt::h78a5099be12f51a6
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x560f5e8d91da - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha1c5390454d74f71
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x560f5e8f741f - core::fmt::write::h9ffde816c577717b
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/fmt/mod.rs:1254:17
   5:     0x560f5e8d6485 - std::io::Write::write_fmt::h88186074961638e4
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/io/mod.rs:1698:15
   6:     0x560f5e8d8fa5 - std::sys_common::backtrace::_print::h184198273ed08d59
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x560f5e8d8fa5 - std::sys_common::backtrace::print::h1b4d8e7add699453
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x560f5e8da64e - std::panicking::default_hook::{{closure}}::h393bcea75423915a
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:269:22
   9:     0x560f5e8da3f5 - std::panicking::default_hook::h48c64f31d8b3fd03
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:288:9
  10:     0x560f5e8dabae - std::panicking::rust_panic_with_hook::hafdc493a79370062
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:691:13
  11:     0x560f5e8daaa9 - std::panicking::begin_panic_handler::{{closure}}::h0a64bc82e36bedc7
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:582:13
  12:     0x560f5e8d9646 - std::sys_common::backtrace::__rust_end_short_backtrace::hc203444fb7416a16
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/sys_common/backtrace.rs:150:18
  13:     0x560f5e8da802 - rust_begin_unwind
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:578:5
  14:     0x560f5e7bd5d3 - core::panicking::panic_fmt::h0f6ef0178afce4f2
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/panicking.rs:67:14
  15:     0x560f5e7bdab3 - core::result::unwrap_failed::h8090202169109f9c
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/result.rs:1687:5
  16:     0x560f5e7da14d - core::result::Result<T,E>::unwrap::ha2b22adfda4650c0
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/result.rs:1089:23
  17:     0x560f5e7d1b70 - gfw_library::gfw_proxy::transfer_decrypt::{{closure}}::h014751c11c5a6a61
                               at /home/a14248/project/rust/gfw.press.rust/gfw_library/src/gfw_proxy.rs:180:17
  18:     0x560f5e7d499a - <F as futures_core::future::TryFuture>::try_poll::h8af813bbce42cae9
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/futures-core-0.3.28/src/future.rs:82:9
  19:     0x560f5e7c2a83 - <futures_util::future::try_maybe_done::TryMaybeDone<Fut> as core::future::future::Future>::poll::hfdbbeb82b60ab8a1
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/futures-util-0.3.28/src/future/try_maybe_done.rs:79:57
  20:     0x560f5e7c5a40 - <futures_util::future::try_join::TryJoin<Fut1,Fut2> as core::future::future::Future>::poll::hc383f5bc6e5c540d
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/futures-util-0.3.28/src/future/try_join.rs:72:29
  21:     0x560f5e7d3e5e - gfw_library::gfw_proxy::gfw_relay::{{closure}}::h53fa9d11d11c6e32
                               at /home/a14248/project/rust/gfw.press.rust/gfw_library/src/gfw_proxy.rs:88:17
  22:     0x560f5e7d2ca8 - gfw_library::gfw_proxy::handle_connection::{{closure}}::h9809023e34f910ec
                               at /home/a14248/project/rust/gfw.press.rust/gfw_library/src/gfw_proxy.rs:51:58
  23:     0x560f5e7d0a09 - gfw_library::gfw_proxy::gfw_press_proxy::{{closure}}::{{closure}}::haeaa4418d845930c
                               at /home/a14248/project/rust/gfw.press.rust/gfw_library/src/gfw_proxy.rs:37:82
  24:     0x560f5e7ce5b7 - tokio::runtime::task::core::Core<T,S>::poll::{{closure}}::h7ee202b77ef5a833
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/task/core.rs:311:17
  25:     0x560f5e7dbc5f - tokio::loom::std::unsafe_cell::UnsafeCell<T>::with_mut::h6b8d37b0f7133000
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/loom/std/unsafe_cell.rs:14:9
  26:     0x560f5e7ce0ca - tokio::runtime::task::core::Core<T,S>::poll::h21b19b8f964c50da
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/task/core.rs:300:13
  27:     0x560f5e7c8ef1 - tokio::runtime::task::harness::poll_future::{{closure}}::hf7bd42b3876bdf3c
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/task/harness.rs:476:19
  28:     0x560f5e7c7a33 - <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::hb5ab85bebddaad26
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/panic/unwind_safe.rs:271:9
  29:     0x560f5e7d89fb - std::panicking::try::do_call::ha64e05305e66c5c2
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:485:40
  30:     0x560f5e7d973b - __rust_try
  31:     0x560f5e7d8687 - std::panicking::try::hefda3830b6516c8c
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:449:19
  32:     0x560f5e7dac1a - std::panic::catch_unwind::h3a5bd7923359e0ba
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panic.rs:140:14
  33:     0x560f5e7c8676 - tokio::runtime::task::harness::poll_future::h950ce7a4611cc5a1
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/task/harness.rs:464:18
  34:     0x560f5e7c905a - tokio::runtime::task::harness::Harness<T,S>::poll_inner::h1e75ccb757ba461e
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/task/harness.rs:198:27
  35:     0x560f5e7c9e23 - tokio::runtime::task::harness::Harness<T,S>::poll::h599a995da73d3f6e
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/task/harness.rs:152:15
  36:     0x560f5e7d73fb - tokio::runtime::task::raw::poll::hbca94d03331598a0
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/task/raw.rs:276:5
  37:     0x560f5e824e08 - tokio::runtime::task::raw::RawTask::poll::h06f0e8c4eeb74d88
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/task/raw.rs:200:18
  38:     0x560f5e846212 - tokio::runtime::task::LocalNotified<S>::run::h467a3975e16b615b
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/task/mod.rs:400:9
  39:     0x560f5e81fd0d - tokio::runtime::scheduler::multi_thread::worker::Context::run_task::{{closure}}::h145c25967dd06188
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/scheduler/multi_thread/worker.rs:576:13
  40:     0x560f5e81fb88 - tokio::runtime::coop::with_budget::h37571074ad2fd40f
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/coop.rs:107:5
  41:     0x560f5e81fb88 - tokio::runtime::coop::budget::hfca7fb1dc5d5c9aa
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/coop.rs:73:5
  42:     0x560f5e81fb88 - tokio::runtime::scheduler::multi_thread::worker::Context::run_task::h565d4625c5041821
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/scheduler/multi_thread/worker.rs:575:9
  43:     0x560f5e81f045 - tokio::runtime::scheduler::multi_thread::worker::Context::run::h5c03320b89c80172
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/scheduler/multi_thread/worker.rs:526:24
  44:     0x560f5e81eb39 - tokio::runtime::scheduler::multi_thread::worker::run::{{closure}}::{{closure}}::h4acd90b395cf5c31
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/scheduler/multi_thread/worker.rs:491:21
  45:     0x560f5e83bb70 - tokio::runtime::context::scoped::Scoped<T>::set::h196ee1da5165e587
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/context/scoped.rs:40:9
  46:     0x560f5e85256b - tokio::runtime::context::set_scheduler::{{closure}}::h5ef38901115555c2
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/context.rs:176:26
  47:     0x560f5e858a3e - std::thread::local::LocalKey<T>::try_with::h8b14b121ea648fed
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/thread/local.rs:252:16
  48:     0x560f5e85741b - std::thread::local::LocalKey<T>::with::hbbafebcf9939d07a
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/thread/local.rs:228:9
  49:     0x560f5e8524a4 - tokio::runtime::context::set_scheduler::hb22c4aae1a682500
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/context.rs:176:9
  50:     0x560f5e81ea37 - tokio::runtime::scheduler::multi_thread::worker::run::{{closure}}::hf4435c1abd2241bb
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/scheduler/multi_thread/worker.rs:486:9
  51:     0x560f5e8197f8 - tokio::runtime::context::runtime::enter_runtime::h75b376e7dc430c89
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/context/runtime.rs:65:16
  52:     0x560f5e81e790 - tokio::runtime::scheduler::multi_thread::worker::run::h5e65c44af228f76d
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/scheduler/multi_thread/worker.rs:478:5
  53:     0x560f5e81e62b - tokio::runtime::scheduler::multi_thread::worker::Launch::launch::{{closure}}::h218b78964a4d94b7
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/scheduler/multi_thread/worker.rs:447:45
  54:     0x560f5e85e53e - <tokio::runtime::blocking::task::BlockingTask<T> as core::future::future::Future>::poll::hfc76c3b4cc9c4324
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/blocking/task.rs:42:21
  55:     0x560f5e82fa66 - tokio::runtime::task::core::Core<T,S>::poll::{{closure}}::h708ea7c3ed081074
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/task/core.rs:311:17
  56:     0x560f5e801d3f - tokio::loom::std::unsafe_cell::UnsafeCell<T>::with_mut::hc7a93bd4d041d69b
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/loom/std/unsafe_cell.rs:14:9
  57:     0x560f5e82f58e - tokio::runtime::task::core::Core<T,S>::poll::hc115de0bc72f38f7
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/task/core.rs:300:13
  58:     0x560f5e809575 - tokio::runtime::task::harness::poll_future::{{closure}}::h852d394a14b148f3
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/task/harness.rs:476:19
  59:     0x560f5e83f6f3 - <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::hf0333ef0874c3392
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/panic/unwind_safe.rs:271:9
  60:     0x560f5e872a0c - std::panicking::try::do_call::he3f7515b52a23cbc
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:485:40
  61:     0x560f5e8737cb - __rust_try
  62:     0x560f5e8718a7 - std::panicking::try::h41cde1abcd13ee33
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:449:19
  63:     0x560f5e85eeab - std::panic::catch_unwind::ha8d1f56992371b3c
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panic.rs:140:14
  64:     0x560f5e808e07 - tokio::runtime::task::harness::poll_future::hbad6c1eb20aa6e5f
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/task/harness.rs:464:18
  65:     0x560f5e806a99 - tokio::runtime::task::harness::Harness<T,S>::poll_inner::hbce4dedf00ea7cfe
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/task/harness.rs:198:27
  66:     0x560f5e8064a7 - tokio::runtime::task::harness::Harness<T,S>::poll::h5256c3c368099d85
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/task/harness.rs:152:15
  67:     0x560f5e82503d - tokio::runtime::task::raw::poll::h7a36f089c14cce2b
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/task/raw.rs:276:5
  68:     0x560f5e824e08 - tokio::runtime::task::raw::RawTask::poll::h06f0e8c4eeb74d88
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/task/raw.rs:200:18
  69:     0x560f5e8462d7 - tokio::runtime::task::UnownedTask<S>::run::h95b3d2b1f95d9d12
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/task/mod.rs:437:9
  70:     0x560f5e852837 - tokio::runtime::blocking::pool::Task::run::h82cbac95dd80d22f
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/blocking/pool.rs:159:9
  71:     0x560f5e8560f3 - tokio::runtime::blocking::pool::Inner::run::h2c19e91e84cacdf8
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/blocking/pool.rs:513:17
  72:     0x560f5e855634 - tokio::runtime::blocking::pool::Spawner::spawn_thread::{{closure}}::h6b09d5b914314fdf
                               at /home/a14248/.cargo/registry/src/index.crates.io-6f17d22bba15001f/tokio-1.29.1/src/runtime/blocking/pool.rs:471:13
  73:     0x560f5e7f7739 - std::sys_common::backtrace::__rust_begin_short_backtrace::h642b4a57bb47445b
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/sys_common/backtrace.rs:134:18
  74:     0x560f5e828de2 - std::thread::Builder::spawn_unchecked_::{{closure}}::{{closure}}::h02f691f63664d7ed
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/thread/mod.rs:526:17
  75:     0x560f5e83f5a2 - <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::h843b5f2118eee3ee
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/panic/unwind_safe.rs:271:9
  76:     0x560f5e87281b - std::panicking::try::do_call::h8a30f90aff1eb640
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:485:40
  77:     0x560f5e8737cb - __rust_try
  78:     0x560f5e872311 - std::panicking::try::hf579716ff05ba486
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panicking.rs:449:19
  79:     0x560f5e85eeca - std::panic::catch_unwind::hac652e1bf6fb1efe
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/panic.rs:140:14
  80:     0x560f5e828bef - std::thread::Builder::spawn_unchecked_::{{closure}}::hdedc736ab0dcf7c1
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/thread/mod.rs:525:30
  81:     0x560f5e80a04f - core::ops::function::FnOnce::call_once{{vtable.shim}}::h2375e6454c5a4d36
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/core/src/ops/function.rs:250:5
  82:     0x560f5e8dd295 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::ha1f2224656a778fb
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/alloc/src/boxed.rs:1973:9
  83:     0x560f5e8dd295 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::haa29ed9703f354b7
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/alloc/src/boxed.rs:1973:9
  84:     0x560f5e8dd295 - std::sys::unix::thread::Thread::new::thread_start::h33b6dae3e3692197
                               at /rustc/90c541806f23a127002de5b4038be731ba1458ca/library/std/src/sys/unix/thread.rs:108:17
  85:     0x7f6fe8ba8fd4 - <unknown>
  86:     0x7f6fe8c295bc - <unknown>
  87:                0x0 - <unknown>
