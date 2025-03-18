use leptos::ev::SubmitEvent;
use leptos::prelude::*;
use leptos_captcha::Captcha;
use leptos_meta::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/axum-example.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        <FormExample />
    }
}

// You need to provide at least one server function, that creates and returns a PoW challenge
#[server]
pub async fn get_pow() -> Result<String, ServerFnError> {
    use leptos_captcha::spow::pow::Pow;

    // I highly suggest, that you create a global static variable in your app
    // as an indicator if you are in DEV / DEBUG mode, or something like that.
    // You could pull it from the context, or where ever it makes sense for you.
    // In debug mode, the speed of the verification in the UI is a lot slower, and
    // you should just use the lowest difficulty of `10` during development.
    const DEV_MODE: bool = true;

    if DEV_MODE {
        Ok(Pow::with_difficulty(10, 10)?.to_string())
    } else {
        Ok(Pow::new(10)?.to_string())
    }
}

#[server]
pub async fn post_form(pow: String, name: String) -> Result<String, ServerFnError> {
    use leptos_captcha::spow::pow::Pow;
    use leptos::logging::log;

    Pow::validate(&pow)?;
    log!("pow {} is valid on the server", pow);

    Ok(format!("Hello {} - your request was valid", name))
}

#[component]
fn FormExample() -> impl IntoView {
    let action_save: ServerAction<PostForm> = ServerAction::new();
    let is_pending = RwSignal::new(None);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        if let Ok(mut data) = PostForm::from_event(&ev) {
            // The Captcha validation is running thread local.
            // This means a too high difficulty will block the thread.
            // The default of 20 is reasonable for a release build, but
            // way too high for development.
            //
            // The validation might be improved in the future by using
            // a web worker for this purpose, but this is not yet implemented.
            leptos_captcha::pow_dispatch(get_pow, is_pending, move |pow| {
                data.pow = pow.unwrap();
                action_save.dispatch(data);
            })
        }
    };

    view! {
        <form on:submit=on_submit>
            <input name="name" placeholder="Your Name" />

            // The Captcha component adds a hidden input with the name `pow` (Proof of Work) to
            // the form. This is empty in the beginning to make the `PostForm::from_event(&ev)`
            // above work without any issues.
            <Captcha is_pending />

            <button type="submit">"Submit"</button>
            <p>
                {move || action_save.value()
                    .get()
                    .map(|r| r.unwrap_or_default())
                }
            </p>
        </form>
    }
}
