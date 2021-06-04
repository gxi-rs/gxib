//! Web Socket Actor for Actix Web
use crate::*;
use actix::prelude::*;
use actix::{Actor, StreamHandler};
use actix_web::http::StatusCode;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use futures::future::Future;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tokio::sync::watch;
use tokio::task;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Msg sent to WS Actor
#[derive(Debug, Clone)]
pub enum ActorMsg {
    FileChange(String),
    None,
}

impl ToString for ActorMsg {
    fn to_string(&self) -> String {
        match self {
            ActorMsg::FileChange(hashed_name) => format!(
                r#"{{
                    "event":"FileChange",
                    "hashed_name":"{}"
                }}"#,
                hashed_name
            ),
            _ => String::from("{}"),
        }
    }
}

impl actix::Message for ActorMsg {
    type Result = ();
}

/// Web Socket Actor
pub struct WsActor {
    heartbeat: Instant,
    rx: Option<watch::Receiver<ActorMsg>>,
}

/// impl Actor for WsActor
impl Actor for WsActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // listen to watch::channel for msg send by WebPipeline
        if let Some(rx) = &self.rx {
            let mut rx = rx.clone();
            let addr = ctx.address();
            ctx.spawn(actix::fut::wrap_future::<_, Self>(async move {
                let mut once = false;
                while rx.changed().await.is_ok() {
                    if once {
                        let k = rx.borrow();
                        match *k {
                            ActorMsg::None => {}
                            _ => addr
                                .send(k.clone())
                                .await
                                .with_context(|| "Unable to send msg to actor")
                                .unwrap(),
                        }
                    } else {
                        once = true;
                    }
                }
            }));
        }
        // send heartbeat to client every HEARTBEAT_INTERVAL amount of time
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.heartbeat) > CLIENT_TIMEOUT {
                eprintln!("Websocket Client heartbeat failed, disconnecting!");
                ctx.stop();
            } else {
                ctx.ping(b"");
            }
        });
    }
}

/// React to Actor Msgs
impl Handler<ActorMsg> for WsActor {
    type Result = ();
    /// Handle incoming actor messages from rx channel
    fn handle(&mut self, msg: ActorMsg, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.to_string())
    }
}

/// handler for ws::Message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            // send a pong back with the same message
            Ok(ws::Message::Ping(msg)) => {
                self.heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => self.heartbeat = Instant::now(),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

async fn web_socket_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(
        WsActor {
            heartbeat: Instant::now(),
            rx: if let Some(data) = req.app_data::<watch::Receiver<ActorMsg>>() {
                Some(data.clone())
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
    // rm / from path
    let mut path = PathBuf::from(&path[1..]);
    // if uri contains an extension then its a static file
    const OUT_DIR: &str = "target/.gxi";
    const PUBLIC_DIR: &str = "examples/web/public";
    // let mut mime = actix_files::file_extension_to_mime("html");
    if {
        if let Some(ext) = path.extension() {
            let ext = ext.to_str().unwrap();
            // if extension is html then serve index.html
            let is_html = ext == "html";
            if !is_html {
                //  mime = actix_files::file_extension_to_mime(ext);
                //check if file exists in output dir
                let output_path = PathBuf::from(OUT_DIR).join(&path);
                if output_path.exists() {
                    path = output_path;
                } else {
                    path = PathBuf::from(PUBLIC_DIR).join(&path);
                }
            }
            is_html
        } else {
            // if path has no extension then serve html
            true
        }
    } {
        path = PathBuf::from(OUT_DIR).join("index.html")
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
    rx: Option<watch::Receiver<ActorMsg>>,
    serve_dir: PathBuf,
    serve_addrs: String,
    public_dir: PathBuf,
) -> impl Future<Output = Result<Result<()>, task::JoinError>> {
    tokio::task::spawn(async move {
        actix_web::rt::System::new("web server").block_on(async move {
            info!("Serving at http://{}", serve_addrs);
            HttpServer::new(move || {
                if let Some(rx) = &rx {
                    App::new().app_data(rx.clone())
                } else {
                    App::new()
                }
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
