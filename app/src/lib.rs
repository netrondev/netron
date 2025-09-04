pub mod p2p;
use crate::{
    components::{
        button::ButtonIcon,
        sidebar::{NavBarLink, SideBar, SidebarItem},
    },
    navbar::Navbar,
    screens::HomeScreen,
    theme::ThemeProvider,
};
use backend::*;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use phosphor_leptos::{CUBE, GEAR, PLANET, SHARE_NETWORK};
pub mod apperror;
pub mod auth;
pub mod chat;
pub mod colors;
pub mod components;
pub mod date_utils;
pub mod navbar;
pub mod theme;
pub use apperror::AppError;
pub mod db;
pub mod screens;

pub mod surrealtypes;
#[cfg(feature = "ssr")]
pub use db::db_init;

#[cfg(feature = "ssr")]
pub use surrealdb::{Datetime, RecordId};

#[cfg(not(feature = "ssr"))]
pub use crate::surrealtypes::{Datetime, RecordId};

pub const LOGO: &str = "
 _____ _____ _____ _____ _____ _____ 
|   | |   __|_   _| __  |     |   | |
| | | |   __| | | |    -|  |  | | | |
|_|___|_____| |_| |__|__|_____|_|___|
";

#[component]
pub fn App() -> impl IntoView {
    let links = vec![
        SidebarItem::Link(NavBarLink {
            name: "Dashboard".to_string(),
            icon: ButtonIcon::Icon(CUBE),
            icon_hover: None,
            url: "/".to_string(),
        }),
        SidebarItem::Link(NavBarLink {
            name: "Global".to_string(),
            icon: ButtonIcon::Icon(PLANET),
            icon_hover: None,
            url: "/global".to_string(),
        }),
        SidebarItem::Link(NavBarLink {
            name: "iroh".to_string(),
            icon: ButtonIcon::Icon(SHARE_NETWORK),
            icon_hover: None,
            url: "/iroh".to_string(),
        }),
        SidebarItem::Gap,
        SidebarItem::Divider,
        SidebarItem::Link(NavBarLink {
            name: "Settings".to_string(),
            icon: ButtonIcon::Icon(GEAR),
            icon_hover: None,
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

                                         <Routes fallback=|| "Page not found.".into_view()>
                                            <Route path=path!("/") view=HomeScreen />
                                            <Route path=path!("/iroh") view=p2p::iroh_ui::IrohTest />
                                        </Routes>
                                    </div>
                                </div>
                            </div>
                        </section>
                    </main>
            </Router>
        </ThemeProvider>
    }
}
