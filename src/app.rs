use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use std::collections::HashMap;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use tempfile::tempdir;

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
pub async fn branding(client_name: String, client_title: String) -> Result<(), ServerFnError> {
    let client_title = if client_title.trim().is_empty() {
        "Клиент:".to_string()
    } else {
        client_title
    };


    let payload = CreateWorksheet {
        template_path: "assets/worksheets/bolevoj-dnevnik".to_string(),
        client_title: "String".to_string(),
        client_name: client_name.clone(),
        practice_name: "String".to_string(),
        therapist_title: "String".to_string(),
        therapist_name: "String".to_string(),
    };

    let template_path = format!("{}/worksheet.tex", payload.template_path);
    let content = tokio::fs::read_to_string(&template_path).await?;

    let reg = Handlebars::new();
    let mut data: HashMap<String, String> = HashMap::default();
    // data.insert("client_title".to_owned(), "client title".to_owned());
    data.insert("clientName".to_owned(), "client name".to_owned());

    let latex = reg.render_template(&content, &data)?;
    let latex = branding_(content, &payload)?;

    let hash = Sha256::digest(&latex);
    let filename = format!("assets/branded/worksheets/{:x}.pdf", hash);
    let path = compile_latex(&filename, &latex).await?;
    let data = tokio::fs::read(path).await?;

    Ok(())
}

#[component]
fn BrandingPage() -> impl IntoView {
    let branding = ServerAction::<Branding>::new();
    // holds the latest *returned* value from the server
    let value = branding.value();
    // check if the server has returned an error
    let has_error = move || value.with(|val| matches!(val, Some(Err(_))));

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
                            <input type="text" name="client_name"  />
                        </label>
                    </div>
                    <div>
                        <label class="field">
                            <span class="label">
                                Форма обращение к клиенту
                            </span>
                            <input
                                type="text"
                                name="client_title"
                                placeholder="Клиент:"
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

#[derive(Deserialize, Serialize, Debug)]
struct CreateWorksheet {
    template_path: String,
    client_title: String,
    client_name: String,
    practice_name: String,
    therapist_title: String,
    therapist_name: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, PartialOrd, Ord, Eq)]
struct Worksheet {
    path: String,
    mdx: String,
}

#[cfg(feature = "ssr")]
fn branding_(content: String, worksheet: &CreateWorksheet) -> Result<String, ServerFnError> {
    let reg = Handlebars::new();
    let s = serde_json::to_string(worksheet).unwrap();
    let data: HashMap<String, String> = serde_json::from_str(&s).unwrap();
    let latex = reg.render_template(&content, &data)?;
    
    Ok(latex)
}

#[cfg(feature = "ssr")]
pub async fn compile_latex(filename: &str, latex: &str) -> Result<PathBuf, ServerFnError> {
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;
    use tokio::io::{AsyncBufReadExt, BufReader, BufWriter};
    use tokio::process::Command;
    use std::process::Stdio;

    
    let dir = tempdir()?;
    let workdir = dir.path();
    let tex_path = workdir.join("main.tex");
    let mut file: tokio::fs::File = tokio::fs::File::create(&tex_path).await?;
    // todo
    // tokio::io::copy(&mut reader, &mut file).await?;
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

    let stdout = child.stdout.take().ok_or_else(|| "stdout not available");
    let stderr = child.stderr.take().ok_or_else(|| "stderr not available");

    // let out_task = tokio::spawn(async move {
    //     let mut lines = BufReader::new(stdout).lines();
    //     while let Ok(Some(line)) = lines.next_line().await {
    //         println!("[pdflatex] {line}");
    //     }
    // });

    // let err_task = tokio::spawn(async move {
    //     let mut lines = BufReader::new(stderr).lines();
    //     while let Ok(Some(line)) = lines.next_line().await {
    //         eprintln!("[pdflatex:err] {line}");
    //     }
    // });

    let status = child.wait().await?;
    // let _ = out_task.await;
    // let _ = err_task.await;


    let pdf_path = workdir.join("main.pdf");

    let out = PathBuf::from(filename);
    tokio::fs::copy(&pdf_path, &out).await?;

    Ok(out)
}


#[cfg(feature = "ssr")]
async fn create_worksheet(
    payload: CreateWorksheet,
) -> Result<(), ServerFnError> {
    dbg!(&payload);

    let template_path = format!("{}/worksheet.tex", payload.template_path);
    let content = tokio::fs::read_to_string(&template_path).await?;

    let reg = Handlebars::new();
    let mut data: HashMap<String, String> = HashMap::default();
    // data.insert("client_title".to_owned(), "client title".to_owned());
    data.insert("clientName".to_owned(), "client name".to_owned());

    let latex = reg.render_template(&content, &data)?;

    let latex = branding_(content, &payload)?;

    let hash = Sha256::digest(&latex);
    let filename = format!("assets/branded/worksheets/{:x}.pdf", hash);
    let path = compile_latex(&filename, &latex).await?;
    let data = tokio::fs::read(path).await?;

    // let worksheet_response = WorksheetResponse { filename, data };
    // Ok(Json(worksheet_response))
    
    Ok(())
}

