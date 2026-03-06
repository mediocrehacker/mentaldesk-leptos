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
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
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
        <Stylesheet id="leptos" href="/pkg/mentaldesk.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router>
            <div id="app">
                <nav>HEADER</nav>
                <main class="wrapper">
                    <Routes fallback=|| "Page not found.".into_view()>
                        <Route path=StaticSegment("") view=HomePage />
                        <Route path=StaticSegment("/branding") view=BrandingPage />
                    </Routes>
                </main>
                <footer>FOOTER</footer>
            </div>
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
        <header>
            <h2>Брэндировать</h2>
            <div>
                <p>Заголовок рабочего листа</p>
            </div>
        </header>

        <div class="branding">
            <div class="branding__form">
                <ActionForm action=add_todo>
                    <div>
                        <label class="field">
                            <span class="label">ФИО Клиента</span>
                            <input type="text" id="client_name" client_name="title" />
                        </label>
                    </div>
                    <div>
                        <label class="field">
                            <span class="label">
                                Форма обращение к клиенту
                            </span>
                            <input
                                type="text"
                                id="client_title"
                                placeholder="Клиент:"
                                client_title="title"
                            />
                        </label>
                    </div>
                    <div>
                        <input
                            class="button tonal"
                            type="submit"
                            value="Сгенерировать"
                        />
                    </div>
                </ActionForm>
            </div>
            <div class="branding__preview">
                <div class="card elevated">
                    <img src="worksheet.png" />
                </div>
            </div>
        </div>
    }
}
