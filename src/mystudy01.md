further read:

[part 4](https://crates.io/crates/tcp-relay-rust)

[part 3](https://github.com/icodesign/proxy-relay/blob/tokio0.2/src/lib.rs)

[further read part 2](https://tokio.rs/tokio/tutorial/io)

[further read part 1](https://v0-1--tokio.netlify.app/docs/io/reading_writing_data/)


Error Message:

thread 'tokio-runtime-worker' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }'

Solution:

can't perform infinite loop on tranfer <readStream,writeStream>, fix it and using: **tokio::select!**

