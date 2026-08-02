use crate::state::{use_app_state, Modo};
use leptos::prelude::*;

#[component]
pub fn ModoSelector() -> impl IntoView {
    let state = use_app_state();

    view! {
        <div class="modo-selector">
            <div class="modo-tarjeta">
                <h1 class="modo-titulo">"Gestor de Notas"</h1>
                <p class="modo-subtitulo">"¿Desde dónde vas a entrar?"</p>
                <div class="modo-opciones">
                    <button class="modo-boton" on:click=move |_| state.elegir_modo(Modo::Computador)>
                        <span class="modo-icono">"💻"</span>
                        <span class="modo-nombre">"Computador"</span>
                        <span class="modo-desc">"Barra lateral fija, columnas lado a lado"</span>
                    </button>
                    <button class="modo-boton" on:click=move |_| state.elegir_modo(Modo::Telefono)>
                        <span class="modo-icono">"📱"</span>
                        <span class="modo-nombre">"Teléfono"</span>
                        <span class="modo-desc">"Barra lateral ocultable, pantalla adaptada"</span>
                    </button>
                </div>
                <p class="modo-hint">"Puedes cambiarlo después desde Ajustes."</p>
            </div>
        </div>
    }
}
