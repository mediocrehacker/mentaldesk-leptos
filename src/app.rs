use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use miette::Result;
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Serialize, Debug)]
#[allow(unused)]
struct CreateWorksheet {
    template_path: String,
    client_title: String,
    client_name: String,
    practice_name: String,
    therapist_title: String,
    therapist_name: String,
}


#[server]
pub async fn branding(client_name: String, client_title: String, therapist_name: String, therapist_title: String, practice_name: String) -> Result<Vec<u8>, ServerFnError> {
    use sha2::{Digest, Sha256};

    let client_title = if client_title.trim().is_empty() {
        "Клиент:".to_string()
    } else {
        client_title
    };

    let payload = CreateWorksheet {
        template_path: "assets/worksheets/dnevnik-goryachih-tochek".to_string(),
        client_title: client_title.to_string(),
        client_name: client_name.clone(),
        practice_name: practice_name.clone(),
        therapist_title: therapist_title.clone(),
        therapist_name: therapist_name.clone(),
    };
    let template_path = format!("{}/worksheet.tex", payload.template_path);
    let content = tokio::fs::read_to_string(&template_path).await?;
    
    let latex = branding_template(content, &payload)?;

    let hash = Sha256::digest(&latex);
    let filename = format!("public/cache/{:x}.pdf", hash);

    // let data = tokio::fs::read(&pdf_path).await;
    let data = compile_latex(&filename, &latex).await?;

    Ok(data)
}

fn pdf_bytes_to_url(bytes: &[u8]) -> String {
    use web_sys::{Blob, BlobPropertyBag, Url};

    let array = js_sys::Uint8Array::from(bytes);
    let parts = js_sys::Array::new();
    parts.push(&array.buffer());

    let bag = BlobPropertyBag::new();
    bag.set_type("application/pdf");

    let blob = Blob::new_with_u8_array_sequence_and_options(&parts, &bag).unwrap();
    Url::create_object_url_with_blob(&blob).unwrap()
}
#[component]
fn BrandingPage() -> impl IntoView {
    let branding = ServerAction::<Branding>::new();

    Effect::new({
        let branding = branding.clone();
        move |_| {
            branding.dispatch(Branding {
                client_name: "Кириллов Егор Маркович".to_string(),
                client_title: "Клиент:".to_string(),
                practice_name: "MetnalDesk".to_string(),
                therapist_title: "Психотерапевт:".to_string(),
                therapist_name: "Зубков Тимур Владимирович".to_string(),
            });
        }
    });
    
    // holds the latest *returned* value from the server
    let value = branding.value();
    // check if the server has returned an error
    let _has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    view! {
        <header>
            <h2>Брэндировать</h2>
            <div>
                <p>Заголовок рабочего листа</p>
            </div>
        </header>

        <div class="branding">
            <div class="branding__form">
                <ActionForm action=branding>
                    <div>
                        <label class="field">
                            <span class="label">ФИО Клиента</span>
                            <input type="text" name="client_name" />
                        </label>
                    </div>
                    <div>
                        <input
                            class="button tonal"
                            type="submit"
                            value="Сгенерировать"
                        />
                    </div>
                    <div>
                        <label class="field">
                            <span class="label">
                                Форма обращение к клиенту
                            </span>
                            <input type="text" name="client_title" placeholder="Клиент:" />
                        </label>
                    </div>
                    <div>
                        <label class="field">
                            <span class="label">Звание</span>
                            <input
                                type="text"
                                name="therapist_title"
                                placeholder="Психолог"
                            />
                        </label>
                    </div>
                    <div>
                        <label class="field">
                            <span class="label">ФИО Специалиста</span>
                            <input
                                type="text"
                                name="therapist_name"
                                placeholder="Соснина Мария Викторовна"
                            />
                        </label>
                    </div>
                    <div>
                        <label class="field">
                            <span class="label">Название Практики</span>
                            <input type="text" name="practice_name" placeholder="MentalDesk" />
                        </label>
                    </div>
                </ActionForm>
            </div>

            <div class="branding__preview">
                <div class="card tonal">
                    <embed
                        type="application/pdf"
                        src=move || {
                            value
                                .get()
                                .map(|res| match res {
                                    Ok(v) => pdf_bytes_to_url(&v),
                                    Err(_) => "".to_string(),
                                })
                        }
                    />
                </div>
            </div>
        </div>
    }
}


#[cfg(feature = "ssr")]
fn branding_template(content: String, worksheet: &CreateWorksheet) -> Result<String, ServerFnError> {
    use handlebars::Handlebars;
    use std::collections::HashMap;

    let reg = Handlebars::new();
    let s = serde_json::to_string(worksheet).unwrap();
    let data: HashMap<String, String> = serde_json::from_str(&s).unwrap();
    let latex = reg.render_template(&content, &data)?;

    Ok(latex)
}

#[cfg(feature = "ssr")]
pub async fn compile_latex(filename: &str, latex: &str) -> Result<Vec<u8>, ServerFnError> {
    use tempfile::tempdir;
    use std::path::PathBuf;
    use std::process::Stdio;
    use tokio::io::AsyncWriteExt;
    use tokio::process::Command;

    let dir = tempdir()?;
    let workdir = dir.path();
    let tex_path = workdir.join("main.tex");
    let mut file: tokio::fs::File = tokio::fs::File::create(&tex_path).await?;
    file.write_all(latex.as_bytes()).await?;
    file.flush().await?;

    tokio::fs::copy(
        "assets/shared/worksheet_landscape.cls",
        workdir.join("worksheet_landscape.cls"),
    )
    .await?;
    tokio::fs::copy("assets/shared/worksheet.cls", workdir.join("worksheet.cls")).await?;
    tokio::fs::copy("assets/shared/survey.cls", workdir.join("survey.cls")).await?;

    let mut child = Command::new("pdflatex")
        .current_dir(workdir)
        .arg("-interaction=nonstopmode")
        .arg("-halt-on-error")
        .arg("-file-line-error")
        .arg("main.tex")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let _ = child.wait().await?;
    let pdf_path = workdir.join("main.pdf");

    let out = PathBuf::from(filename);
    let data = tokio::fs::read(&pdf_path).await?;
    
    tokio::spawn(async {
        let _ = tokio::fs::copy(pdf_path, out).await;
    });

    Ok(data)
}
