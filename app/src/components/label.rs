use leptos::prelude::*;

#[component]
pub fn Label(
    children: Children,
    #[prop(into)] title: String,
    #[prop(optional)] required: bool,
) -> impl IntoView {
    let default_class = "block text-sm font-medium text-neutral-700 dark:text-neutral-500";

    view! {
        <div class="flex flex-col" >
            <label class=default_class>{title}</label>
            {children()}
            {if required {
                view! { <span class="text-red-500">" *"</span> }.into_any()
            } else {
                view! {}.into_any()
            }}
        </div>
    }
}
