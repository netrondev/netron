use crate::components::{Tooltip, UserAvatar};
use crate::date_utils::{format_time_iso, TimeFormatVariant};

use leptos::prelude::*;

use crate::chat::models::{get_chat_history, get_user_info, ChatEventType};
use crate::chat::shared::{ChatMessage, WsMessage};

#[cfg(not(feature = "ssr"))]
use {
    wasm_bindgen::prelude::*,
    web_sys::js_sys,
    web_sys::{MessageEvent, WebSocket},
};

use crate::RecordId;

#[component]
pub fn ChatApp() -> impl IntoView {
    #[allow(unused)]
    let (messages, set_messages) = signal(Vec::<WsMessage>::new());
    let (input_value, set_input_value) = signal(String::new());
    #[allow(unused)]
    let (connected, set_connected) = signal(false);
    let (new_message_count, _set_new_message_count) = signal(0usize);
    let messages_container_ref = NodeRef::<leptos::html::Div>::new();

    // Load chat history
    let chat_history = Resource::new(|| (), |_| async move { get_chat_history(Some(50)).await });

    // Convert loaded history to WsMessage format
    Effect::new(move |_| {
        if let Some(Ok(history)) = chat_history.get() {
            let ws_messages: Vec<WsMessage> = history
                .into_iter()
                .map(|event| match event.event_type {
                    ChatEventType::Message => WsMessage::Message(ChatMessage {
                        user_id: event.user_id.unwrap_or_else(|| {
                            // Fallback for anonymous users (shouldn't happen now)

                            RecordId::from(("user", "anonymous"))
                        }),
                        username: event.username,
                        message: event.message.unwrap_or_default(),
                        timestamp: event.timestamp.to_string(),
                    }),
                    ChatEventType::UserJoined => WsMessage::UserJoined {
                        username: event.username,
                    },
                    ChatEventType::UserLeft => WsMessage::UserLeft {
                        username: event.username,
                    },
                })
                .collect();
            set_messages.set(ws_messages);
        }
    });

    #[cfg(not(feature = "ssr"))]
    {
        Effect::new(move |_| {
            use leptos::logging::log;

            let window = web_sys::window().expect("window");
            let location = window.location();
            let protocol = location.protocol().expect("protocol");
            let host = location.host().expect("host");

            let ws_protocol = if protocol == "https:" { "wss:" } else { "ws:" };
            let ws_url = format!("{}//{}/api/chat/ws", ws_protocol, host);

            log!("Connecting to WebSocket at: {}", ws_url);

            match WebSocket::new(&ws_url) {
                Ok(ws) => {
                    let ws_clone = ws.clone();

                    let onopen = Closure::wrap(Box::new(move || {
                        log!("WebSocket connected");
                        set_connected.set(true);
                    }) as Box<dyn Fn()>);
                    ws.set_onopen(Some(onopen.as_ref().unchecked_ref()));
                    onopen.forget();

                    let onmessage = Closure::wrap(Box::new(move |e: MessageEvent| {
                        if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                            let msg_str: String = txt.into();
                            if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&msg_str) {
                                // Increment new message count for visual indication
                                if let WsMessage::Message(_) = &ws_msg {
                                    _set_new_message_count.update(|count| *count += 1);
                                }
                                set_messages.update(|msgs| msgs.push(ws_msg));
                            }
                        }
                    })
                        as Box<dyn Fn(MessageEvent)>);
                    ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
                    onmessage.forget();

                    let onclose = Closure::wrap(Box::new(move || {
                        log!("WebSocket disconnected");
                        set_connected.set(false);
                    }) as Box<dyn Fn()>);
                    ws.set_onclose(Some(onclose.as_ref().unchecked_ref()));
                    onclose.forget();

                    // Store websocket in window for later access
                    let window = web_sys::window().expect("window");
                    js_sys::Reflect::set(&window, &JsValue::from_str("chat_ws"), &ws_clone)
                        .unwrap();
                }
                Err(e) => {
                    log!("Failed to create WebSocket: {:?}", e);
                }
            }
        });

        // Auto-scroll to bottom when new messages arrive
        Effect::new(move |_| {
            let _ = messages.get(); // Subscribe to message changes
            if let Some(container) = messages_container_ref.get() {
                // Scroll to bottom immediately
                container.set_scroll_top(container.scroll_height());
            }
        });
    }

    let send_message = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let msg = input_value.get();
        if msg.trim().is_empty() {
            return;
        }

        #[cfg(not(feature = "ssr"))]
        {
            let window = web_sys::window().expect("window");
            if let Ok(ws_value) = js_sys::Reflect::get(&window, &JsValue::from_str("chat_ws")) {
                if let Ok(ws) = ws_value.dyn_into::<WebSocket>() {
                    if ws.ready_state() == WebSocket::OPEN {
                        let _ = ws.send_with_str(&msg);
                        set_input_value.set(String::new());
                        // Reset new message count when user sends a message
                        _set_new_message_count.set(0);
                    }
                }
            }
        }
    };

    view! {
        <div class="flex flex-col h-full bg-white dark:bg-neutral-950">
            <div class="">
                <div class="">
                    <div class="flex items-center justify-between mt-2">
                        <div class="flex items-center">
                            <div class={move || if connected.get() { "w-5 h-5 bg-green-500 rounded-full" } else { "w-5 h-5 bg-red-500 rounded-full" }}></div>
                        </div>
                        {move || {
                            let count = new_message_count.get();
                            if count > 0 {
                                view! {
                                    <div class="bg-blue-500 text-white text-xs px-2 py-1 rounded-full animate-pulse">
                                        {if count == 1 { "New message".to_string() } else { format!("{} new messages", count) }}
                                    </div>
                                }.into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }
                        }}
                    </div>
                </div>
            </div>

            <div
                class="flex-1 overflow-y-auto p-4 flex flex-col justify-end"
                node_ref=messages_container_ref
            >
                <div class="max-w-4xl mx-auto space-y-3 flex flex-col justify-end min-h-full">
                    <For
                        each=move || messages.get()
                        key=|msg| match msg {
                            WsMessage::UserJoined { username } => format!("join_{}", username),
                            WsMessage::UserLeft { username } => format!("leave_{}", username),
                            WsMessage::Message(m) => format!("msg_{}_{}", m.username, m.timestamp),
                        }
                        children=move |msg| {
                            match msg {
                                WsMessage::UserJoined { username } => {
                                    view! {
                                        <div class="text-center text-sm text-neutral-500 dark:text-neutral-400">
                                            <span class="font-medium">{username}</span> " joined the chat"
                                        </div>
                                    }.into_any()
                                },
                                WsMessage::UserLeft { username } => {
                                    view! {
                                        <div class="text-center text-sm text-neutral-500 dark:text-neutral-400">
                                            <span class="font-medium">{username}</span> " left the chat"
                                        </div>
                                    }.into_any()
                                },
                                WsMessage::Message(chat_msg) => {
                                    let user_id = chat_msg.user_id.clone();
                                    let username = chat_msg.username.clone();
                                    let message = chat_msg.message.clone();
                                    let timestamp = chat_msg.timestamp.clone();
                                    let fallback_username = username.clone();

                                    let user_resource = Resource::new(
                                        move || user_id.clone(),
                                        |user_id| get_user_info(user_id)
                                    );

                                    view! {
                                        <div class="p-1 animate-in slide-in-from-bottom-2 duration-300">
                                            <div class="flex items-start space-x-3">
                                                <Suspense fallback=move || {
                                                    view! {
                                                        <div class="w-6 h-6 bg-neutral-300 rounded-full animate-pulse"></div>
                                                    }
                                                }>
                                                    {move || {
                                                        match user_resource.get() {
                                                            Some(Ok(Some(user))) => {
                                                                view! {
                                                                    <UserAvatar
                                                                        name=Some(user.name.clone())
                                                                        image=user.image.clone()
                                                                        size="sm"
                                                                    />
                                                                }.into_any()
                                                            }
                                                            _ => {
                                                                view! {
                                                                    <UserAvatar
                                                                        name=Some(fallback_username.clone())
                                                                        image=None
                                                                        size="sm"
                                                                    />
                                                                }.into_any()
                                                            }
                                                        }
                                                    }}
                                                </Suspense>
                                                <div class="flex-1 min-w-0">
                                                    <div class="flex items-baseline">

                                                          <p class="text-neutral-700 dark:text-neutral-300 w-full">
                                                            <span class="font-semibold text-neutral-900 dark:text-neutral-100 mr-2 ">
                                                                {username.clone()}
                                                            </span>: {message.clone()}
                                                        </p>
                                                        <span class="text-xs text-neutral-500 dark:text-neutral-400">
                                                            {

                                                                let time_display = format_time_iso(timestamp.clone(), TimeFormatVariant::Ago);

                                                            view! {<Tooltip label=time_display.0>
                                                                <span class="whitespace-nowrap">{time_display.1}</span>
                                                            </Tooltip>}
                                        }

                                                        </span>
                                                    </div>

                                                </div>
                                            </div>
                                        </div>
                                    }.into_any()
                                }
                            }
                        }
                    />
                </div>
            </div>

            <div class="bg-white dark:bg-neutral-800 border-t border-neutral-200 dark:border-neutral-700 p-4">
                <div class="max-w-4xl mx-auto">
                    <form on:submit=send_message class="flex gap-2">
                        <input
                            type="text"
                            class="flex-1 px-4 py-2 border border-neutral-300 dark:border-neutral-600 rounded-lg bg-white dark:bg-neutral-900 text-neutral-900 dark:text-neutral-100 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400"
                            placeholder="Type a message..."
                            prop:value=move || input_value.get()
                            on:input=move |ev| set_input_value.set(event_target_value(&ev))
                            disabled=move || !connected.get()
                        />
                        <button
                            type="submit"
                            class="px-6 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-neutral-400 text-white font-medium rounded-lg transition-colors duration-200 disabled:cursor-not-allowed"
                            disabled=move || !connected.get()
                        >
                            "Send"
                        </button>
                    </form>
                </div>
            </div>
        </div>
    }
}
