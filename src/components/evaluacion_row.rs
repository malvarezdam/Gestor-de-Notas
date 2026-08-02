use crate::components::editable_number::EditableNumber;
use crate::components::editable_opcional::EditableOpcional;
use crate::components::editable_text::EditableText;
use crate::components::reorder_buttons::ReorderButtons;
use crate::models::mover;
use crate::state::use_app_state;
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn EvaluacionRow(
    ramo_id: Uuid,
    seccion_id: Uuid,
    tipo_id: Uuid,
    evaluacion_id: Uuid,
    idx: usize,
    total: usize,
) -> impl IntoView {
    let state = use_app_state();

    let nombre = Signal::derive(move || {
        state
            .notebook
            .with(|nb| nb.evaluacion(ramo_id, seccion_id, tipo_id, evaluacion_id).map(|e| e.nombre.clone()))
            .unwrap_or_default()
    });
    let ponderacion = Signal::derive(move || {
        state
            .notebook
            .with(|nb| nb.evaluacion(ramo_id, seccion_id, tipo_id, evaluacion_id).map(|e| e.ponderacion))
            .unwrap_or(0.0)
    });
    let nota = Signal::derive(move || {
        state
            .notebook
            .with(|nb| nb.evaluacion(ramo_id, seccion_id, tipo_id, evaluacion_id).and_then(|e| e.nota))
    });

    let set_nombre = move |v: String| {
        state.notebook.update(|nb| {
            if let Some(e) = nb.evaluacion_mut(ramo_id, seccion_id, tipo_id, evaluacion_id) {
                e.nombre = v;
            }
        });
    };
    let set_ponderacion = move |v: f64| {
        state.notebook.update(|nb| {
            if let Some(e) = nb.evaluacion_mut(ramo_id, seccion_id, tipo_id, evaluacion_id) {
                e.ponderacion = v;
            }
        });
    };
    let set_nota = move |v: Option<f64>| {
        state.notebook.update(|nb| {
            if let Some(e) = nb.evaluacion_mut(ramo_id, seccion_id, tipo_id, evaluacion_id) {
                e.nota = v;
            }
        });
    };
    let borrar = move |_| {
        state.notebook.update(|nb| {
            if let Some(t) = nb.tipo_mut(ramo_id, seccion_id, tipo_id) {
                t.evaluaciones.retain(|e| e.id != evaluacion_id);
            }
        });
    };
    let mover_fila = move |dir: isize| {
        state.notebook.update(|nb| {
            if let Some(t) = nb.tipo_mut(ramo_id, seccion_id, tipo_id) {
                mover(&mut t.evaluaciones, idx, dir);
            }
        });
    };

    view! {
        <div class="evaluacion-row">
            <ReorderButtons
                on_up=move || mover_fila(-1)
                on_down=move || mover_fila(1)
                disable_up={idx == 0}
                disable_down={idx + 1 >= total}
            />
            <EditableText value=nombre on_change=set_nombre class="nombre-evaluacion" placeholder="Evaluación" />
            <EditableNumber value=ponderacion on_change=set_ponderacion class="ponderacion-evaluacion" />
            <span class="unidad">"%"</span>
            <EditableOpcional value=nota on_change=set_nota class="nota-evaluacion" placeholder="nota" />
            <button class="btn-borrar" title="Borrar evaluación" on:click=borrar>
                "🗑"
            </button>
        </div>
    }
}
