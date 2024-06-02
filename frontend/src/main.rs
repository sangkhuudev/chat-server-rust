use common::{WebsocketMessage, WebsocketMessageType};
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew_hooks::use_websocket;

#[function_component]
fn App() -> Html {
    let message_handle = use_state(Vec::default);
    let message = (*message_handle).clone();
    let mut cloned_message = message.clone();

    let new_message_handle = use_state(String::default);
    let new_message = (*new_message_handle).clone();
    let cloned_new_message = new_message.clone();

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
                WebsocketMessageType::UsersList => {}
            }
    
        }
    });

    let cloned_new_message_handle = new_message_handle.clone();
    let on_message_change = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlTextAreaElement>();
        if let Some(t) = target {
            cloned_new_message_handle.set(t.value());
        }
    });

    let cloned_ws = ws.clone();
    let on_button_click = Callback::from(move |_: MouseEvent| {
        cloned_ws.send(cloned_new_message.clone());
        new_message_handle.set("".to_string());
    });
    html! {
        <div class="container">
            <div class="row">
                <div class="list-group">
                {
                    message.iter().map(|m| html!{
                        <div class="list-group-item list-group-item-action">
                            <div class="d-flex w-100 flex-row justify-content-between">              
                                <h5 class ="flex-fill">{m.author.clone()}</h5>
                                <small class ="flex-fill">{m.created_at.format("%Y-%m-%d %H:%M:%S").to_string()}</small>
                                <p class ="flex-fill">{m.message.clone()}</p>
                            </div>
                        </div>
                    }).collect::<Html>()
                }
                </div>
            </div>
            <div class="row">
                <div class="input-group">
                    <textarea class="form-control" onchange={on_message_change} value={new_message}></textarea>
                    <button class="btn-primary" type="submit" onclick={on_button_click}>{"Send"}</button>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
