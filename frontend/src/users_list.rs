use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub users: Vec<String>,
}

#[function_component(UsersList)]
pub fn users_list(props: &Props) -> Html {
    html! {
        <div id= "users" class="list-group">
            {
                props.users.iter().map(|username| html! {
                    <li>{username}</li>
                }
                ).collect::<Html>()
            }
            </div>
    }
}
