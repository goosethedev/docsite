use dioxus::prelude::*;
use dioxus_playground::PlaygroundUrls;

#[cfg(not(feature = "production"))]
const URLS: PlaygroundUrls = PlaygroundUrls {
    socket: "ws://localhost:3000/ws",
    server: "http://localhost:3000",
    location: "http://localhost:8080/playground",
};

#[cfg(feature = "production")]
const URLS: PlaygroundUrls = PlaygroundUrls {
    socket: "wss://docsite-playground.fly.dev/ws",
    server: "https://docsite-playground.fly.dev",
    location: "https://dioxuslabs.com/playground",
};

#[component]
pub fn Playground(share_code: Option<String>) -> Element {
    // Only render playground on client.
    let mut on_client = use_signal(|| false);
    use_effect(move || on_client.set(true));

    if on_client() {
        rsx! {
            ErrorBoundary {
                handle_error: move |err: ErrorContext| {
                    let errors = err.errors();

                    rsx! {
                        div {
                            class: "mx-auto mt-8 max-w-3/4",

                            h4 {
                                class: "dark:text-white font-light text-ghdarkmetal",
                                "The Dioxus Playground encountered an error."
                            }

                            br {}

                            for error in errors {
                                p {
                                    class: "dark:text-white font-light text-ghdarkmetal",
                                    "{error:?}"
                                }
                                br {}
                            }
                        }
                    }
                },

                dioxus_playground::Playground {
                    class: "playground-container max-w-screen-2xl mx-auto mt-8",
                    urls: URLS,
                    share_code,
                }
            }
        }
    } else {
        rsx! {}
    }
}

#[component]
pub fn SharePlayground(share_code: String) -> Element {
    rsx! {
        Playground { share_code }
    }
}
