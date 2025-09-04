use leptos::prelude::*;
use phosphor_leptos::ARROW_RIGHT;

use crate::{
    components::{
        button::{BtnVariant, ButtonIcon},
        label::Label,
        qrcode::QRCode,
        Button, Input,
    },
    p2p::iroh::ChatTicket,
};

#[component]
pub fn IrohTest() -> impl IntoView {
    let username = RwSignal::new("unnamed_user".to_string());
    let ticket: RwSignal<Option<String>> = RwSignal::new(None);

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
                    let ticketcreated = ChatTicket::new_random();
                    let output = ticketcreated.serialize();
                    ticket.set(Some(output.clone()));
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
