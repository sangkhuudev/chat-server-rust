use std::{collections::HashMap, sync::atomic::{AtomicUsize, Ordering}};

use common::{ChatMessage, WebsocketMessage, WebsocketMessageType};
use rocket_ws::{stream::DuplexStream, Channel, Message, WebSocket};
use rocket::{futures::{stream::SplitSink, SinkExt, StreamExt}, tokio::sync::Mutex, State};
use serde_json::json;
use serde_json::Result as SerdeResult;

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

    pub async fn broadcast_message(&self, message: ChatMessage) {
        let mut conns = self.connections.lock().await;
        let websocket_message = WebsocketMessage {
            message_type: WebsocketMessageType::NewMessage,
            message: Some(message),
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
        if let Some(connection) = conns.get_mut(&id) {
            let websocket_message = WebsocketMessage {
                message_type: WebsocketMessageType::UsernameChange,
                message: None,
                users: None,
                username: Some(connection.username.clone()),
            };

            if let Err(e) = connection.sink.send(Message::Text(json!(websocket_message).to_string())).await {
                eprintln!("Failed to send message: {}", e);
            }
        } else {
            eprintln!("No connection found with id: {}", id);
        }
    }

    pub async fn change_username(&self, new_username: String, id: usize) {
        let mut conns = self.connections.lock().await;
        if let Some(connection) = conns.get_mut(&id) {
            connection.username = new_username;
        } else {
            eprintln!("No connection found with id: {}", id);
        }
    }

    pub async fn remove(&self, id: usize) {
        let mut conns = self.connections.lock().await;
            conns.remove(&id);
    }
}

//----------------------------------------------------------------------------
// Helper functions
async fn handle_incoming_message(
    message_contents: Message,
    state: &State<ChatRoom>,
    connection_id: usize,
) {
    if let Message::Text(json) = message_contents {
        match parse_websocket_message(&json) {
            Ok(ws_message) => process_websocket_message(ws_message, state, connection_id).await,
            Err(_) => {
                // Handle parsing error if needed
            },
        }
    } else {
        // Handle unsupported message type if needed
    }
}

fn parse_websocket_message(json: &str) -> SerdeResult<WebsocketMessage> {
    serde_json::from_str::<WebsocketMessage>(json)
}

async fn process_websocket_message(
    ws_message: WebsocketMessage,
    state: &State<ChatRoom>,
    connection_id: usize,
) {
    match ws_message.message_type {
        WebsocketMessageType::NewMessage => {
            if let Some(ws_msg) = ws_message.message {
                state.broadcast_message(ws_msg).await;
            }
        },
        WebsocketMessageType::UsernameChange => {
            if let Some(ws_username) = ws_message.username {
                state.change_username(ws_username, connection_id).await;
                state.send_username(connection_id).await;
                state.broadcast_users().await;
            }
        },
        _ => {
            // Handle unsupported WebsocketMessageType if needed
        }
    }
}
// //---------------------------------------------------------------

#[rocket::get("/")]
async fn chat<'r>(ws: WebSocket, state:  &'r State<ChatRoom>) -> Channel<'r> {

    ws.channel(move |stream| Box::pin(async move {
        let user_id = USER_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        let (ws_sink, mut ws_stream) = stream.split();
        state.add(user_id, ws_sink).await;
        state.broadcast_users().await;
        state.send_username(user_id).await;

        while let Some(message) = ws_stream.next().await {
            if let Ok(message_contents) = message {
                handle_incoming_message(message_contents, state, user_id).await;
            }
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