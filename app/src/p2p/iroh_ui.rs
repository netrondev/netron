use leptos::prelude::*;

use crate::components::Input;

#[component]
pub fn IrohTest() -> impl IntoView {
    let username = RwSignal::new("unnamed_user".to_string());

    // Your component implementation here
    view! {
        <div>"Iroh test 1234"

               <Input
                    placeholder="Search for tokens..."
                    class="w-full"
                    value=username
                    on_input=Box::new(move |val| {
                        username.set(val);
                    })
                />

        </div>
    }
}
