use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::Router;
use serde::Serialize;
use std::fs;
use tera::Tera;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use x509_parser::{self, pem::parse_x509_pem, x509::X509Name};

struct AppError(anyhow::Error);

#[derive(Serialize)]
struct DisplayCert {
    name: String,
    common_name: String,
    issuer: String,
    key_info: String,
    valid_date: String,
    filename: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "cert_file_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(display_certs))
        .nest_service("/certs", ServeDir::new("certs"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:2002")
        .await
        .expect("This address should be free");
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    println!("Hello, world!");
}

async fn display_certs() -> Result<Html<String>, AppError> {
    let tera = Tera::new("templates/**/*").unwrap();

    let mut credentials = Vec::<DisplayCert>::new();

    for entry in fs::read_dir("./certs/")? {
        let path = entry?.path();
        if path.is_file() {
            let data = fs::read(&path)?;
            let (_, pem) = parse_x509_pem(&data).expect("This should be a valid PEM cert");
            let cert = pem.parse_x509().expect("A valid PEM should be convertable");

            let subject_cn = get_first_cn_as_str(cert.subject());
            let issuer_cn = get_first_cn_as_str(cert.issuer());
            let valid_date = cert.validity().not_after.to_string();
            // let key_info = cert.public_key().algorithm.;
            let filename = path.to_str().expect("Files should be unicode").to_string();

            let cert = DisplayCert {
                name: subject_cn.clone(),
                common_name: subject_cn,
                issuer: issuer_cn,
                key_info: "hello".to_string(),
                valid_date,
                filename,
            };

            credentials.push(cert);
        }
    }

    let mut context = tera::Context::new();
    context.insert("certs", &credentials);

    let output = tera.render("certs.html", &context)?;

    Ok(Html::from(output))
}

fn get_first_cn_as_str(name: &X509Name) -> String {
    name.iter_common_name()
        .next()
        .and_then(|cn| cn.as_str().ok())
        .unwrap_or("")
        .to_string()
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
