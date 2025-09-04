#[cfg(feature = "hydrate")]
use std::{collections::BTreeSet, sync::{Arc, Mutex}};

#[cfg(feature = "hydrate")]
use anyhow::Result;
#[cfg(feature = "hydrate")]
use crate::p2p::iroh::{ChatTicket, NodeId, TopicId};
#[cfg(feature = "hydrate")]
// use n0_future::StreamExt; // Not needed
#[cfg(feature = "hydrate")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "hydrate")]
use wasm_bindgen::{prelude::wasm_bindgen, JsError, JsValue};
#[cfg(feature = "hydrate")]
// use wasm_bindgen_futures::spawn_local; // Not needed here

#[cfg(feature = "hydrate")]
#[wasm_bindgen]
pub struct ChatNode(crate::p2p::iroh::ChatNode);

#[cfg(feature = "hydrate")]
#[wasm_bindgen]
impl ChatNode {
    pub async fn spawn() -> Result<Self, JsError> {
        let inner = crate::p2p::iroh::ChatNode::spawn(None)
            .await
            .map_err(to_js_err)?;
        Ok(Self(inner))
    }

    pub fn node_id(&self) -> String {
        self.0.node_id().to_string()
    }

    pub async fn create(&self, nickname: String) -> Result<Channel, JsError> {
        let ticket = ChatTicket::new_random();
        self.join_inner(ticket, nickname).await
    }

    pub async fn join(&self, ticket: String, nickname: String) -> Result<Channel, JsError> {
        let ticket = ChatTicket::deserialize(&ticket).map_err(to_js_err)?;
        self.join_inner(ticket, nickname).await
    }

    async fn join_inner(&self, ticket: ChatTicket, nickname: String) -> Result<Channel, JsError> {
        let (sender, _receiver) = self.0.join(&ticket, nickname).await.map_err(to_js_err)?;
        let sender = ChannelSender(sender);
        let neighbors = Arc::new(Mutex::new(BTreeSet::new()));
        
        // For now, we'll create a simple channel without the complex stream handling
        // This will be a simplified version that focuses on basic messaging
        let mut ticket = ticket;
        ticket.bootstrap.insert(self.0.node_id());

        let channel = Channel {
            topic_id: ticket.topic_id,
            bootstrap: ticket.bootstrap,
            neighbors,
            me: self.0.node_id(),
            sender,
        };
        Ok(channel)
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen]
pub struct Channel {
    topic_id: TopicId,
    me: NodeId,
    bootstrap: BTreeSet<NodeId>,
    neighbors: Arc<Mutex<BTreeSet<NodeId>>>,
    sender: ChannelSender,
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen]
impl Channel {
    #[wasm_bindgen(getter)]
    pub fn sender(&self) -> ChannelSender {
        self.sender.clone()
    }

    pub fn ticket(&self, _opts: JsValue) -> Result<String, JsError> {
        // For now, just use default options since we don't have serde_wasm_bindgen
        let opts = TicketOpts {
            include_myself: true,
            include_bootstrap: true,
            include_neighbors: false,
        };
        let mut ticket = ChatTicket::new(self.topic_id);
        if opts.include_myself {
            ticket.bootstrap.insert(self.me);
        }
        if opts.include_bootstrap {
            ticket.bootstrap.extend(self.bootstrap.iter().copied());
        }
        if opts.include_neighbors {
            let neighbors = self.neighbors.lock().unwrap();
            ticket.bootstrap.extend(neighbors.iter().copied())
        }
        Ok(ticket.serialize())
    }

    pub fn id(&self) -> String {
        self.topic_id.to_string()
    }

    pub fn neighbors(&self) -> Vec<String> {
        self.neighbors
            .lock()
            .unwrap()
            .iter()
            .map(|x| x.to_string())
            .collect()
    }
}

#[cfg(feature = "hydrate")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketOpts {
    pub include_myself: bool,
    pub include_bootstrap: bool,
    pub include_neighbors: bool,
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct ChannelSender(crate::p2p::iroh::ChatSender);

#[cfg(feature = "hydrate")]
#[wasm_bindgen]
impl ChannelSender {
    pub async fn broadcast(&self, text: String) -> Result<(), JsError> {
        self.0.send(text).await.map_err(to_js_err)?;
        Ok(())
    }

    pub fn set_nickname(&self, nickname: String) {
        self.0.set_nickname(nickname);
    }
}

#[cfg(feature = "hydrate")]
fn to_js_err(err: impl Into<anyhow::Error>) -> JsError {
    let err: anyhow::Error = err.into();
    JsError::new(&err.to_string())
}