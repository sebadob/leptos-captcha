# axum-example

This example shows how simple it is to use the `leptos-captcha` crate.

This example is based on the official [start-axum](https://github.com/leptos-rs/start-axum) example.  
Differences to the original template:
- add `leptos-captcha` and `serde` to `Cargo.toml`
- copy the CSS content from `leptos-captcha/css/leptos-captcha.css` into `style/main.scss`
- modify `src/app.rs` to show how to use the crate
- add `leptos_captcha::spow::pow::Pow::init_random().unwrap();` to `src/main.rs`
Any `Pow::init()` variant must be called once at app startup. This will initialize a secret behind the scenes,
which will be used to generate and verify Proof of Work's (PoW).
