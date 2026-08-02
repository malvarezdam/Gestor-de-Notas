use crate::utils::{formatear, parsear_numero};
use leptos::ev;
use leptos::prelude::*;

#[component]
pub fn EditableNumber(
    value: Signal<f64>,
    on_change: impl Fn(f64) + 'static + Copy,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let texto = RwSignal::new(formatear(value.get_untracked()));
    let enfocado = RwSignal::new(false);

    view! {
        <input
            type="text"
            inputmode="decimal"
            class=move || format!("editable-number {class}")
            prop:value=move || {
                if enfocado.get() { texto.get() } else { formatear(value.get()) }
            }
            on:focus=move |_| {
                texto.set(formatear(value.get_untracked()));
                enfocado.set(true);
            }
            on:input=move |ev: ev::Event| {
                let v = event_target_value(&ev);
                texto.set(v.clone());
                if let Some(n) = parsear_numero(&v) {
                    on_change(n);
                }
            }
            on:blur=move |_| {
                enfocado.set(false);
            }
        />
    }
}
