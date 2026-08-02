use crate::models::Notebook;
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

pub const GOOGLE_CLIENT_ID: Option<&str> = option_env!("GOOGLE_CLIENT_ID");

const SCOPE_DRIVE_FILE_MSG: &str =
    "Acceso limitado: solo a archivos creados por esta app (scope drive.file)";

const NOMBRE_ARCHIVO_DRIVE: &str = "Gestor de Notas - NO BORRAR - datos guardados.json";
const DESCRIPCION_ARCHIVO_DRIVE: &str = "Archivo de datos de la app \"Gestor de Notas\". Contiene tus ramos, secciones, ponderaciones y notas guardadas. Si lo borras, perderás esta información (a menos que la app la tenga también en el almacenamiento local del navegador donde la creaste).";

pub fn descripcion_scope() -> &'static str {
    SCOPE_DRIVE_FILE_MSG
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = gisRequestAccessToken)]
    fn gis_request_access_token(
        client_id: &str,
        on_token: &Closure<dyn FnMut(String)>,
        on_error: &Closure<dyn FnMut(String)>,
    );
}

pub fn request_access_token(on_token: impl Fn(String) + 'static, on_error: impl Fn(String) + 'static) {
    let Some(client_id) = GOOGLE_CLIENT_ID else {
        on_error("Google Client ID no configurado (ver README)".to_string());
        return;
    };

    let on_token_closure =
        Closure::wrap(Box::new(move |token: String| on_token(token)) as Box<dyn FnMut(String)>);
    let on_error_closure =
        Closure::wrap(Box::new(move |err: String| on_error(err)) as Box<dyn FnMut(String)>);

    gis_request_access_token(client_id, &on_token_closure, &on_error_closure);

    on_token_closure.forget();
    on_error_closure.forget();
}

#[derive(Deserialize)]
struct DriveFile {
    id: String,
}

#[derive(Deserialize)]
struct DriveFileList {
    files: Vec<DriveFile>,
}

async fn fetch_raw(
    url: &str,
    method: &str,
    token: &str,
    body: Option<String>,
    content_type: Option<&str>,
) -> Result<JsValue, String> {
    let opts = RequestInit::new();
    opts.set_method(method);
    opts.set_mode(RequestMode::Cors);
    if let Some(b) = &body {
        opts.set_body(&JsValue::from_str(b));
    }

    let request = Request::new_with_str_and_init(url, &opts).map_err(|e| format!("{e:?}"))?;
    let headers = request.headers();
    headers
        .set("Authorization", &format!("Bearer {token}"))
        .map_err(|e| format!("{e:?}"))?;
    if let Some(ct) = content_type {
        headers.set("Content-Type", ct).map_err(|e| format!("{e:?}"))?;
    }

    let window = web_sys::window().ok_or("Sin objeto window")?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| format!("Error de red: {e:?}"))?;
    let resp: Response = resp_value.dyn_into().map_err(|e| format!("{e:?}"))?;
    if !resp.ok() {
        return Err(format!("Google Drive respondió con error HTTP {}", resp.status()));
    }
    let json_promise = resp.json().map_err(|e| format!("{e:?}"))?;
    JsFuture::from(json_promise)
        .await
        .map_err(|e| format!("Error leyendo respuesta: {e:?}"))
}

async fn buscar_archivo(token: &str) -> Result<Option<String>, String> {
    let query = format!("name = '{NOMBRE_ARCHIVO_DRIVE}' and trashed = false");
    let query_encoded = js_sys::encode_uri_component(&query)
        .as_string()
        .unwrap_or_default();
    let url = format!(
        "https://www.googleapis.com/drive/v3/files?q={query_encoded}&spaces=drive&fields=files(id,name)"
    );
    let json = fetch_raw(&url, "GET", token, None, None).await?;
    let list: DriveFileList = serde_wasm_bindgen::from_value(json).map_err(|e| e.to_string())?;
    Ok(list.files.into_iter().next().map(|f| f.id))
}

async fn crear_archivo(token: &str, notebook: &Notebook) -> Result<String, String> {
    let boundary = "gestor_de_notas_boundary_314159";
    let metadata = serde_json::json!({
        "name": NOMBRE_ARCHIVO_DRIVE,
        "description": DESCRIPCION_ARCHIVO_DRIVE,
    })
    .to_string();
    let contenido = serde_json::to_string(notebook).map_err(|e| e.to_string())?;
    let body = format!(
        "--{boundary}\r\nContent-Type: application/json; charset=UTF-8\r\n\r\n{metadata}\r\n--{boundary}\r\nContent-Type: application/json\r\n\r\n{contenido}\r\n--{boundary}--"
    );
    let url = "https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart&fields=id";
    let content_type = format!("multipart/related; boundary={boundary}");
    let json = fetch_raw(url, "POST", token, Some(body), Some(&content_type)).await?;
    let file: DriveFile = serde_wasm_bindgen::from_value(json).map_err(|e| e.to_string())?;
    Ok(file.id)
}

async fn actualizar_archivo(token: &str, file_id: &str, notebook: &Notebook) -> Result<(), String> {
    let url = format!("https://www.googleapis.com/upload/drive/v3/files/{file_id}?uploadType=media");
    let contenido = serde_json::to_string(notebook).map_err(|e| e.to_string())?;
    fetch_raw(&url, "PATCH", token, Some(contenido), Some("application/json")).await?;
    Ok(())
}

pub async fn descargar_archivo(token: &str, file_id: &str) -> Result<Notebook, String> {
    let url = format!("https://www.googleapis.com/drive/v3/files/{file_id}?alt=media");
    let json = fetch_raw(&url, "GET", token, None, None).await?;
    serde_wasm_bindgen::from_value(json).map_err(|e| e.to_string())
}

pub async fn guardar(token: &str, notebook: &Notebook) -> Result<String, String> {
    let file_id = match &notebook.drive_file_id {
        Some(id) => id.clone(),
        None => match buscar_archivo(token).await? {
            Some(id) => id,
            None => crear_archivo(token, notebook).await?,
        },
    };
    actualizar_archivo(token, &file_id, notebook).await?;
    Ok(file_id)
}

pub async fn cargar(token: &str) -> Result<Option<Notebook>, String> {
    match buscar_archivo(token).await? {
        Some(id) => Ok(Some(descargar_archivo(token, &id).await?)),
        None => Ok(None),
    }
}
