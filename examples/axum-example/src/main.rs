#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use axum_example::app::*;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tokio::net::TcpListener;

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // This (or any other `init()` vairant must be called once at startup
    leptos_captcha::spow::pow::Pow::init_random().unwrap();

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        // .fallback(file_and_error_handler)
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
