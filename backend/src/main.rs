use std::{collections::HashMap, sync::atomic::{AtomicUsize, Ordering}};

use chrono::Utc;
use common::{ChatMessage, WebsocketMessage, WebsocketMessageType};
use rocket_ws::{stream::DuplexStream, Channel, Message, WebSocket};
use rocket::{futures::{stream::SplitSink, SinkExt, StreamExt}, tokio::sync::Mutex, State};
use serde_json::json;

static USER_ID_COUNTER: AtomicUsize = AtomicUsize::new(1);

#[derive(Default)]
struct ChatRoom {
    connections: Mutex<HashMap<usize, ChatRoomConnection>>
}
struct ChatRoomConnection {
    username: String,
    sink: SplitSink<DuplexStream, Message>
}
impl ChatRoom {
    pub async fn add(&self, id: usize, sink: SplitSink<DuplexStream, Message>) {
        let mut conns = self.connections.lock().await;
        let connection = ChatRoomConnection {
            username: format!("User #{}", id),
            sink
        };
            conns.insert(id, connection);
    }

    pub async fn broadcast_message(&self, message: Message, author_id: usize) {
        let mut conns = self.connections.lock().await;
        let connection = conns.get(&author_id).unwrap();

        let chat_message = ChatMessage {
            message: message.to_string(),
            author: connection.username.clone(),
            created_at: Utc::now().naive_utc(),
        };

        let websocket_message = WebsocketMessage {
            message_type: WebsocketMessageType::NewMessage,
            message: Some(chat_message),
            users: None,
            username: None,
        };
        for (_id, connection) in conns.iter_mut() {
            let _ = connection.sink.send(
                Message::Text(json!(websocket_message).to_string())
            ).await;
        }
    }

    pub async fn broadcast_users(&self) {
        let mut conns = self.connections.lock().await;
        let mut users = vec![];
        for (_id, connection) in conns.iter() {
            users.push(connection.username.clone());
        }
        let websocket_message = WebsocketMessage {
            message_type: WebsocketMessageType::UsersList,
            message: None,
            users: Some(users),
            username: None,
        };
        for (_id, connection) in conns.iter_mut() {
            let _ = connection.sink.send(
                Message::Text(json!(websocket_message).to_string())
            ).await;
        }
    }
    pub async fn send_username(&self, id: usize) {
        let mut conns = self.connections.lock().await;
        let connection = conns.get_mut(&id).unwrap();

        let websocket_message = WebsocketMessage {
            message_type: WebsocketMessageType::UsernameChange,
            message: None,
            users: None,
            username: Some(connection.username.clone()),
        };

        let _ = connection.sink.send(Message::Text(json!(websocket_message).to_string())).await;
    }
    pub async fn remove(&self, id: usize) {
        let mut conns = self.connections.lock().await;
            conns.remove(&id);
    }
}


#[rocket::get("/")]
async fn chat<'r>(ws: WebSocket, state:  &'r State<ChatRoom>) -> Channel<'r> {

    ws.channel(move |stream| Box::pin(async move {
        let user_id = USER_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        let (ws_sink, mut ws_stream) = stream.split();
        state.add(user_id, ws_sink).await;
        state.broadcast_users().await;
        state.send_username(user_id).await;

        while let Some(message) = ws_stream.next().await {
            state.broadcast_message(message?, user_id).await
        }

        state.remove(user_id).await;
        state.broadcast_users().await;

        Ok(())
    }))
}

#[rocket::main] 
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![
            chat,
        ])
        .manage(ChatRoom::default())
        .launch()
        .await;
}