use chrono::Utc;
use common::{ChatMessage, WebsocketMessage, WebsocketMessageType};
use message_list::MessageList;
use send_dialog::SendDialog;
use serde_json::json;
use users_list::UsersList;
use yew::prelude::*;
use yew_hooks::use_websocket;

mod message_list;
mod send_dialog;
mod users_list;

#[function_component]
fn App() -> Html {
    let message_handle = use_state(Vec::default);
    let message = (*message_handle).clone();
    let mut cloned_message = message.clone();
    let users_handle = use_state(Vec::default);
    let users = (*users_handle).clone();
    let username_handle = use_state(String::default);
    let username = (*username_handle).clone();

    // let new_message_handle = use_state(String::default);
    // let new_username_handle = use_state(String::default);

    let ws = use_websocket("ws://127.0.0.1:8000/".to_string());

    use_effect_with(ws.message.clone(), move |ws_message| {
        if let Some(msg) = &**ws_message {
            let websocket_message: WebsocketMessage = serde_json::from_str(msg).unwrap();
            match websocket_message.message_type {
                WebsocketMessageType::NewMessage => {
                    let text = websocket_message.message.expect("Missing message payload");
                    cloned_message.push(text);
                    message_handle.set(cloned_message);
                },
                WebsocketMessageType::UsersList => {
                    let user = websocket_message.users.expect("Missing users payload");
                    users_handle.set(user);
                },
                WebsocketMessageType::UsernameChange => {
                    let username = websocket_message.username.expect("Missing username payload");
                    username_handle.set(username);
                }
            }
        }
    });

    let cloned_username = username.clone();
    let cloned_ws = ws.clone();
    let send_dialog_callback = Callback::from(move |msg: String| {
        let websocket_message = WebsocketMessage {
            message_type: WebsocketMessageType::NewMessage,
            message: Some( ChatMessage {
                message: msg,
                author: cloned_username.clone(),
                created_at: Utc::now().naive_utc()
            }),
            users: None,
            username: None,
        };
        cloned_ws.send(json!(websocket_message).to_string());
    });

    let cloned_ws = ws.clone();
    let username_change_callback = Callback::from(move |username: String| {
        let websocket_message = WebsocketMessage {
            message_type: WebsocketMessageType::UsernameChange,
            message: None,
            users: None,
            username: Some(username),
        };
        cloned_ws.send(json!(websocket_message).to_string());
    });
    html! {
        <div class="container-fluid">
            <div class="row">
                <div class="col-sm-3">
                    <UsersList users={users} />
                </div>
                <div class="col-sm-9">
                    <MessageList message={message}/>
                </div>
            </div>
            <div class="row">
                if username.len() > 0 {
                    <SendDialog 
                        username_change_callback={username_change_callback}
                        send_dialog_callback={send_dialog_callback} 
                        username={username}
                    />
                }
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
