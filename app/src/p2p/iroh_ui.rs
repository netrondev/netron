use leptos::prelude::*;
use phosphor_leptos::{Icon, ARROW_RIGHT, PAPER_PLANE, USERS};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn current_timestamp() -> u64 {
    #[cfg(feature = "hydrate")]
    {
        web_sys::js_sys::Date::now() as u64
    }
    #[cfg(not(feature = "hydrate"))]
    {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }
}

use crate::{
    components::{
        button::{BtnVariant, ButtonIcon},
        label::Label,
        qrcode::QRCode,
        Button, Input,
    },
    p2p::iroh::ChatTicket,
};

#[cfg(feature = "hydrate")]
fn start_receiver_consumer(
    stream: wasm_streams::readable::sys::ReadableStream,
    active_chat: RwSignal<Option<ActiveChat>>,
) {
    use wasm_streams::ReadableStream;

    wasm_bindgen_futures::spawn_local(async move {
        // Convert the sys ReadableStream to the wasm_streams ReadableStream
        let mut readable_stream = ReadableStream::from_raw(stream);
        let mut reader = readable_stream.get_reader();

        loop {
            // Read from the stream
            match reader.read().await {
                Ok(Some(chunk)) => {
                    // Parse the event from the JS value
                    if let Ok(event) =
                        serde_wasm_bindgen::from_value::<crate::p2p::iroh::Event>(chunk)
                    {
                        handle_received_event(event, active_chat);
                    }
                }
                Ok(None) => {
                    web_sys::console::log_1(&"Stream ended".into());
                    break;
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("Stream error: {:?}", e).into());
                    break;
                }
            }
        }
    });
}

