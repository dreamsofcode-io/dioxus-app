use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Layout)]
        #[route("/")]
        Home {},
        #[route("/about")]
        About {},
        #[route("/suspense")]
        SuspenseDemo {},
        #[route("/visits")]
        Visits {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Title { "Dioxus App" }
        Router::<Route> {}
    }
}

#[component]
fn Layout() -> Element {
    rsx! {
        main { class: "min-h-screen flex justify-center px-5 pt-12 pb-8 sm:px-8 sm:pt-16 bg-zinc-950 text-zinc-100",
            div { class: "w-full max-w-5xl grid gap-7 content-start",
                header { class: "grid gap-5",
                    div { class: "max-w-2xl",
                        span { class: "inline-block mb-3 text-xs font-bold tracking-widest uppercase text-zinc-400", "Dioxus Playground" }
                        h1 { class: "text-5xl sm:text-7xl font-bold tracking-tighter leading-none text-white", "Explore" }
                        p { class: "mt-3 max-w-xl text-zinc-400 leading-relaxed", "A full-stack Rust web app built with Dioxus." }
                    }
                    nav { class: "flex flex-wrap gap-3",
                        "aria-label": "Primary",
                        Link { class: "inline-flex items-center justify-center min-w-[110px] rounded-full border border-white/20 bg-zinc-900/70 px-5 py-3 font-semibold text-white no-underline hover:bg-zinc-800 hover:-translate-y-0.5 transition-all", to: Route::Home {}, "Counter" }
                        Link { class: "inline-flex items-center justify-center min-w-[110px] rounded-full border border-white/20 bg-zinc-900/70 px-5 py-3 font-semibold text-white no-underline hover:bg-zinc-800 hover:-translate-y-0.5 transition-all", to: Route::About {}, "Router" }
                        Link { class: "inline-flex items-center justify-center min-w-[110px] rounded-full border border-white/20 bg-zinc-900/70 px-5 py-3 font-semibold text-white no-underline hover:bg-zinc-800 hover:-translate-y-0.5 transition-all", to: Route::SuspenseDemo {}, "Suspense" }
                        Link { class: "inline-flex items-center justify-center min-w-[110px] rounded-full border border-white/20 bg-zinc-900/70 px-5 py-3 font-semibold text-white no-underline hover:bg-zinc-800 hover:-translate-y-0.5 transition-all", to: Route::Visits {}, "Visits" }
                    }
                }
                section { class: "flex justify-center",
                    Outlet::<Route> {}
                }
            }
        }
    }
}

/// Counter page
#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0_i32);
    let count_color = use_memo(move || {
        if count() > 0 {
            "text-green-400"
        } else if count() < 0 {
            "text-red-400"
        } else {
            "text-zinc-300"
        }
    });

    rsx! {
        section { class: "grid gap-8 lg:grid-cols-2 lg:items-center",
            div { class: "max-w-lg",
                span { class: "inline-block mb-3 text-xs font-bold tracking-widest uppercase text-zinc-400", "Dioxus Demo" }
                h2 { class: "text-4xl font-bold tracking-tight text-white", "Counter" }
                p { class: "mt-3 text-zinc-400 leading-relaxed", "A simple client-side state example in Rust." }
            }
            div { class: "rounded-3xl border border-white/10 bg-white/[0.04] backdrop-blur-lg p-6 shadow-2xl",
                div { class: "rounded-2xl border border-white/[0.08] bg-zinc-950/70 p-5",
                    p { class: "text-center text-7xl font-bold tracking-tighter mb-5 transition-colors {count_color}",
                        "{count}"
                    }
                    div { class: "grid grid-cols-2 gap-3",
                        button {
                            class: "rounded-xl border border-white/20 ring-1 ring-white/[0.06] bg-zinc-800 px-4 py-3 font-semibold text-white hover:bg-zinc-700 hover:-translate-y-0.5 transition-all cursor-pointer",
                            onclick: move |_| count -= 1,
                            "Decrease"
                        }
                        button {
                            class: "rounded-xl border border-zinc-300 ring-1 ring-white/20 bg-white px-4 py-3 font-semibold text-zinc-900 hover:bg-zinc-200 hover:-translate-y-0.5 transition-all cursor-pointer",
                            onclick: move |_| count += 1,
                            "Increase"
                        }
                    }
                }
            }
        }
    }
}

