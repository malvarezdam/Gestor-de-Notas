use crate::utils::{formatear, parsear_numero};
use leptos::ev;
use leptos::prelude::*;

#[component]
pub fn EditableOpcional(
    value: Signal<Option<f64>>,
    on_change: impl Fn(Option<f64>) + 'static + Copy,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] placeholder: String,
) -> impl IntoView {
    let formatear_valor = move |v: Option<f64>| v.map(formatear).unwrap_or_default();
    let texto = RwSignal::new(formatear_valor(value.get_untracked()));
    let enfocado = RwSignal::new(false);

    view! {
        <input
            type="text"
            inputmode="decimal"
            class=move || format!("editable-number {class}")
            placeholder=placeholder
            prop:value=move || {
                if enfocado.get() { texto.get() } else { formatear_valor(value.get()) }
            }
            on:focus=move |_| {
                texto.set(formatear_valor(value.get_untracked()));
                enfocado.set(true);
            }
            on:input=move |ev: ev::Event| {
                let v = event_target_value(&ev);
                texto.set(v.clone());
                if v.trim().is_empty() {
                    on_change(None);
                } else if let Some(n) = parsear_numero(&v) {
                    on_change(Some(n));
                }
            }
            on:blur=move |_| {
                enfocado.set(false);
            }
        />
    }
}