#[cfg(feature = "hydrate")]
fn handle_received_event(
    event: crate::p2p::iroh::Event,
    active_chat: RwSignal<Option<ActiveChat>>,
) {
    use crate::p2p::iroh::Event;

    match event {
        Event::MessageReceived {
            from,
            text,
            nickname,
            sent_timestamp,
        } => {
            web_sys::console::log_1(
                &format!("Received message from {}: {}", nickname, text).into(),
            );

            let new_message = ChatMessage {
                from: from.to_string(),
                nickname,
                text,
                timestamp: sent_timestamp,
                is_own: false,
            };

            // Add the message to the active chat
            active_chat.update(|chat_opt| {
                if let Some(ref mut chat) = chat_opt {
                    chat.messages.push(new_message);
                }
            });
        }
        Event::Presence {
            from,
            nickname,
            sent_timestamp: _,
        } => {
            web_sys::console::log_1(&format!("Presence update: {} is online", nickname).into());

            // Update online users
            active_chat.update(|chat_opt| {
                if let Some(ref mut chat) = chat_opt {
                    chat.online_users.insert(from.to_string(), nickname);
                }
            });
        }
        Event::Joined { neighbors } => {
            web_sys::console::log_1(
                &format!("Joined gossip network with {} neighbors", neighbors.len()).into(),
            );
        }
        Event::NeighborUp { node_id } => {
            web_sys::console::log_1(&format!("Neighbor connected: {}", node_id).into());
        }
        Event::NeighborDown { node_id } => {
            web_sys::console::log_1(&format!("Neighbor disconnected: {}", node_id).into());
        }
        Event::Lagged => {
            web_sys::console::warn_1(&"Stream lagged - some messages may have been missed".into());
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMessage {
    from: String,
    nickname: String,
    text: String,
    timestamp: u64,
    is_own: bool,
}

#[derive(Debug, Clone)]
struct ActiveChat {
    messages: Vec<ChatMessage>,
    online_users: HashMap<String, String>,
    topic_id: String,
    #[cfg(feature = "hydrate")]
    sender: Option<crate::p2p::wasm_chat::ChannelSender>,
}

#[component]
pub fn IrohTest() -> impl IntoView {
    let username = RwSignal::new("unnamed_user".to_string());
    let ticket: RwSignal<Option<String>> = RwSignal::new(None);
    let join_ticket = RwSignal::new(String::new());
    let active_chat: RwSignal<Option<ActiveChat>> = RwSignal::new(None);
    let message_input = RwSignal::new(String::new());
    let status = RwSignal::new("P2P Chat - Click to initialize node".to_string());
    let node_ready = RwSignal::new(false);

    #[cfg(feature = "hydrate")]
    let chat_node: RwSignal<Option<crate::p2p::wasm_chat::ChatNode>> = RwSignal::new(None);

    let initialize_node = move |_| {
        #[cfg(feature = "hydrate")]
        {
            status.set("Initializing P2P node...".to_string());
            wasm_bindgen_futures::spawn_local(async move {
                match crate::p2p::wasm_chat::ChatNode::spawn().await {
                    Ok(node) => {
                        let node_id = node.node_id();
                        status.set(format!("Node ready! ID: {:.8}...", node_id));
                        chat_node.set(Some(node));
                        node_ready.set(true);
                    }
                    Err(e) => {
                        status.set(format!("Failed to start P2P node: {:?}", e));
                    }
                }
            });
        }
        #[cfg(not(feature = "hydrate"))]
        {
            status.set("P2P chat only works in browser mode".to_string());
        }
    };

    let create_chat = move |_| {
        if !node_ready.get() {
            status.set("Please initialize the node first".to_string());
            return;
        }

        #[cfg(feature = "hydrate")]
        {
            chat_node.with(|node_opt| {
                if let Some(node) = node_opt {
                    let username_val = username.get();
                    let node_clone = node.clone();
                    status.set("Creating chat room...".to_string());
                    wasm_bindgen_futures::spawn_local(async move {
                        match node_clone.create(username_val).await {
                            Ok(mut channel) => {
                                let sender = channel.sender();
                                let topic_id = channel.id();
                                let opts = web_sys::js_sys::Object::new();
                                let ticket_str = channel.ticket(opts.into()).unwrap();

                                ticket.set(Some(ticket_str));

                                let chat = ActiveChat {
                                    messages: vec![ChatMessage {
                                        from: "system".to_string(),
                                        nickname: "System".to_string(),
                                        text:
                                            "Chat room created! Others can now join using the ticket."
                                                .to_string(),
                                        timestamp: current_timestamp(),
                                        is_own: false,
                                    }],
                                    online_users: HashMap::new(),
                                    topic_id: format!("{:.8}", topic_id),
                                    sender: Some(sender),
                                };
                                active_chat.set(Some(chat));
                                // Start consuming the receiver stream
                                let receiver = channel.receiver();
                                start_receiver_consumer(receiver, active_chat);
                                status.set("Chat room created successfully!".to_string());
                            }
                            Err(e) => {
                                status.set(format!("Failed to create chat: {:?}", e));
                            }
                        }
                    });
                } else {
                    status.set("Node not initialized yet".to_string());
                }
            });
        }
        #[cfg(not(feature = "hydrate"))]
        {
            status.set("P2P chat only works in browser mode".to_string());
        }
    };

    let join_chat = move |_| {
        let ticket_str = join_ticket.get();
        if ticket_str.is_empty() {
            status.set("Please enter a ticket to join".to_string());
            return;
        }

        if !node_ready.get() {
            status.set("Please initialize the node first".to_string());
            return;
        }

        #[cfg(feature = "hydrate")]
        {
            chat_node.with(|node_opt| {
                if let Some(node) = node_opt {
                    let username_val = username.get();
                    let node_clone = node.clone();
                    status.set("Joining chat room...".to_string());
                    wasm_bindgen_futures::spawn_local(async move {
                        match node_clone.join(ticket_str, username_val.clone()).await {
                            Ok(mut channel) => {
                                let sender = channel.sender();
                                let topic_id = channel.id();

                                let chat = ActiveChat {
                                    messages: vec![ChatMessage {
                                        from: "system".to_string(),
                                        nickname: "System".to_string(),
                                        text: format!(
                                            "Joined chat room! Welcome, {}.",
                                            username_val
                                        ),
                                        timestamp: current_timestamp(),
                                        is_own: false,
                                    }],
                                    online_users: HashMap::new(),
                                    topic_id: format!("{:.8}", topic_id),
                                    sender: Some(sender),
                                };
                                active_chat.set(Some(chat));

                                // Start consuming the receiver stream
                                let receiver = channel.receiver();
                                start_receiver_consumer(receiver, active_chat);

                                status.set("Successfully joined chat room!".to_string());
                            }
                            Err(e) => {
                                status.set(format!("Failed to join chat: {:?}", e));
                            }
                        }
                    });
                } else {
                    status.set("Node not initialized yet".to_string());
                }
            });
        }
        #[cfg(not(feature = "hydrate"))]
        {
            status.set("P2P chat only works in browser mode".to_string());
        }
    };

    let send_message = move |_| {
        let message = message_input.get().trim().to_string();
        if message.is_empty() {
            return;
        }

        if let Some(chat) = active_chat.get() {
            #[cfg(feature = "hydrate")]
            {
                if let Some(ref sender) = chat.sender {
                    let sender_clone = sender.clone();
                    let current_username = username.get();
                    let message_clone = message.clone();

                    // Add our own message to the UI immediately
                    let own_message = ChatMessage {
                        from: "self".to_string(),
                        nickname: current_username,
                        text: message_clone.clone(),
                        timestamp: current_timestamp(),
                        is_own: true,
                    };

                    let mut updated_chat = chat.clone();
                    updated_chat.messages.push(own_message);
                    active_chat.set(Some(updated_chat));
                    message_input.set(String::new());

                    // Send via P2P
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Err(e) = sender_clone.broadcast(message_clone).await {
                            web_sys::console::log_1(
                                &format!("Failed to send message: {:?}", e).into(),
                            );
                        }
                    });
                }
            }
            #[cfg(not(feature = "hydrate"))]
            {
                // Fallback for SSR
                let new_message = ChatMessage {
                    from: "self".to_string(),
                    nickname: username.get(),
                    text: message,
                    timestamp: current_timestamp(),
                    is_own: true,
                };

                let mut updated_chat = chat.clone();
                updated_chat.messages.push(new_message);
                active_chat.set(Some(updated_chat));
                message_input.set(String::new());
            }
        }
    };

    view! {
        <div class="p-4 max-w-4xl mx-auto">
            <div class="mb-4">
                <div class="text-sm text-gray-600 mb-2">{move || status.get()}</div>
                <div class="text-xs text-green-600">
                    "Real P2P chat using iroh network - messages are shared between all connected browsers!"
                </div>

                {move || {
                    if !node_ready.get() && active_chat.get().is_none() {
                        view! {
                            <Button
                                variant=BtnVariant::CallToAction
                                on:click=initialize_node
                                class="mt-2"
                            >
                                "Initialize P2P Node"
                            </Button>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }}
            </div>

            {move || {
                if active_chat.get().is_some() {
                    view! {
                        <div class="space-y-4">
                            <div class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg p-4">
                                <div class="flex items-center justify-between mb-4">
                                    <h3 class="text-lg font-semibold flex items-center gap-2">
                                        <Icon icon=USERS size="20px" />
                                        "Chat Room"
                                        {move || {
                                            if let Some(chat) = active_chat.get() {
                                                format!(" - {}", chat.topic_id)
                                            } else {
                                                String::new()
                                            }
                                        }}
                                    </h3>
                                    <Button
                                        variant=BtnVariant::Default
                                        on:click=move |_| {
                                            active_chat.set(None);
                                            ticket.set(None);
                                            // Keep the node ready and available for reuse
                                        }
                                    >
                                        "Leave Chat"
                                    </Button>
                                </div>

                                // Show ticket for sharing when available
                                {move || {
                                    if let Some(t) = ticket.get() {
                                        view! {
                                            <div class="mb-4 p-3 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg">
                                                <div class="text-sm font-medium text-blue-900 dark:text-blue-100 mb-2">"Share this ticket to invite others:"</div>
                                                <div class="flex items-start gap-3">
                                                    <div class="shrink-0">
                                                        <div class="w-20 h-20 p-2 bg-white rounded border">
                                                            <QRCode input={t.clone()} />
                                                        </div>
                                                    </div>
                                                    <div class="flex-1">
                                                        <div class="p-2 bg-white dark:bg-gray-800 border rounded text-xs break-all font-mono">{t}</div>
                                                    </div>
                                                </div>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! { <div></div> }.into_any()
                                    }
                                }}

                                <div class="h-64 bg-gray-50 dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded p-4 overflow-y-auto mb-4">
                                    <For
                                        each=move || {
                                            if let Some(chat) = active_chat.get() {
                                                chat.messages
                                            } else {
                                                vec![]
                                            }
                                        }
                                        key=|msg| (msg.timestamp, msg.text.clone())
                                        children=move |msg| {
                                            let msg_class = if msg.is_own {
                                                "mb-2 text-right"
                                            } else {
                                                "mb-2"
                                            };
                                            let bubble_class = if msg.is_own {
                                                "inline-block bg-blue-500 text-white px-3 py-2 rounded-lg max-w-xs text-left"
                                            } else {
                                                "inline-block bg-gray-200 dark:bg-gray-700 px-3 py-2 rounded-lg max-w-xs"
                                            };
                                            view! {
                                                <div class={msg_class}>
                                                    <div class={bubble_class}>
                                                        <div class="text-xs text-gray-500 dark:text-gray-400 mb-1">{msg.nickname}</div>
                                                        <div>{msg.text}</div>
                                                    </div>
                                                </div>
                                            }
                                        }
                                    />
                                </div>

                                <div class="flex gap-2">
                                    <Input
                                        placeholder="Type a message..."
                                        class="flex-1"
                                        value=message_input
                                        on_input=Box::new(move |val| { message_input.set(val); })
                                    />
                                    <Button
                                        icon=ButtonIcon::Icon(PAPER_PLANE)
                                        variant=BtnVariant::Default
                                        on:click=send_message
                                    >
                                        "Send"
                                    </Button>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="space-y-6">
                            <Label title="Username">
                                <Input
                                    placeholder="Enter your username"
                                    class="w-full"
                                    value=username
                                    on_input=Box::new(move |val| { username.set(val); })
                                />
                            </Label>

                            <div class="grid md:grid-cols-2 gap-6">
                                <div class="space-y-4">
                                    <h3 class="text-lg font-semibold">"Create New Chat"</h3>
                                    <Button
                                        icon=ButtonIcon::Icon(ARROW_RIGHT)
                                        variant=BtnVariant::Default
                                        on:click=create_chat
                                        class="w-full"
                                        // disabled=Signal::derive(move || !node_ready.get())
                                    >
                                        "Create Chat Room"
                                    </Button>

                                    {move || {
                                        if let Some(t) = ticket.get() {
                                            view! {
                                                <div class="space-y-2">
                                                    <div class="text-sm font-medium">"Share this ticket to invite others:"</div>
                                                    <QRCode input={t.clone()} />
                                                    <div class="p-2 bg-gray-100 dark:bg-gray-800 rounded text-xs break-all">{t}</div>
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <div></div> }.into_any()
                                        }
                                    }}
                                </div>

                                <div class="space-y-4">
                                    <h3 class="text-lg font-semibold">"Join Existing Chat"</h3>
                                    <Label title="Chat Ticket">
                                        <Input
                                            placeholder="Paste chat ticket here..."
                                            class="w-full"
                                            value=join_ticket
                                            on_input=Box::new(move |val| { join_ticket.set(val); })
                                        />
                                    </Label>
                                    <Button
                                        icon=ButtonIcon::Icon(ARROW_RIGHT)
                                        variant=BtnVariant::Default
                                        on:click=join_chat
                                        class="w-full"
                                        // disabled=Signal::derive(move || !node_ready.get())
                                    >
                                        "Join Chat Room"
                                    </Button>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}
