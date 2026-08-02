use crate::models::Notebook;
use crate::state::Modo;
use gloo_storage::{errors::StorageError, LocalStorage, Storage};
use std::cell::Cell;
use std::rc::Rc;

const STORAGE_KEY: &str = "gestor-de-notas:notebook";
const MODO_KEY: &str = "gestor-de-notas:modo";
const DEBOUNCE_MS: u32 = 400;

pub fn load() -> Notebook {
    LocalStorage::get::<Notebook>(STORAGE_KEY).unwrap_or_default()
}

pub fn load_modo() -> Option<Modo> {
    LocalStorage::get::<Modo>(MODO_KEY).ok()
}

pub fn save_modo(modo: Modo) {
    let _ = LocalStorage::set(MODO_KEY, modo);
}

fn save_now(nb: &Notebook) -> Result<(), StorageError> {
    LocalStorage::set(STORAGE_KEY, nb)
}

pub fn save_debounced(nb: Notebook, token: Rc<Cell<u64>>) {
    let mine = token.get() + 1;
    token.set(mine);
    wasm_bindgen_futures::spawn_local(async move {
        gloo_timers::future::TimeoutFuture::new(DEBOUNCE_MS).await;
        if token.get() == mine {
            if let Err(e) = save_now(&nb) {
                web_sys::console::error_1(&format!("Error guardando en localStorage: {e}").into());
            }
        }
    });
}
