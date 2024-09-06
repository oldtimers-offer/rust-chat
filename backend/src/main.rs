use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

use chrono::Utc;
use common::WebSocketMessage;
use rocket::{
    futures::{stream::SplitSink, SinkExt, StreamExt},
    tokio::sync::Mutex,
    State,
};

use serde_json::json;

use rocket_ws::{stream::DuplexStream, Channel, Message, WebSocket};

static USER_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Default)]
struct ChatRoom {
    connections: Mutex<HashMap<usize, SplitSink<DuplexStream, Message>>>,
}

impl ChatRoom {
    pub async fn add(&self, id: usize, sink: SplitSink<DuplexStream, Message>) {
        let mut conns = self.connections.lock().await;
        conns.insert(id, sink);
    }
    pub async fn remove(&self, id: usize) {
        let mut conns = self.connections.lock().await;
        conns.remove(&id);
    }

    pub async fn broadcast_message(&self, message: Message, user_id: usize) {
        let chat_message = common::ChatMessages {
            message: message.to_string(),
            author: format!("User id:{}", user_id),
            created_at: Utc::now().naive_utc(),
        };

        let websocket_message = WebSocketMessage {
            message_type: common::WebSocketMessageType::NewMessage,
            message: Some(chat_message),
            users: None,
        };
        let mut conns = self.connections.lock().await;
        for (_id, sink) in conns.iter_mut() {
            let _ = sink
                .send(Message::Text(json!(websocket_message).to_string()))
                .await;
        }
    }

    pub async fn broadcast_user_list(&self) {
        let mut conns = self.connections.lock().await;

        let mut users = vec![];

        for (id, _) in conns.iter() {
            users.push(format!("User #{}", id));
        }

        let websocket_message = WebSocketMessage {
            message_type: common::WebSocketMessageType::UsersList,
            message: None,
            users: Some(users),
        };

        for (_id, sink) in conns.iter_mut() {
            let _ = sink
                .send(Message::Text(json!(websocket_message).to_string()))
                .await;
        }
    }
}

#[rocket::get("/")]
fn chat<'r>(ws: WebSocket, tate: &'r State<ChatRoom>) -> Channel<'r> {
    ws.channel(move |stream| {
        Box::pin(async move {
            let user_id = USER_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
            let (ws_sink, mut ws_stream) = stream.split();
            tate.add(user_id, ws_sink).await;
            tate.broadcast_user_list().await;

            while let Some(message) = ws_stream.next().await {
                tate.broadcast_message(message?, user_id).await;
            }

            tate.remove(user_id).await;
            tate.broadcast_user_list().await;

            Ok(())
        })
    })
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![chat])
        .manage(ChatRoom::default())
        .launch()
        .await;
}
