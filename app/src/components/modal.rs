use leptos::prelude::*;
use tw_merge::tw_merge;

#[component]
pub fn Modal(
    show: Signal<bool>,
    #[prop(optional)] on_close: Option<Callback<()>>,
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] size: ModalSize,
    children: ChildrenFn,
) -> impl IntoView {
    let close_modal = move |_| {
        if let Some(callback) = on_close {
            callback.run(());
        }
    };

    let size_class = match size {
        ModalSize::Small => "max-w-md",
        ModalSize::Medium => "max-w-lg",
        ModalSize::Large => "max-w-2xl",
        ModalSize::ExtraLarge => "max-w-4xl",
        ModalSize::FullWidth => "max-w-7xl",
    };

    view! {
        <Show when=move || show.get() fallback=|| ()>
            <div class="fixed inset-0 z-50 overflow-y-auto" aria-labelledby="modal-title" role="dialog" aria-modal="true">
                <div class="flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
                    <div
                        class="fixed inset-0 bg-red-500 opacity-0"
                        aria-hidden="true"
                        on:click=close_modal
                    ></div>

                    <div class=tw_merge!("inline-block align-bottom bg-white dark:bg-neutral-800 rounded-lg container text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle {} w-full", size_class)>
                        <div class="px-4 pt-5 pb-4 sm:p-6 sm:pb-4 relative dark:bg-black bg-white rounded-md">
                            {title.as_ref().map(|t| {
                                view! {
                                    <h3 class="text-lg leading-6 font-medium text-neutral-900 dark:text-neutral-100 mb-4" id="modal-title">
                                        {t.clone()}
                                    </h3>
                                }
                            })}
                            {children()}
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ModalSize {
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
    FullWidth,
}