/// About page
#[component]
fn About() -> Element {
    rsx! {
        section { class: "grid gap-8 lg:grid-cols-2 lg:items-center",
            div { class: "max-w-lg",
                span { class: "inline-block mb-3 text-xs font-bold tracking-widest uppercase text-zinc-400", "Dioxus Router" }
                h2 { class: "text-4xl font-bold tracking-tight text-white", "About" }
                p { class: "mt-3 text-zinc-400 leading-relaxed", "This route is handled by the Dioxus router, so the page content swaps without a full document navigation." }
            }
            div { class: "rounded-3xl border border-white/10 bg-white/[0.04] backdrop-blur-lg p-8 shadow-2xl",
                p { class: "text-zinc-300 leading-relaxed", "The layout is shared across routes. Each page renders its own content inside the same shell, demonstrating client-side routing in a full-stack Rust app with server-side rendering and hydration." }
            }
        }
    }
}

/// Server function that returns a message after a delay.
#[post("/api/delayed-message")]
async fn get_delayed_message() -> Result<String, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
    Ok(String::from("Loaded after a short async delay."))
}

/// Suspense demo page
#[component]
fn SuspenseDemo() -> Element {
    let data = use_resource(move || async move { get_delayed_message().await });

    match data() {
        Some(Ok(message)) => rsx! {
            div { class: "min-h-[40vh] flex items-center justify-center",
                div { class: "w-full max-w-md rounded-3xl border border-white/20 bg-zinc-800/60 backdrop-blur-lg p-8 shadow-2xl shadow-black/40 ring-1 ring-white/[0.06] text-center",
                    span { class: "inline-block mb-4 rounded-full bg-green-500/20 px-4 py-1.5 text-xs font-bold tracking-widest uppercase text-green-400", "Resolved" }
                    p { class: "text-zinc-300 leading-relaxed", "{message}" }
                }
            }
        },
        Some(Err(e)) => rsx! {
            p { class: "text-red-400", "Error: {e}" }
        },
        None => rsx! {
            div { class: "min-h-[40vh] flex items-center justify-center",
                div { class: "w-full max-w-md rounded-3xl border border-white/20 bg-zinc-800/60 backdrop-blur-lg p-8 shadow-2xl shadow-black/40 ring-1 ring-white/[0.06] text-center",
                    span { class: "inline-block mb-4 rounded-full bg-amber-500/20 px-4 py-1.5 text-xs font-bold tracking-widest uppercase text-amber-400", "Fetching" }
                    p { class: "text-zinc-400 leading-relaxed mb-5", "Waiting for async data to resolve..." }
                    div { class: "flex items-end justify-center gap-1.5 h-8",
                        "aria-hidden": "true",
                        span { class: "loading-bar w-1.5 h-full rounded-full bg-zinc-500" }
                        span { class: "loading-bar w-1.5 h-full rounded-full bg-zinc-500" }
                        span { class: "loading-bar w-1.5 h-full rounded-full bg-zinc-500" }
                    }
                }
            }
        },
    }
}

/// Server-side page visit counter.
#[cfg(feature = "server")]
static VISIT_COUNT: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

#[post("/api/visit-count")]
async fn get_visit_count() -> Result<u64, ServerFnError> {
    let count = VISIT_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
    Ok(count)
}

/// Visits page
#[component]
fn Visits() -> Element {
    let visits = use_resource(move || async move { get_visit_count().await });

    rsx! {
        section { class: "grid gap-8 lg:grid-cols-2 lg:items-center",
            div { class: "max-w-lg",
                span { class: "inline-block mb-3 text-xs font-bold tracking-widest uppercase text-zinc-400", "Server Function" }
                h2 { class: "text-4xl font-bold tracking-tight text-white", "Page Visits" }
                p { class: "mt-3 text-zinc-400 leading-relaxed", "This counter lives on the server. Each load calls a server function that increments and returns the visit count." }
            }
            div { class: "rounded-3xl border border-white/10 bg-white/[0.04] backdrop-blur-lg p-6 shadow-2xl",
                div { class: "rounded-2xl border border-white/[0.08] bg-zinc-950/70 p-5",
                    {match visits() {
                        Some(Ok(count)) => rsx! {
                            p { class: "text-center text-7xl font-bold tracking-tighter text-green-400 mb-5", "{count}" }
                        },
                        Some(Err(e)) => rsx! {
                            p { class: "text-center text-red-400", "Error: {e}" }
                        },
                        None => rsx! {
                            p { class: "text-center text-7xl font-bold tracking-tighter text-zinc-500 mb-5", "..." }
                        },
                    }}
                    p { class: "text-center text-zinc-400 text-sm", "Total visits (server-side counter)" }
                }
            }
        }
    }
}
