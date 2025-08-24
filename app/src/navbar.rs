use crate::{
    auth::session::get_user,
    components::{
        AvatarButton, Dropdown, DropdownHeader, DropdownItem, DropdownMenu, DropdownSide,
    },
    theme::ThemeToggle,
};
use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    let user_resource = Resource::new(|| (), |_| get_user());

    view! {
        <nav class="">
            <div class="px-4">
                <div class="flex justify-between items-center h-16">
                    <div class="flex items-center space-x-4">
                        // <OrganizationSelector />
                    </div>

                    <div class="flex items-center space-x-4">
                        <ThemeToggle />
                        // <WalletConnectButton />

                        <Dropdown>
                            <Suspense fallback=move || {
                                view! {
                                    <div class="w-8 h-8 bg-neutral-300 rounded-full animate-pulse"></div>
                                }
                            }>
                                {move || {
                                    match user_resource.get() {
                                        Some(Ok(user)) => {
                                            let avatar_url = if let Some(image) = user.image.clone() {
                                                image
                                            } else {
                                                format!(
                                                    "https://ui-avatars.com/api/?name={}&background=3B82F6&color=fff&size=32",
                                                    user.name.clone(),
                                                )
                                            };

                                            view! {
                                                <AvatarButton
                                                    src=avatar_url
                                                    alt="User avatar".to_string()
                                                    class="flex items-center space-x-2 p-2 rounded-full hover:bg-neutral-100 dark:hover:bg-neutral-700 focus:outline-none focus:ring-2 focus:ring-blue-500 touch-manipulation active:bg-neutral-200 dark:active:bg-neutral-600 transition-colors duration-150"
                                                />
                                            }
                                                .into_any()
                                        }
                                        Some(Err(_)) => {
                                            view! {
                                                <button
                                                    class="w-8 h-8 bg-neutral-400 rounded-full hidden"
                                                    disabled=true
                                                ></button>
                                            }
                                                .into_any()
                                        }
                                        None => {
                                            view! {
                                                <button
                                                    class="w-8 h-8 bg-neutral-300 rounded-full animate-pulse"
                                                    disabled=true
                                                ></button>
                                            }
                                                .into_any()
                                        }
                                    }
                                }}
                            </Suspense>

                            <DropdownMenu side=DropdownSide::Right>
                                <Suspense fallback=move || {
                                    view! { <div class="p-2">"Loading..."</div> }
                                }>
                                    {move || {
                                        match user_resource.get() {
                                            Some(Ok(user)) => {
                                                view! {
                                                    <DropdownHeader>
                                                        {user.name}
                                                    </DropdownHeader>
                                                    <DropdownItem href="/settings">
                                                        "Settings"
                                                    </DropdownItem>
                                                    <DropdownItem href="/logout">
                                                        "Logout"
                                                    </DropdownItem>
                                                }
                                                    .into_any()
                                            }
                                            _ => {
                                                view! {
                                                    <DropdownHeader>
                                                        "Error"
                                                    </DropdownHeader>
                                                    <DropdownItem href="/logout">
                                                        "Logout"
                                                    </DropdownItem>
                                                }
                                                    .into_any()
                                            }
                                        }
                                    }}
                                </Suspense>
                            </DropdownMenu>
                        </Dropdown>
                    </div>
                </div>
            </div>
        </nav>
    }
}
