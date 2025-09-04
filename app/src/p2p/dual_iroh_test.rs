use crate::p2p::iroh_ui::IrohTest;
use leptos::prelude::*;

#[component]
pub fn DualIrohTest() -> impl IntoView {
    view! {
        <div class="p-4 max-w-7xl mx-auto">
            <div class="mb-6">
                <h1 class="text-2xl font-bold mb-2">"P2P Chat Test Environment"</h1>
                <p class="text-gray-600 dark:text-gray-400">
                    "Two independent chat instances for testing P2P functionality in one browser"
                </p>
            </div>

            <div class="grid lg:grid-cols-2 gap-8">
                <div class="border border-gray-200 dark:border-gray-700 rounded-lg p-4">
                    <h2 class="text-lg font-semibold mb-4 text-blue-600 dark:text-blue-400">"Chat Instance A"</h2>
                    <IrohTest />
                </div>

                <div class="border border-gray-200 dark:border-gray-700 rounded-lg p-4">
                    <h2 class="text-lg font-semibold mb-4 text-green-600 dark:text-green-400">"Chat Instance B"</h2>
                    <IrohTest />
                </div>
            </div>
        </div>
    }
}
