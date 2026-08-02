use leptos::ev;
use leptos::prelude::*;

#[component]
pub fn EditableText(
    value: Signal<String>,
    on_change: impl Fn(String) + 'static + Copy,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] placeholder: String,
) -> impl IntoView {
    let texto = RwSignal::new(value.get_untracked());
    let enfocado = RwSignal::new(false);

    view! {
        <input
            type="text"
            class=move || format!("editable-text {class}")
            placeholder=placeholder
            prop:value=move || if enfocado.get() { texto.get() } else { value.get() }
            on:focus=move |_| {
                texto.set(value.get_untracked());
                enfocado.set(true);
            }
            on:input=move |ev: ev::Event| {
                let v = event_target_value(&ev);
                texto.set(v.clone());
                on_change(v);
            }
            on:blur=move |_| enfocado.set(false)
        />
    }
}
