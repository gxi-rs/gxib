use crate::pipelines::web::server::ws_actor::{WsActor, WsActorMsg};
use crate::*;
use actix_web::http::StatusCode;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use futures::future::Future;
use std::path::PathBuf;
use std::time::Instant;
use tokio::sync::watch;
use tokio::task;

#[derive(Clone)]
pub struct WebServerState {
    pub output_dir: PathBuf,
    pub public_dir: Option<PathBuf>,
    pub rx: Option<watch::Receiver<WsActorMsg>>,
}

async fn web_socket_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let state: &WebServerState = req.app_data().unwrap();
    ws::start(
        WsActor {
            heartbeat: Instant::now(),
            rx: if let Some(rx) = &state.rx {
                Some(rx.clone())
            } else {
                None
            },
        },
        &req,
        stream,
    )
}

#[actix_web::get("/*")]
async fn index(req: HttpRequest) -> Result<HttpResponse, Error> {
    let path = req.uri().path();
    let mut path = PathBuf::from(&path[1..]); // rm / from path
    let state: &WebServerState = req.app_data().unwrap();
    if {
        // if uri contains an extension then its a static file
        if let Some(ext) = path.extension() {
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
        }
    } {
        path = PathBuf::from(&state.output_dir).join("index.html")
    }
    // if path exist then serve it
    return if path.exists() {
        Ok(actix_files::NamedFile::open(path)?
            .prefer_utf8(true)
            .into_response(&req)?)
    } else {
        Ok(HttpResponse::new(StatusCode::NOT_FOUND))
    };
}

pub fn start_web_server(
    state: WebServerState,
    serve_addrs: String,
) -> impl Future<Output = Result<Result<()>, task::JoinError>> {
    tokio::task::spawn(async move {
        actix_web::rt::System::new("web server").block_on(async move {
            info!("initialising server to listen at http://{}", serve_addrs);
            HttpServer::new(move || {
                App::new()
                    .app_data(state.clone())
                    .route("/__gxi__", web::get().to(web_socket_route))
                    .service(index)
            })
            .disable_signals()
            .bind(serve_addrs.clone())?
            .run()
            .await
            .with_context(|| "Error running web server")?;
            Err::<(), anyhow::Error>(anyhow!("Web server exited unexpectedly"))
        })?;
        Err::<(), anyhow::Error>(anyhow!("Web server exited unexpectedly"))
    })
}
