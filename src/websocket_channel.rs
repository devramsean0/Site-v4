use actix::Actor;
use actix::AsyncContext;
use actix::{ActorContext, Addr, Context, Handler, StreamHandler};
use actix_web_actors::ws; // Import the trait for stop()
pub struct WsSession {
    pub channel_name: String,
    pub channels: Addr<ChannelsActor>,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::debug!("WsSession started, subscribing to {}", self.channel_name);
        self.channels.do_send(Subscribe {
            channel: self.channel_name.clone(),
            addr: ctx.address().recipient(),
        });
    }
}

// Handle messages from the client
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                log::debug!("Received from client: {}", text);
                ctx.text(format!("echo: {}", text));
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Close(reason)) => {
                log::debug!("Client disconnected");
                ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}

// Handle messages from ChannelsActor
impl Handler<BroadcastMessage> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, ctx: &mut Self::Context) {
        log::debug!("Broadcast delivered to session: {}", msg.0);
        ctx.text(msg.0);
    }
}

use actix::{Message, Recipient};
use std::collections::HashMap;

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct BroadcastMessage(pub String);

pub struct Channel {
    pub clients: Vec<Recipient<BroadcastMessage>>,
}

pub struct Channels {
    pub inner: HashMap<String, Channel>,
}

impl Channels {
    pub fn new() -> Self {
        Channels {
            inner: HashMap::new(),
        }
    }

    pub fn subscribe(&mut self, channel: &str, client: Recipient<BroadcastMessage>) {
        self.inner
            .entry(channel.to_string())
            .or_insert(Channel { clients: vec![] })
            .clients
            .push(client);
    }

    pub fn broadcast(&self, channel: &str, msg: String) {
        if let Some(ch) = self.inner.get(channel) {
            for client in &ch.clients {
                let _ = client.do_send(BroadcastMessage(msg.clone()));
            }
        }
    }
}

pub struct ChannelsActor {
    state: Channels,
}

impl ChannelsActor {
    pub fn new() -> Self {
        ChannelsActor {
            state: Channels::new(),
        }
    }
}

impl Actor for ChannelsActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Subscribe {
    pub channel: String,
    pub addr: Recipient<BroadcastMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Publish {
    pub channel: String,
    pub payload: String,
}

impl Handler<Subscribe> for ChannelsActor {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, _: &mut Self::Context) {
        log::debug!("Subscribing to channel: {}", msg.channel);
        self.state.subscribe(&msg.channel, msg.addr);
    }
}

impl Handler<Publish> for ChannelsActor {
    type Result = ();

    fn handle(&mut self, msg: Publish, _: &mut Self::Context) {
        log::debug!("Publishing to channel: {}", msg.channel);
        self.state.broadcast(&msg.channel, msg.payload);
    }
}
