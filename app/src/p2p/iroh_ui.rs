use leptos::prelude::*;
use phosphor_leptos::ARROW_RIGHT;

use crate::components::{
    button::{BtnVariant, ButtonIcon},
    label::Label,
    qrcode::QRCode,
    Button, Input,
};

#[server]
async fn create_ticket_sf(user: String) -> Result<String, ServerFnError> {
    let result = crate::p2p::iroh::iroh_create(user).await?;
    Ok(result)
}

#[component]
pub fn IrohTest() -> impl IntoView {
    let username = RwSignal::new("unnamed_user".to_string());
    let ticket: RwSignal<Option<String>> = RwSignal::new(None);
    let create_ticket = Action::new(move |user: &String| {
        let u = user.clone();
        async move {
            let output = create_ticket_sf(u).await;

            match &output {
                Ok(t) => {
                    ticket.set(Some(t.clone()));
                }
                Err(e) => ticket.set(None),
            }

            output
        }
    });

    // Your component implementation here
    view! {
        <div class="p-4">

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

            <Button
                icon=ButtonIcon::Icon(ARROW_RIGHT)
                variant=BtnVariant::Default
                on:click=move |_| {
                    let user = username.get();
                    create_ticket.dispatch(user);
                }
            >
                "Create Ticket"
            </Button>

            <div>
                {move || {

                    match ticket.get() {
                        Some(t) =>

                            view! {
                                <div>
                                    <QRCode input={t.clone()} />
                                    <div class="w-[300px] overflow-x-scroll">{t}</div>
                                </div>
                            }.into_any(),


                        None => view! { <div>"No ticket yet"</div> }.into_any()
                    }

                }
                }
            </div>

        </div>
    }
}
