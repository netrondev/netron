use crate::{
    colors::Color,
    components::{
        button::ButtonIcon,
        sidebar::{NavBarLink, SideBar, SidebarItem},
    },
    navbar::Navbar,
    theme::ThemeProvider,
};
use backend::*;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet};
use leptos_router::components::Router;
use phosphor_leptos::{CHART_LINE_UP, CURRENCY_ETH, GEAR, WALLET};
pub mod apperror;
pub mod auth;
pub mod colors;
pub mod components;
pub mod email;
pub mod navbar;
pub mod theme;
pub use apperror::AppError;
pub mod db;
pub use db::db_init;

#[component]
pub fn App() -> impl IntoView {
    let links = vec![
        SidebarItem::Link(NavBarLink {
            name: "Trading".to_string(),
            icon: ButtonIcon::Icon(CHART_LINE_UP),
            icon_hover: None,
            background_color: Color::from_tailwind("amber-600"),
            url: "/trading".to_string(),
        }),
        SidebarItem::Link(NavBarLink {
            name: "EVM".to_string(),
            icon: ButtonIcon::Icon(CURRENCY_ETH),
            icon_hover: None,
            background_color: Color::from_tailwind("cyan-600"),
            url: "/evm".to_string(),
        }),
        SidebarItem::Link(NavBarLink {
            name: "Wallets".to_string(),
            icon: ButtonIcon::Icon(WALLET),
            icon_hover: None,
            background_color: Color::from_tailwind("purple-600"),
            url: "/wallets".to_string(),
        }),
        SidebarItem::Gap,
        SidebarItem::Divider,
        SidebarItem::Link(NavBarLink {
            name: "Settings".to_string(),
            icon: ButtonIcon::Icon(GEAR),
            icon_hover: None,
            background_color: Color::from_tailwind("teal-600"),
            url: "/settings".to_string(),
        }),
    ];

    provide_meta_context();
    let action = ServerAction::<HelloWorldServer>::new();
    let vals = RwSignal::new(String::new());
    Effect::new(move |_| {
        if let Some(resp) = action.value().get() {
            match resp {
                Ok(val) => vals.set(val),
                Err(err) => vals.set(format!("{err:?}")),
            }
        }
    });
    view! {
        <Stylesheet id={"leptos"} href={"/pkg/netron.css"} />
        <ThemeProvider>
            <Router>
                    <main class="bg-neutral-100 dark:bg-neutral-900 flex flex-col transition-colors text-neutral-700 dark:text-neutral-300">
                        <section class="flex flex-row w-full gap-2">
                            <SideBar links=links />
                            <div class="h-screen flex flex-col flex-1 overflow-y-auto gap-2">
                                <div class="pl-12 md:pl-0">
                                    <Navbar />
                                </div>
                                <div class="flex-1 flex flex-row min-h-0 w-full">
                                    <div class="flex flex-col w-full">
                                        <span>"Router pages here"</span>
                                    </div>
                                </div>
                            </div>
                        </section>
                    </main>
            </Router>
        </ThemeProvider>
    }
}
