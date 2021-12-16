//! Web Socket Actor for Actix Web
use actix::prelude::*;
use actix::{Actor, StreamHandler};
use actix_web_actors::ws;
use anyhow::Context;
use log::warn;
use std::time::{Duration, Instant};
use tokio::sync::watch;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Msg sent to WS Actor
#[derive(Debug, Clone)]
pub enum WsActorMsg {
    FileChange(String),
    None,
}

impl ToString for WsActorMsg {
    fn to_string(&self) -> String {
        match self {
            WsActorMsg::FileChange(hashed_name) => format!(
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

impl actix::Message for WsActorMsg {
    type Result = ();
}

/// Web Socket Actor
pub struct WsActor {
    pub(crate) heartbeat: Instant,
    pub(crate) rx: Option<watch::Receiver<WsActorMsg>>,
}

/// impl Actor for WsActor
impl Actor for WsActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // listen to watch::channel for msg send by WebPipeline
        if let Some(rx) = &self.rx {
            let mut rx = rx.clone();
            let adds = ctx.address();
            ctx.spawn(actix::fut::wrap_future::<_, Self>(async move {
                let mut once = false;
                while rx.changed().await.is_ok() {
                    if once {
                        let k = rx.borrow();
                        match *k {
                            WsActorMsg::None => {}
                            _ => adds
                                .send(k.clone())
                                .await
                                .context("Unable to send msg to actor")
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
                warn!("Websocket Client heartbeat failed, disconnecting!");
                ctx.stop();
            } else {
                ctx.ping(b"");
            }
        });
    }
}

/// React to Actor Msgs
impl Handler<WsActorMsg> for WsActor {
    type Result = ();
    /// Handle incoming actor messages from rx channel
    fn handle(&mut self, msg: WsActorMsg, ctx: &mut Self::Context) -> Self::Result {
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
