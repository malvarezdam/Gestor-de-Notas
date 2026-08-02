use crate::components::editable_number::EditableNumber;
use crate::state::{use_app_state, Modo};
use leptos::prelude::*;

#[component]
pub fn AjustesPanel() -> impl IntoView {
    let state = use_app_state();
    let mostrar = RwSignal::new(false);

    let nota_minima = Signal::derive(move || state.notebook.with(|nb| nb.nota_minima));
    let set_nota_minima = move |v: f64| {
        state.notebook.update(|nb| nb.nota_minima = v);
    };
    let nota_maxima = Signal::derive(move || state.notebook.with(|nb| nb.nota_maxima));
    let set_nota_maxima = move |v: f64| {
        state.notebook.update(|nb| nb.nota_maxima = v);
    };
    let es_telefono = Signal::derive(move || matches!(state.modo.get(), Some(Modo::Telefono)));

    view! {
        <div class="ajustes-panel">
            <button class="btn-ajustes" on:click=move |_| mostrar.update(|v| *v = !*v)>
                "⚙ Ajustes"
            </button>
            <Show when=move || mostrar.get()>
                <div class="ajustes-contenido">
                    <label>
                        "Nota mínima de tu escala"
                        <EditableNumber value=nota_minima on_change=set_nota_minima class="nota-minima" />
                    </label>
                    <label>
                        "Nota máxima de tu escala"
                        <EditableNumber value=nota_maxima on_change=set_nota_maxima class="nota-maxima" />
                    </label>
                    <p class="ajustes-hint">"Se usan para saber si tu nota objetivo por ramo es alcanzable."</p>
                    <div class="ajustes-modo">
                        <span>"Vista"</span>
                        <button
                            class="btn-modo"
                            class:activo=move || !es_telefono.get()
                            on:click=move |_| state.elegir_modo(Modo::Computador)
                        >
                            "💻 Computador"
                        </button>
                        <button
                            class="btn-modo"
                            class:activo=move || es_telefono.get()
                            on:click=move |_| state.elegir_modo(Modo::Telefono)
                        >
                            "📱 Teléfono"
                        </button>
                    </div>
                </div>
            </Show>
        </div>
    }
}
