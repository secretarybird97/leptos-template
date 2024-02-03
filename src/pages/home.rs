use crate::components::navbar::*;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn Home() -> impl IntoView {
    let (value, set_value) = create_signal(0);
    view! {
        <Title text="Leptos + Tailwindcss"/>
        <main>
            <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
                <Navbar/>
                <div class="flex flex-row-reverse flex-wrap m-auto">
                    <button on:click=move |_| set_value.update(|value| *value += 1) class="btn">
                        "+"
                    </button>
                    <button class="btn">{value}</button>
                    <button on:click=move |_| set_value.update(|value| *value -= 1) class="btn">
                        "-"
                    </button>
                </div>
            </div>
        </main>
    }
}
