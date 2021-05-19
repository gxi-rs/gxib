//! Web Socket Actor for Actix Web
use actix::{Actor, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use crate::*;
use futures::future::Future;
use tokio::task;

pub struct WsActor;

impl Actor for WsActor {
    type Context = ws::WebsocketContext<Self>;
}

/// handeler for ws::Message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsActor {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(WsActor {}, &req, stream);
    println!("{:?}", resp);
    resp
}

pub fn start_web_server() -> impl Future<Output = Result<Result<()>,task::JoinError>> {
    tokio::task::spawn(async {
        actix_web::rt::System::new("web server").block_on(async {
            HttpServer::new(|| App::new().route("/__gxib__", web::get().to(index)))
                .bind("127.0.0.1:8080")?
                .run()
                .await
                .with_context(|| "Error running web server")?;
            Err::<(),anyhow::Error>(anyhow!("Web server exited unexpectidly"))
        })?;
        Err::<(),anyhow::Error>(anyhow!("Web server exited unexpectidly"))
    })
}
