use leptos::prelude::*;

#[component]
pub fn QRCode(input: String) -> impl IntoView {
    let encoded = urlencoding::encode(&input);
    let src = format!(
        "https://api.qrserver.com/v1/create-qr-code/?data={}&size=600x600",
        encoded
    );

    view! {
        <img src=src alt="QR Code" />
    }
}
