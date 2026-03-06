use std::collections::HashMap;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::io::{AsyncBufReadExt, BufReader, BufWriter};
use tokio::process::Command;

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

pub async fn compile_latex(filename: &str, latex: &str) -> Result<(), ServerFnError> {
    // let dir = tempdir()?;
    // let workdir = dir.path();
    // let tex_path = workdir.join("main.tex");
    // let mut file = tokio::fs::File::create(&tex_path).await?;
    // // todo
    // // tokio::io::copy(&mut reader, &mut file).await?;
    // file.write_all(latex.as_bytes()).await?;
    // file.flush().await?;

    // tokio::fs::copy(
    //     "assets/shared/worksheet_landscape.cls",
    //     workdir.join("worksheet_landscape.cls"),
    // )
    // .await?;
    // tokio::fs::copy("assets/shared/worksheet.cls", workdir.join("worksheet.cls")).await?;
    // tokio::fs::copy("assets/shared/survey.cls", workdir.join("survey.cls")).await?;

    // let mut child = Command::new("pdflatex")
    //     .current_dir(workdir)
    //     .arg("-interaction=nonstopmode")
    //     .arg("-halt-on-error")
    //     .arg("-file-line-error")
    //     .arg("main.tex")
    //     .stdout(Stdio::piped())
    //     .stderr(Stdio::piped())
    //     .spawn()?;

    // let stdout = child.stdout.take().ok_or_else(|| "stdout not available");
    // let stderr = child.stderr.take().ok_or_else(|| "stderr not available");

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

    // let status = child.wait().await?;
    // let _ = out_task.await;
    // let _ = err_task.await;

    // let pdf_path = workdir.join("main.pdf");

    // let out = PathBuf::from(filename);
    // tokio::fs::copy(&pdf_path, &out).await?;

    // Ok(out)
    Ok(())
}

fn branding_(content: String, worksheet: &CreateWorksheet) -> Result<String, ServerFnError> {
    let reg = Handlebars::new();
    let s = serde_json::to_string(worksheet).unwrap();
    let data: HashMap<String, String> = serde_json::from_str(&s).unwrap();
    let latex = reg.render_template(&content, &data)?;
    
    Ok(latex)
}

async fn create_worksheet(
    payload: CreateWorksheet,
) -> Result<(), ServerFnError> {
    let template_path = format!("{}/worksheet.tex", payload.template_path);
    let content = fs::read_to_string(&template_path).await?;

    // let reg = Handlebars::new();
    // let mut data: HashMap<String, String> = HashMap::default();
    // data.insert("client_title".to_owned(), "client title".to_owned());
    // data.insert("clientName".to_owned(), "client name".to_owned());

    // let latex = reg.render_template(&content, &data)?;

    let latex = branding_(content, &payload)?;

    let hash = Sha256::digest(&latex);
    let filename = format!("assets/branded/worksheets/{:x}.pdf", hash);
    let path = compile_latex(&filename, &latex).await?;
    // let data = tokio::fs::read(path).await?;

    // let worksheet_response = WorksheetResponse { filename, data };
    // Ok(Json(worksheet_response))
    
    Ok(())
}
