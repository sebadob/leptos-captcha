// Copyright 2023 Sebastian Dobe <sebastiandobe@mailbox.org>

#![doc = include_str!("../README.md")]

use core::future::Future;
use leptos::*;

// re-export the Pow for ease of use
pub use spow;

pub fn pow_dispatch<C, F, Fut>(get_pow: F, is_pending: RwSignal<Option<bool>>, callback: C)
where
    C: FnOnce(Result<String, ServerFnError>) + 'static,
    F: FnOnce() -> Fut + 'static,
    Fut: Future<Output = Result<String, ServerFnError>>,
{
    is_pending.set(Some(true));
    spawn_local(async move {
        match get_pow().await {
            Ok(challenge) => {
                #[cfg(target_arch = "wasm32")]
                let work = spow::wasm::pow_work(&challenge).unwrap();
                #[cfg(not(target_arch = "wasm32"))]
                let work = spow::pow::Pow::work(&challenge).unwrap();
                is_pending.set(Some(false));
                callback(Ok(work));
            }
            Err(err) => callback(Err(err)),
        }
    });
}

#[component]
pub fn Captcha(
    is_pending: RwSignal<Option<bool>>,
    #[prop(default = "Not a Robot")] text: &'static str,
    #[prop(default = "Verifying")] text_pending: &'static str,
    #[prop(default = "Verified")] text_verified: &'static str,
) -> impl IntoView {
    let data_state = move || match is_pending.get() {
        None => "",
        Some(true) => "pending",
        Some(false) => "verified",
    };

    view! {
        <div class="leptos-captcha" data-state=data_state>
            <label>
                <input type="hidden" name="pow" value="" />
                {move || match is_pending.get() {
                    None => view! {
                        <div class="icon-front">
                            <ShieldExclamation />
                        </div>
                        <div class="text">
                            {text}
                        </div>
                    },
                    Some(true) => view! {
                        <div class="icon-front">
                            <ShieldExclamation />
                        </div>
                        <div class="text pending">
                            {text_pending}
                        </div>
                        <div class="spinner"><div></div><div></div><div></div><div></div></div>
                    },
                    Some(false) => view! {
                        <div class="icon-front">
                            <ShieldCheck />
                        </div>
                        <div class="text verified">
                            {text_verified}
                        </div>
                        <div class="icon-back">
                            <IconCheck />
                        </div>
                    },
                }}
            </label>
        </div>
    }
}

#[component]
fn ShieldExclamation() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="currentColor"
            class="w-6 h-6"
        >
            <path
                fill-rule="evenodd"
                d="M11.484 2.17a.75.75 0 0 1 1.032 0 11.209 11.209 0 0 0 7.877 3.08.75.75 0 0 \
                1 .722.515 12.74 12.74 0 0 1 .635 3.985c0 5.942-4.064 10.933-9.563 12.348a.749.749 0 \
                0 1-.374 0C6.314 20.683 2.25 15.692 2.25 9.75c0-1.39.223-2.73.635-3.985a.75.75 0 0 \
                1 .722-.516l.143.001c2.996 0 5.718-1.17 7.734-3.08ZM12 8.25a.75.75 0 0 1 \
                .75.75v3.75a.75.75 0 0 1-1.5 0V9a.75.75 0 0 1 .75-.75ZM12 15a.75.75 0 0 0-.75.75v.008c0 \
                .414.336.75.75.75h.008a.75.75 0 0 0 .75-.75v-.008a.75.75 0 0 0-.75-.75H12Z"
                clip-rule="evenodd"
            />
        </svg>
    }
}

#[component]
fn ShieldCheck() -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 24 24"
            fill="currentColor"
            class="w-6 h-6"
        >
            <path
                fill-rule="evenodd"
                d="M12.516 2.17a.75.75 0 0 0-1.032 0 11.209 11.209 0 0 1-7.877 3.08.75.75 0 0 \
                0-.722.515A12.74 12.74 0 0 0 2.25 9.75c0 5.942 4.064 10.933 9.563 12.348a.749.749 \
                0 0 0 .374 0c5.499-1.415 9.563-6.406 9.563-12.348 0-1.39-.223-2.73-.635-3.985a.75.75 \
                0 0 0-.722-.516l-.143.001c-2.996 0-5.717-1.17-7.734-3.08Zm3.094 8.016a.75.75 0 1 \
                0-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 0 0-1.06 1.06l2.25 2.25a.75.75 0 0 0 \
                1.14-.094l3.75-5.25Z"
                clip-rule="evenodd"
            />
        </svg>
    }
}

#[component]
pub fn IconCheck() -> impl IntoView {
    view! {
        <svg
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
            stroke-width=2
        >
            <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12.75l6 6 9-13.5" />
        </svg>
    }
}
