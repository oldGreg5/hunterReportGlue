use std::{collections::HashMap, io::Cursor, path::PathBuf, sync::Arc};

use axum::{
    body::Body,
    extract::State,
    http::{header, HeaderValue, Response, StatusCode},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use docx_rs::*;
use serde::{Deserialize, Serialize};
use tower_http::services::{ServeDir, ServeFile};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WizardStatePayload {
    full_name: String,
    test_date: String,
    manual_notes: String,
    burdens: Vec<String>,
    toxic_metals: String,
    deficiencies: String,
    frequencies: String,
    selected_pathogens: Vec<String>,
    recommendations: String,
}

#[derive(Clone)]
struct AppState {
    descriptions: Arc<HashMap<String, String>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let static_dir = PathBuf::from("static");
    let hunter_report_dir = static_dir.join("hunterReport");
    let descriptions_file = static_dir.join("HunterWyniki.txt");
    let descriptions = Arc::new(load_descriptions(&descriptions_file)?);

    let state = AppState {
        descriptions,
    };

    let app = Router::new()
        .route("/hunterReport/api/docx", post(generate_docx))
        .nest_service("/hunterReport/HunterWyniki.txt", ServeFile::new(descriptions_file))
        .nest_service(
            "/hunterReport",
            ServeDir::new(hunter_report_dir).fallback(ServeFile::new(static_dir.join("hunterReport/index.html"))),
        )
        .with_state(state);

    let addr = "0.0.0.0:8081";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Serving at: http://{addr}/hunterReport");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn generate_docx(
    State(state): State<AppState>,
    Json(payload): Json<WizardStatePayload>,
) -> impl IntoResponse {
    if payload.full_name.trim().is_empty() || payload.test_date.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "full_name and test_date are required").into_response();
    }

    let file_name = format!(
        "{}_Vega_Test_{}.docx",
        sanitize_filename(&payload.full_name),
        sanitize_filename(&payload.test_date)
    );

    let mut docx = Docx::new();
    docx = docx.add_paragraph(
        Paragraph::new().add_run(Run::new().add_text(format!(
            "{} Vega Test {}",
            payload.full_name.trim(),
            payload.test_date.trim()
        ))),
    );
    docx = docx.add_paragraph(Paragraph::new());

    docx = add_heading_and_text(docx, "2. Ręczne notki", &payload.manual_notes);

    docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_text(
        "3. Wskazanie na obciążenia",
    )));
    if payload.burdens.is_empty() {
        docx = docx.add_paragraph(Paragraph::new());
    } else {
        for burden in payload.burdens.iter() {
            docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_text(burden)));
        }
    }

    docx = add_heading_and_text(docx, "4. Metale toksyczne", &payload.toxic_metals);
    docx = add_heading_and_text(docx, "5. Niedobory", &payload.deficiencies);
    docx = add_heading_and_text(
        docx,
        "6. Częstotliwości mogące odpowiadać patogenom",
        &payload.frequencies,
    );

    docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_text(
        "7. Wyklikane z hunterreportglue opisy patogenów",
    )));
    if payload.selected_pathogens.is_empty() {
        docx = docx.add_paragraph(Paragraph::new());
    } else {
        for key in payload.selected_pathogens.iter() {
            let val = state.descriptions.get(key).cloned().unwrap_or_default();
            docx = docx.add_paragraph(
                Paragraph::new()
                    .add_run(Run::new().add_text(key).bold())
                    .add_run(Run::new().add_text(format!(" - {val}"))),
            );
        }
    }

    docx = add_heading_and_text(docx, "8. Zalecenia", &payload.recommendations);

    let mut buffer = Cursor::new(Vec::new());
    if let Err(err) = docx.build().pack(&mut buffer) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("DOCX build failed: {err}"),
        )
            .into_response();
    }

    let bytes = buffer.into_inner();
    let mut res = Response::new(Body::from(bytes));
    *res.status_mut() = StatusCode::OK;
    res.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static(
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        ),
    );
    res.headers_mut().insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{file_name}\"")).unwrap(),
    );
    res
}

fn add_heading_and_text(mut docx: Docx, heading: &str, text: &str) -> Docx {
    docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_text(heading)));
    let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
    let mut has_any = false;
    for line in normalized.split('\n') {
        if !line.trim().is_empty() {
            has_any = true;
            break;
        }
    }
    if !has_any {
        return docx.add_paragraph(Paragraph::new());
    }
    for line in normalized.split('\n') {
        docx = docx.add_paragraph(Paragraph::new().add_run(Run::new().add_text(line)));
    }
    docx
}

fn sanitize_filename(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => out.push('_'),
            c if c.is_whitespace() => out.push('_'),
            c => out.push(c),
        }
    }
    out.trim_matches('_').to_string()
}

fn load_descriptions(path: &PathBuf) -> anyhow::Result<HashMap<String, String>> {
    let text = std::fs::read_to_string(path)?;
    let mut map = HashMap::new();

    for part in text.split(">>").skip(1) {
        let mut pieces = part.split('^');
        let _category = pieces.next().unwrap_or("");
        for item in pieces.skip(1) {
            let mut split = item.split('$');
            let key = split.next().unwrap_or("").trim();
            let val = split.next().unwrap_or("").trim();
            if !key.is_empty() {
                map.insert(key.to_string(), val.to_string());
            }
        }
    }

    Ok(map)
}
