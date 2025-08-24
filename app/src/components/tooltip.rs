use leptos::prelude::*;
use tw_merge::tw_merge;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    Left,
    Top,
    Right,
    Bottom,
}

#[component]
pub fn Tooltip<T>(
    label: T,
    #[prop(optional)] align: Option<Align>,
    children: Children,
) -> impl IntoView
where
    T: ToString + Clone + 'static,
{
    let alignment = align.unwrap_or(Align::Top);
    let is_visible = RwSignal::new(false);
    let label_string = label.to_string();

    let tooltip_classes = "absolute z-[200] px-2 py-1 text-sm text-white bg-neutral-900 dark:bg-neutral-700 rounded-lg shadow-lg pointer-events-none whitespace-nowrap transition-opacity duration-200";

    let position_classes = match alignment {
        Align::Top => "bottom-full left-1/2 transform -translate-x-1/2 mb-2",
        Align::Bottom => "top-full left-1/2 transform -translate-x-1/2 mt-2",
        Align::Left => "right-full top-1/2 transform -translate-y-1/2 mr-2",
        Align::Right => "left-full top-1/2 transform -translate-y-1/2 ml-2",
    };

    let arrow_classes = "absolute w-0 h-0 border-solid";
    let arrow_position = match alignment {
        Align::Top => "top-full left-1/2 transform -translate-x-1/2 border-l-[5px] border-l-transparent border-r-[5px] border-r-transparent border-t-[5px] border-t-neutral-900 dark:border-t-neutral-700",
        Align::Bottom => "bottom-full left-1/2 transform -translate-x-1/2 border-l-[5px] border-l-transparent border-r-[5px] border-r-transparent border-b-[5px] border-b-neutral-900 dark:border-b-neutral-700",
        Align::Left => "left-full top-1/2 transform -translate-y-1/2 border-t-[5px] border-t-transparent border-b-[5px] border-b-transparent border-l-[5px] border-l-neutral-900 dark:border-l-neutral-700",
        Align::Right => "right-full top-1/2 transform -translate-y-1/2 border-t-[5px] border-t-transparent border-b-[5px] border-b-transparent border-r-[5px] border-r-neutral-900 dark:border-r-neutral-700",
    };

    let label_clone = label_string.clone();

    view! {
        <div
            class="relative inline-block w-min"
            on:mouseenter=move |_| is_visible.set(true)
            on:mouseleave=move |_| is_visible.set(false)
        >
            {children()}
            <Show when=move || is_visible.get()>
                <div class=tw_merge!("opacity-100", tooltip_classes, position_classes)>
                    {label_string.clone()}
                    <div class=tw_merge!(arrow_classes, arrow_position)></div>
                </div>
            </Show>
            <Show when=move || !is_visible.get()>
                <div class=tw_merge!("opacity-0", tooltip_classes, position_classes)>
                    {label_clone.clone()}
                    <div class=tw_merge!(arrow_classes, arrow_position)></div>
                </div>
            </Show>
        </div>
    }
}
