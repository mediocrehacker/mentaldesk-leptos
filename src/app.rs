use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

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
        <Stylesheet id="leptos" href="/pkg/mentaldesk.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=StaticSegment("/branding") view=BrandingPage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
            <button on:click=on_click>"Click Me: " {count}</button>
    }
}


#[server]
pub async fn add_todo(_title: String) -> Result<(), ServerFnError> {
    todo!()
}

#[component]
fn BrandingPage() -> impl IntoView {
    let add_todo = ServerAction::<AddTodo>::new();
    // holds the latest *returned* value from the server
    // let value = add_todo.value();
    // check if the server has returned an error
    // let has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    view! {
        <h1>Брэндировать</h1>
        <ActionForm action=add_todo>
            <label>
                "Add a Todo"
                // `title` matches the `title` argument to `add_todo`
                <input type="text" name="title"/>
            </label>
            
            <input type="submit" value="Add"/>
                        <button class="button">Default</button>
            <button class="button outlined">Outlined</button>
            <button class="button tonal">Tonal</button>
            <button class="button filled">Filled</button>
        </ActionForm>
    }
}
