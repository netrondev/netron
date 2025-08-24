use crate::{
    components::button::{BtnState, BtnVariant, Button, ButtonIcon},
    theme::ThemeToggle,
};
use leptos::prelude::*;
use phosphor_leptos::{LIST, X};

#[derive(Clone, Debug)]
pub struct NavItem {
    pub label: &'static str,
    pub href: String,
}

#[component]
pub fn AppHeader(title: &'static str, nav_items: Vec<NavItem>) -> impl IntoView {
    let location = leptos_router::hooks::use_location();
    let pathname = move || location.pathname.get();
    let (is_mobile_menu_open, set_mobile_menu_open) = signal(false);
    let nav_items = StoredValue::new(nav_items);

    // Find the current active nav item
    let active_nav_label = move || {
        nav_items
            .get_value()
            .into_iter()
            .find(|item| pathname() == item.href)
            .map(|item| item.label)
            .unwrap_or("")
    };

    view! {
        <header class="">
            <div class="px-6">
                <div class="flex items-center justify-between h-16">
                    <div class="flex items-center space-x-3">
                        // <NavigationBackButton />
                        <h1 class="text-2xl font-bold text-neutral-900 dark:text-neutral-100">{title}</h1>
                    </div>

                    // Desktop navigation
                    <nav class="hidden md:flex items-center space-x-4">
                        <For
                            each=move || nav_items.get_value()
                            key=|item| item.label
                            children=move |item| {
                                let href = item.href.clone();
                                let is_active = move || pathname() == href;

                                view! {
                                    <Button
                                        variant=BtnVariant::CallToAction
                                        state=if is_active() { BtnState::Active } else { BtnState::Default }
                                        on_click=Callback::new(move |_| {
                                            window().location().set_href(&item.href).unwrap();
                                        })
                                    >
                                        {item.label}
                                    </Button>
                                }
                            }
                        />
                    </nav>

                    // Mobile navigation area
                    <div class="md:hidden flex items-center space-x-2">
                        // Current active page label
                        <span class="text-sm font-medium text-neutral-700 dark:text-neutral-300">
                            {move || active_nav_label()}
                        </span>

                        // Mobile menu button
                        {move || {
                            if is_mobile_menu_open.get() {
                                view! {
                                    <Button
                                        variant=BtnVariant::Square
                                        icon=ButtonIcon::Icon(&X)
                                        on_click=Callback::new(move |_| set_mobile_menu_open.update(|v| *v = !*v))
                                        class="p-2"
                                    />
                                }
                            } else {
                                view! {
                                    <Button
                                        variant=BtnVariant::Square
                                        icon=ButtonIcon::Icon(&LIST)
                                        on_click=Callback::new(move |_| set_mobile_menu_open.update(|v| *v = !*v))
                                        class="p-2"
                                    />
                                }
                            }
                        }}
                    </div>
                </div>

                // Mobile menu dropdown
                <Show when=move || is_mobile_menu_open.get()>
                    <nav class="md:hidden pb-4">
                        <div class="flex flex-col space-y-1">
                            <For
                                each=move || nav_items.get_value()
                                key=|item| item.label
                                children=move |item| {
                                    let href = item.href.clone();
                                    let is_active = move || pathname() == href;

                                    view! {
                                        <Button
                                            variant=BtnVariant::Default
                                            state=if is_active() { BtnState::Active } else { BtnState::Default }
                                            on_click=Callback::new(move |_| {
                                                set_mobile_menu_open.set(false);
                                                window().location().set_href(&item.href).unwrap();
                                            })
                                            class="w-full text-left"
                                        >
                                            {item.label}
                                        </Button>
                                    }
                                }
                            />

                            <ThemeToggle />

                        </div>
                    </nav>
                </Show>
            </div>
        </header>
    }
}
