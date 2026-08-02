use leptos::prelude::*;

#[component]
pub fn ReorderButtons(
    on_up: impl Fn() + 'static,
    on_down: impl Fn() + 'static,
    disable_up: bool,
    disable_down: bool,
) -> impl IntoView {
    view! {
        <div class="reorder-buttons">
            <button class="reorder-btn" title="Subir" disabled=disable_up on:click=move |_| on_up()>
                "▲"
            </button>
            <button class="reorder-btn" title="Bajar" disabled=disable_down on:click=move |_| on_down()>
                "▼"
            </button>
        </div>
    }
}
