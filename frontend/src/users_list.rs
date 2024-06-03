use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub users: Vec<String>,
}

#[function_component(UsersList)]
pub fn users_list(props: &Props) -> Html {
    html! {
        <ul class="list-group list-group-flush">
        <li class="list-group-item list-group-item-dark">{"List of users in the chat room"}</li>
            {
                props.users.iter().map(|username| html!{
                    <li class="list-group-item list-group-item-success" >{username}</li>
                }).collect::<Html>()
            }
        </ul>
    }
}    