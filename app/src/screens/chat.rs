use backend::*;
use leptos::ev::SubmitEvent;
use leptos::*;

#[component]
pub fn ChatScreen() -> impl IntoView {
    let (messages, set_messages) = signal(Vec::<String>::new());
    let (input_value, set_input_value) = signal(String::new());

    let send_message = move |ev: SubmitEvent| {
        ev.prevent_default();
        let msg = input_value.get();
        if !msg.is_empty() {
            set_messages.update(|msgs| msgs.push(msg.clone()));
            set_input_value.set(String::new());
        }
    };

    let update_input = move |ev| {
        let value = event_target_value(&ev);
        set_input_value.set(value);
    };

    view! {
        <div class="screen chat-screen">
            <div class="chat-header">
                <h2>"Chat"</h2>
                <p>"Connect with peers"</p>
            </div>

            <div class="messages-container">
                {move || messages.get().iter().enumerate().map(|(_i, msg)| {
                    view! {
                        <div class="message">
                            <span class="message-author">"You: "</span>
                            <span class="message-content">{msg.clone()}</span>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>

            <form class="message-input-form" on:submit=send_message>
                <input
                    type="text"
                    class="message-input"
                    placeholder="Type a message..."
                    value=move || input_value.get()
                    on:input=update_input
                />
                <button type="submit" class="send-button">
                    <span>"✈️"</span>
                </button>
            </form>
        </div>
    }
}
