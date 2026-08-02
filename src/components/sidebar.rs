use crate::components::ajustes::AjustesPanel;
use crate::components::editable_text::EditableText;
use crate::components::reorder_buttons::ReorderButtons;
use crate::drive;
use crate::models::{mover, Ramo};
use crate::state::{use_app_state, DriveStatus};
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn Sidebar() -> impl IntoView {
    let state = use_app_state();

    let ramo_ids = Memo::new(move |_| state.notebook.with(|nb| nb.ramos.iter().map(|r| r.id).collect::<Vec<_>>()));

    let agregar_ramo = move |_| {
        let nuevo_id = Uuid::new_v4();
        state.notebook.update(|nb| {
            let mut ramo = Ramo::nuevo("Nuevo ramo");
            ramo.id = nuevo_id;
            nb.ramos.push(ramo);
        });
        state.ramo_seleccionado.set(Some(nuevo_id));
        state.cerrar_sidebar_si_telefono();
    };

    view! {
        <aside class="sidebar" class:abierta=move || state.sidebar_abierta.get()>
            <div class="sidebar-header">
                <div class="sidebar-titulo">"Mis Ramos"</div>
                <button class="btn-cerrar-sidebar" title="Cerrar" on:click=move |_| state.sidebar_abierta.set(false)>
                    "✕"
                </button>
            </div>
            <div class="ramos-lista">
                <For each=move || ramo_ids.get() key=|id| *id let(ramo_id)>
                    {move || {
                        let total = ramo_ids.get().len();
                        let idx = ramo_ids.get().iter().position(|id| *id == ramo_id).unwrap_or(0);
                        view! { <RamoItem ramo_id idx total /> }
                    }}
                </For>
            </div>
            <button class="btn-agregar" on:click=agregar_ramo>
                "+ Agregar ramo"
            </button>
            <AjustesPanel />
            <DrivePanel />
        </aside>
    }
}

#[component]
fn RamoItem(ramo_id: Uuid, idx: usize, total: usize) -> impl IntoView {
    let state = use_app_state();

    let nombre = Signal::derive(move || {
        state.notebook.with(|nb| nb.ramo(ramo_id).map(|r| r.nombre.clone())).unwrap_or_default()
    });
    let seleccionado = Signal::derive(move || state.ramo_seleccionado.get() == Some(ramo_id));

    let set_nombre = move |v: String| {
        state.notebook.update(|nb| {
            if let Some(r) = nb.ramo_mut(ramo_id) {
                r.nombre = v;
            }
        });
    };
    let seleccionar = move |_| {
        state.ramo_seleccionado.set(Some(ramo_id));
        state.cerrar_sidebar_si_telefono();
    };
    let borrar = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        let confirmado = web_sys::window()
            .and_then(|w| w.confirm_with_message("¿Borrar este ramo y todo su contenido?").ok())
            .unwrap_or(false);
        if !confirmado {
            return;
        }
        state.notebook.update(|nb| nb.ramos.retain(|r| r.id != ramo_id));
        if state.ramo_seleccionado.get() == Some(ramo_id) {
            let primero = state.notebook.with(|nb| nb.ramos.first().map(|r| r.id));
            state.ramo_seleccionado.set(primero);
        }
    };
    let mover_ramo = move |dir: isize| {
        state.notebook.update(|nb| mover(&mut nb.ramos, idx, dir));
    };

    view! {
        <div class="ramo-item" class:activo=move || seleccionado.get() on:click=seleccionar>
            <ReorderButtons
                on_up=move || mover_ramo(-1)
                on_down=move || mover_ramo(1)
                disable_up={idx == 0}
                disable_down={idx + 1 >= total}
            />
            <EditableText value=nombre on_change=set_nombre class="nombre-ramo-item" placeholder="Ramo" />
            <button class="btn-borrar" title="Borrar ramo" on:click=borrar>
                "🗑"
            </button>
        </div>
    }
}

#[component]
fn DrivePanel() -> impl IntoView {
    let state = use_app_state();
    let sin_client_id = drive::GOOGLE_CLIENT_ID.is_none();

    let conectar = move |_| {
        state.drive_status.set(DriveStatus::Conectando);
        drive::request_access_token(
            move |token| state.drive_status.set(DriveStatus::Conectado { access_token: token }),
            move |err| state.drive_status.set(DriveStatus::Error(err)),
        );
    };

    let guardar = move |_| {
        let DriveStatus::Conectado { access_token } = state.drive_status.get_untracked() else {
            return;
        };
        state.drive_status.set(DriveStatus::Sincronizando);
        let nb = state.notebook.get_untracked();
        wasm_bindgen_futures::spawn_local(async move {
            match drive::guardar(&access_token, &nb).await {
                Ok(file_id) => {
                    state.notebook.update(|nb| nb.drive_file_id = Some(file_id));
                    state.drive_status.set(DriveStatus::Conectado { access_token });
                }
                Err(e) => state.drive_status.set(DriveStatus::Error(e)),
            }
        });
    };

    let cargar = move |_| {
        let DriveStatus::Conectado { access_token } = state.drive_status.get_untracked() else {
            return;
        };
        let confirmado = web_sys::window()
            .and_then(|w| w.confirm_with_message("Esto reemplazará tus datos locales con los de Google Drive. ¿Continuar?").ok())
            .unwrap_or(false);
        if !confirmado {
            return;
        }
        state.drive_status.set(DriveStatus::Sincronizando);
        wasm_bindgen_futures::spawn_local(async move {
            match drive::cargar(&access_token).await {
                Ok(Some(nuevo)) => {
                    state.notebook.set(nuevo);
                    state.ramo_seleccionado.set(None);
                    let primero = state.notebook.with(|nb| nb.ramos.first().map(|r| r.id));
                    state.ramo_seleccionado.set(primero);
                    state.drive_status.set(DriveStatus::Conectado { access_token });
                }
                Ok(None) => state.drive_status.set(DriveStatus::Conectado { access_token }),
                Err(e) => state.drive_status.set(DriveStatus::Error(e)),
            }
        });
    };

    view! {
        <div class="drive-panel">
            {move || match state.drive_status.get() {
                DriveStatus::Desconectado => view! {
                    <button class="btn-drive" disabled=sin_client_id on:click=conectar>
                        "Conectar Google Drive"
                    </button>
                    {sin_client_id
                        .then(|| view! { <p class="drive-hint">"No configurado (ver README)"</p> }.into_any())}
                }
                .into_any(),
                DriveStatus::Conectando => view! { <p class="drive-hint">"Conectando…"</p> }.into_any(),
                DriveStatus::Sincronizando => view! { <p class="drive-hint">"Sincronizando…"</p> }.into_any(),
                DriveStatus::Conectado { .. } => view! {
                    <div class="drive-acciones">
                        <button class="btn-drive" on:click=guardar>
                            "Guardar en Drive"
                        </button>
                        <button class="btn-drive" on:click=cargar>
                            "Cargar desde Drive"
                        </button>
                        <p class="drive-hint">{drive::descripcion_scope()}</p>
                    </div>
                }
                .into_any(),
                DriveStatus::Error(e) => view! {
                    <div>
                        <p class="drive-hint drive-error">{format!("Error: {e}")}</p>
                        <button class="btn-drive" disabled=sin_client_id on:click=conectar>
                            "Reintentar"
                        </button>
                    </div>
                }
                .into_any(),
            }}
        </div>
    }
}
