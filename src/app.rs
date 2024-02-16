use crate::error_template::{AppError, ErrorTemplate};
use gloo_storage::Storage;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    logging::log!("Start...");

    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    create_effect(move |_| {
        let storage = gloo_storage::LocalStorage::raw();
        logging::log!("{storage:?}");
    });

    view! {
        <Title text="Leptos SSR Demo"/>
        <Stylesheet id="tailwindcss" href="/pkg/tailwind.css"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main class="container flex-col m-4">
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <div class="flex m-2 items-center flex-col">
            <h1>"Welcome to Leptos!"</h1>
            <button class="border-2 text-gray-500" on:click=on_click>"Click Me: " {count}</button>
        </div>
    }
}
