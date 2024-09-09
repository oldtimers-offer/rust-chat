use common::ChatMessage;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub messages: Vec<ChatMessage>,
}

#[function_component(MessageList)]
pub fn message_list(props: &Props) -> Html {
    html! {

    <div id= "chat" class="list-group">
            {
                props.messages.iter().map(|m| html! {
                    <li class="list-group-item">
                    <div class="d-flex w-100 justify-content-between">
                    <h5>{m.author.clone()}</h5>
                    <small>{m.created_at.format("%Y-%m-%d %H:%M:%S").to_string()}</small>
                    </div>
                    <p>{m.message.clone()}</p>
                    </li>}
                ).collect::<Html>()
            }
            </div>
    }
}
