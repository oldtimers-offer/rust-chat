use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub send_message_callback: Callback<String>,
}

#[function_component(SendDialog)]
pub fn send_dialog(props: &Props) -> Html {
    let new_message_handle = use_state(String::default);
    let new_message = (*new_message_handle).clone();

    let cloned_new_message_handle = new_message_handle.clone();
    let on_mesage_change = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlTextAreaElement>();
        if let Some(textarea) = target {
            cloned_new_message_handle.set(textarea.value());
        }
    });

    let cloned_new_message = new_message.clone();
    let cloned_props = props.send_message_callback.clone();
    let on_button_click = Callback::from(move |_e: MouseEvent| {
        cloned_props.emit(cloned_new_message.clone());
        new_message_handle.set("".to_string());
    });

    html! {
    <div class="input-group">
            <textarea class="form-control" onchange={on_mesage_change} value={new_message}></textarea>
            <button type="submit" class="btn-primary" onclick={on_button_click}>{"Send"}</button>
            </div>
    }
}
