use crate::pipelines::web::server::ws_actor::{WsActor, WsActorMsg};
use actix_web::http::StatusCode;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use anyhow::{anyhow, Context, Result};
use log::info;
use std::path::PathBuf;
use std::time::Instant;
use tokio::sync::watch;

#[derive(Clone)]
pub struct WebServerState {
    pub output_dir: PathBuf,
    pub public_dir: Option<PathBuf>,
    pub rx: Option<watch::Receiver<WsActorMsg>>,
}

#[actix_web::get("/__gxi__")]
async fn web_socket_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let state: &WebServerState = req.app_data().unwrap();
    ws::start(
        WsActor {
            heartbeat: Instant::now(),
            rx: state.rx.as_ref().cloned(),
        },
        &req,
        stream,
    )
}

async fn index(req: HttpRequest) -> Result<HttpResponse, Error> {
    let path = req.uri().path();
    let mut path = PathBuf::from(&path[1..]); // rm / from path
    let state: &WebServerState = req.app_data().unwrap();
    // if uri contains an extension then its a static file
    if if let Some(ext) = path.extension() {
        let ext = ext.to_str().unwrap();
        // if extension is html then serve index.html
        let is_html = ext == "html";
        if !is_html {
            //check if file exists in output dir
            let output_path = PathBuf::from(&state.output_dir).join(&path);
            if output_path.exists() {
                path = output_path;
            } else if let Some(public_dir) = &state.public_dir {
                path = PathBuf::from(public_dir).join(&path);
            } else {
                return Ok(HttpResponse::new(StatusCode::NOT_FOUND));
            }
        }
        is_html
    } else {
        // if path has no extension then serve html
        true
    } {
        path = PathBuf::from(&state.output_dir).join("index.html")
    }
    // if path exist then serve it
    if path.exists() {
        Ok(actix_files::NamedFile::open(path)?
            .prefer_utf8(true)
            .into_response(&req))
    } else {
        Ok(HttpResponse::new(StatusCode::NOT_FOUND))
    }
}

pub async fn start_web_server(state: WebServerState, serve_addrs: String) -> Result<()> {
    info!("initializing server to listen at http://{}", serve_addrs);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(web_socket_route)
            .default_service(web::route().to(index))
    })
    .disable_signals()
    .bind(serve_addrs.clone())?
    .run()
    .await
    .context("Error running web server")?;

    Err::<(), anyhow::Error>(anyhow!("Web server exited unexpectedly"))
}
