use crate::auth::session::get_session;
use leptos::prelude::*;
use phosphor_leptos::{Icon, CIRCLE_NOTCH};

#[component]
pub fn LoadingIndicator() -> impl IntoView {
    view! {
        <div class="relative">
            <div class="animate-spin text-blue-500 w-18 h-18 absolute left-8 top-8 flex items-center justify-center rounded-full">
                <Icon icon=CIRCLE_NOTCH color="#666" />
            </div>
        </div>
    }
}

#[component]
pub fn AuthCheck<F, IV>(unauthed: F, children: ChildrenFn) -> impl IntoView
where
    F: Fn() -> IV + Send + Sync + 'static,
    IV: IntoView + 'static,
{
    let user_resource = Resource::new(|| (), |_| get_session());

    view! {
        <Suspense fallback=|| {
            view! { <LoadingIndicator /> }
        }>
            {move || {
                match user_resource.get() {
                    Some(Ok(_)) => children().into_any(),
                    Some(Err(_)) => unauthed().into_any(),
                    None => view! { <LoadingIndicator /> }.into_any(),
                }
            }}
        </Suspense>
    }
}
