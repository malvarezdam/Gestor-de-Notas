use crate::utils::formatear_entero;
use leptos::prelude::*;

#[component]
pub fn WeightBadge(suma: Signal<f64>) -> impl IntoView {
    view! {
        <span class=move || {
            if (suma.get() - 100.0).abs() < 0.01 { "weight-badge ok" } else { "weight-badge warn" }
        }>
            "Σ " {move || formatear_entero(suma.get())} "%"
        </span>
    }
}
