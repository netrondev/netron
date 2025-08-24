use crate::components::ParticleAnimation;
use leptos::prelude::*;

#[component]
pub fn Hero(title: &'static str, subtitle: &'static str, children: Children) -> impl IntoView {
    view! {
        <section class="relative py-[150px] flex-1 flex items-center justify-center overflow-hidden bg-neutral-50 dark:bg-neutral-900">
            // Particle animation background
            <div class="absolute inset-0 overflow-hidden">
                <ParticleAnimation />
            </div>

            // Subtle gradient overlay for depth
            <div class="absolute inset-0 bg-gradient-to-b from-transparent via-neutral-50/50 to-neutral-50 dark:via-neutral-950/50 dark:to-neutral-950 opacity-30"></div>

            // Content
            <div class="relative z-10 max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
                <h1 class="text-5xl md:text-7xl font-bold text-neutral-900 dark:text-neutral-100 mb-6 tracking-tight">
                    {title}
                </h1>
                <p class="text-xl md:text-2xl text-neutral-600 dark:text-neutral-400 mb-12 max-w-3xl mx-auto">
                    {subtitle}
                </p>
                <div class="flex flex-col sm:flex-row gap-4 justify-center">
                    {children()}
                </div>
            </div>

            // Scroll indicator
            <div id="arrowdown" class="absolute bottom-8 left-1/2 transform -translate-x-1/2 animate-bounce">
                <svg class="w-6 h-6 text-neutral-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3"></path>
                </svg>
            </div>
        </section>
    }
}

#[component]
pub fn HeroButton(href: &'static str, variant: &'static str, children: Children) -> impl IntoView {
    let class = match variant {
        "primary" => "bg-neutral-900 dark:bg-neutral-100 text-neutral-100 dark:text-neutral-900 hover:bg-neutral-800 dark:hover:bg-neutral-200 px-8 py-4 rounded-lg font-semibold text-lg transition-all transform hover:scale-105 shadow-lg",
        "secondary" => "bg-transparent border-2 border-neutral-900 dark:border-neutral-100 text-neutral-900 dark:text-neutral-100 hover:bg-neutral-900 hover:text-neutral-100 dark:hover:bg-neutral-100 dark:hover:text-neutral-900 px-8 py-4 rounded-lg font-semibold text-lg transition-all transform hover:scale-105",
        _ => ""
    };

    view! {
        <a href={href} class={class}>
            {children()}
        </a>
    }
}
