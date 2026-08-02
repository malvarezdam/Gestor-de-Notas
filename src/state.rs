use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Modo {
    Computador,
    Telefono,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DriveStatus {
    Desconectado,
    Conectando,
    Conectado { access_token: String },
    Sincronizando,
    Error(String),
}

#[derive(Clone, Copy)]
pub struct AppState {
    pub notebook: RwSignal<crate::models::Notebook>,
    pub ramo_seleccionado: RwSignal<Option<Uuid>>,
    pub drive_status: RwSignal<DriveStatus>,
    pub modo: RwSignal<Option<Modo>>,
    pub sidebar_abierta: RwSignal<bool>,
}

impl AppState {
    pub fn new() -> Self {
        let notebook = crate::storage::load();
        let primer_ramo = notebook.ramos.first().map(|r| r.id);
        Self {
            notebook: RwSignal::new(notebook),
            ramo_seleccionado: RwSignal::new(primer_ramo),
            drive_status: RwSignal::new(DriveStatus::Desconectado),
            modo: RwSignal::new(crate::storage::load_modo()),
            sidebar_abierta: RwSignal::new(false),
        }
    }

    pub fn elegir_modo(&self, modo: Modo) {
        crate::storage::save_modo(modo);
        self.modo.set(Some(modo));
        self.sidebar_abierta.set(matches!(modo, Modo::Computador));
    }

    pub fn cerrar_sidebar_si_telefono(&self) {
        if matches!(self.modo.get_untracked(), Some(Modo::Telefono)) {
            self.sidebar_abierta.set(false);
        }
    }
}

pub fn use_app_state() -> AppState {
    use_context::<AppState>().expect("AppState debe estar provisto por <App/>")
}
