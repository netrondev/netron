use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use phosphor_leptos::{SUN, SUN_DIM, SUN_HORIZON};

use crate::components::{
    button::{BtnVariant, Button, ButtonIcon},
    tooltip::{Align, Tooltip},
};

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::System => "system",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "dark" => Theme::Dark,
            "system" => Theme::System,
            _ => Theme::Light,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::System
    }
}

#[derive(Clone, Copy)]
pub struct ThemeContext {
    pub theme: RwSignal<Theme>,
    pub system_prefers_dark: RwSignal<bool>,
}

impl ThemeContext {
    pub fn new() -> Self {
        // Check localStorage on client side
        #[cfg(not(feature = "ssr"))]
        let initial_theme = {
            use web_sys::window;
            if let Some(storage) = window().and_then(|w| w.local_storage().ok()).flatten() {
                storage
                    .get_item("theme")
                    .ok()
                    .flatten()
                    .map(|s| Theme::from_str(s.as_str()))
                    .unwrap_or(Theme::System)
            } else {
                Theme::System
            }
        };

        #[cfg(feature = "ssr")]
        let initial_theme = Theme::System;

        let theme = RwSignal::new(initial_theme);
        let system_prefers_dark = RwSignal::new(false);

        // Detect system preference and listen for changes
        #[cfg(not(feature = "ssr"))]
        {
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::JsCast;
            use web_sys::{window, Event};

            if let Some(window) = window() {
                if let Ok(prefers_dark_media) = window.match_media("(prefers-color-scheme: dark)") {
                    if let Some(media_query) = prefers_dark_media {
                        system_prefers_dark.set(media_query.matches());

                        // Set up listener for system theme changes
                        let system_dark_signal = system_prefers_dark;
                        let media_query_clone = media_query.clone();
                        let callback = Closure::<dyn Fn(Event)>::new(move |_event: Event| {
                            system_dark_signal.set(media_query_clone.matches());
                        });

                        let _ = media_query.add_event_listener_with_callback(
                            "change",
                            callback.as_ref().unchecked_ref(),
                        );
                        callback.forget(); // Keep the closure alive
                    }
                }
            }
        }

        // Set up effect to save theme to localStorage
        #[cfg(not(feature = "ssr"))]
        {
            use web_sys::window;
            let theme_effect = theme;
            let system_dark = system_prefers_dark;
            Effect::new(move |_| {
                let current_theme = theme_effect.get();
                let prefers_dark = system_dark.get();

                if let Some(storage) = window().and_then(|w| w.local_storage().ok()).flatten() {
                    let _ = storage.set_item("theme", current_theme.as_str());
                }

                // Determine effective theme
                let effective_theme = match current_theme {
                    Theme::System => {
                        if prefers_dark {
                            Theme::Dark
                        } else {
                            Theme::Light
                        }
                    }
                    _ => current_theme,
                };

                // Update document class
                if let Some(doc) = window().and_then(|w| w.document()) {
                    if let Some(doc_element) = doc.document_element() {
                        let class_list = doc_element.class_list();
                        match effective_theme {
                            Theme::Dark => {
                                let _ = class_list.add_1("dark");
                                let _ = class_list.remove_1("light");
                            }
                            Theme::Light | Theme::System => {
                                let _ = class_list.remove_1("dark");
                                let _ = class_list.add_1("light");
                            }
                        }
                    }
                }
            });
        }

        ThemeContext {
            theme,
            system_prefers_dark,
        }
    }

    pub fn toggle(&self) {
        self.theme.update(|t| {
            *t = match *t {
                Theme::Light => {
                    if self.system_prefers_dark.get() {
                        Theme::System
                    } else {
                        Theme::Dark
                    }
                }
                Theme::Dark => {
                    if self.system_prefers_dark.get() {
                        Theme::Light
                    } else {
                        Theme::System
                    }
                }
                Theme::System => {
                    if self.system_prefers_dark.get() {
                        Theme::Light
                    } else {
                        Theme::Dark
                    }
                }
            }
        });
    }

    pub fn effective_theme(&self) -> Theme {
        match self.theme.get() {
            Theme::System => {
                if self.system_prefers_dark.get() {
                    Theme::Dark
                } else {
                    Theme::Light
                }
            }
            theme => theme,
        }
    }
}

#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let theme_context = ThemeContext::new();

    provide_context(theme_context);

    children()
}

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let theme_context =
        use_context::<ThemeContext>().expect("ThemeToggle must be used within a ThemeProvider");

    let toggle_theme = move |_| {
        theme_context.toggle();
    };

    let current_theme = move || theme_context.theme.get();

    view! {
        {move || match current_theme() {
            Theme::Light => {
                view! {
                    <Tooltip
                        label="Light Mode".to_string()
                        align=Align::Left
                    >
                        <Button
                            on:click=toggle_theme
                            icon=ButtonIcon::Icon(SUN)
                            variant=BtnVariant::Square
                        /> </Tooltip>
                }.into_any()
            }
            Theme::Dark => {
                view! {
                    <Tooltip
                        label="Dark Mode".to_string()
                        align=Align::Left
                    >
                         <Button
                            on:click=toggle_theme
                            icon=ButtonIcon::Icon(SUN_DIM)
                            variant=BtnVariant::Square
                        /></Tooltip>
                }.into_any()
            }
            Theme::System => {
                view! {
                    <Tooltip
                        label="Use system".to_string()
                        align=Align::Left
                    >
                           <Button
                            on:click=toggle_theme
                            icon=ButtonIcon::Icon(SUN_HORIZON)
                            variant=BtnVariant::Square
                        /></Tooltip>
                }.into_any()
            }
        }}
    }
}
