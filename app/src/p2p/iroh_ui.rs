use leptos::prelude::*;

use crate::components::{label::Label, Input};

#[component]
pub fn IrohTest() -> impl IntoView {
    let username = RwSignal::new("unnamed_user".to_string());

    // Your component implementation here
    view! {
        <div>

            <Label title="Username">
               <Input
                    placeholder="Search for tokens..."
                    class="w-full"
                    value=username
                    on_input=Box::new(move |val| {
                        username.set(val);
                    })
                />
            </Label>

        </div>
    }
}
