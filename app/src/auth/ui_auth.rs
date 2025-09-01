use crate::components::{
    input::{FormField, Input, InputType},
    Seperator,
};

use leptos::{prelude::*, reactive::spawn_local};

use serde::{Deserialize, Serialize};

use crate::auth::user::AdapterUser;

#[cfg(feature = "ssr")]
use crate::auth::user::CreateUserData;

#[component]
pub fn LoginForm() -> impl IntoView {
    view! {
        <div class="">

            <div class="bg-white dark:bg-black rounded-lg shadow-2xl p-8 mx-8 w-full max-w-md mx-auto">
                <div class="text-center mb-8">
                    <h1 class="text-3xl font-bold text-neutral-800 dark:text-neutral-100 mb-2">"Authentication"</h1>
                    <small class="text-neutral-500 dark:text-neutral-400">"Creates a new account or logs you back in."</small>
                </div>

                <Seperator />
            </div>
        </div>
    }
}
