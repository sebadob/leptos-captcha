# leptos-captcha

This is a simple Captcha component. Everything runs on the same backend without any external dependencies.  
The crate for the Proof of Work (PoW) being used is [spow](https://github.com/sebadob/spow), which is another project of
mine. If you want to know more about the algorithm, please take a look at the repository.  
No user input, solving weird puzzles, or anything like that is needed. This provides a way better UX than the
traditional captchas out there.

This component is designed to be injected into a `<form>` for easy usage with leptos server_fn's, but you
can of course use it independently and build a custom solution, if you like. In that case you might consider to use
[spow](https://github.com/sebadob/spow) directly.

No inline-css is being used on purpose for 2 reasons:
- you are not forced to use an `unsafe-inline` CSP for `style-src`, if you don't like that
- you can customize the whole look yourself
For these reasons, you need to import the `css/leptos-captcha.css` in your application in what ever way fits your needs,
or simply copy & paste it into your current `main.scss`.

There is an [example](https://github.com/sebadob/leptos-captcha/tree/main/examples/axum-example) based on the official
`start-axum` template. Just take a look at it, and it should (hopefully) be clear, how to use this component.

At this moment, it runs thread local. This means if you have a too high difficulty, it will block the UI.
An improvement with outsourcing it to a web worker will probably come at some point, even though you should not really
need it in production.