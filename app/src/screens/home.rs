use leptos::prelude::*;

#[component]
pub fn HomeScreen() -> impl IntoView {
    view! {
        <div class="screen home-screen">
            <h1>"Welcome to Netron"</h1>
            <p>"A decentralized communication platform"</p>

            <div class="features">
                <div class="feature-card">
                    <h3>"Secure Messaging"</h3>
                    <p>"End-to-end encrypted communication"</p>
                </div>
                <div class="feature-card">
                    <h3>"Decentralized"</h3>
                    <p>"No central server, peer-to-peer connections"</p>
                </div>
                <div class="feature-card">
                    <h3>"Open Source"</h3>
                    <p>"Transparent and community-driven"</p>
                </div>
            </div>
        </div>
    }
}
