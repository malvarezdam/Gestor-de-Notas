use crate::components::modo_selector::ModoSelector;
use crate::components::ramo_view::RamoView;
use crate::components::sidebar::Sidebar;
use crate::state::{AppState, Modo};
use crate::storage;
use leptos::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

#[component]
pub fn App() -> impl IntoView {
    let state = AppState::new();
    provide_context(state);

    let token = Rc::new(Cell::new(0u64));
    Effect::new(move |_| {
        let nb = state.notebook.get();
        storage::save_debounced(nb, token.clone());
    });

    view! {
        {move || match state.modo.get() {
            None => view! { <ModoSelector /> }.into_any(),
            Some(modo) => {
                let telefono = matches!(modo, Modo::Telefono);
                view! {
                    <div class="app-shell" class:telefono=telefono>
                        <Show when=move || telefono && state.sidebar_abierta.get()>
                            <div
                                class="sidebar-backdrop"
                                on:click=move |_| state.sidebar_abierta.set(false)
                            ></div>
                        </Show>
                        <Sidebar />
                        <div class="main-col">
                            <Show when=move || telefono>
                                <div class="topbar-movil">
                                    <button
                                        class="btn-hamburguesa"
                                        title="Mostrar/ocultar ramos"
                                        on:click=move |_| state.sidebar_abierta.update(|v| *v = !*v)
                                    >
                                        "☰"
                                    </button>
                                    <span class="topbar-titulo">"Gestor de Notas"</span>
                                </div>
                            </Show>
                            <RamoView />
                        </div>
                    </div>
                }
                .into_any()
            }
        }}
    }
}
