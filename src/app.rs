use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <StaticRoute
                        path="/"
                        view=HomePage
                        static_params=move || Box::pin(async move { StaticParamsMap::default() })
                    />
                    <StaticRoute
                        path="/test/hallo"
                        view=HomePage
                        static_params=move || Box::pin(async move { StaticParamsMap::default() })
                    />
                    <StaticRoute
                        path="404"
                        view=NotFound
                        static_params=move || Box::pin(async move { StaticParamsMap::default() })
                    />
                    <StaticRoute
                        path="/*any"
                        view=HomePage
                        static_params=move || Box::pin(async move {
                            let mut map = StaticParamsMap::default();
                            map.insert(
                                "any".to_string(),
                                vec!["steve".to_string(), "vali".to_string()],
                            );
                            map
                        })
                    />

                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <Counter/>
    }
}

#[island]
fn Counter() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    view! { <h1>"Not Found from Leptos"</h1> }
}
