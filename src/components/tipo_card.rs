use crate::components::editable_number::EditableNumber;
use crate::components::editable_text::EditableText;
use crate::components::evaluacion_row::EvaluacionRow;
use crate::components::reorder_buttons::ReorderButtons;
use crate::components::weight_badge::WeightBadge;
use crate::models::{distribuir_100, mover, Evaluacion};
use crate::state::use_app_state;
use crate::utils::formatear;
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn TipoCard(ramo_id: Uuid, seccion_id: Uuid, tipo_id: Uuid, idx: usize, total: usize) -> impl IntoView {
    let state = use_app_state();
    let mostrar_promedio = RwSignal::new(false);

    let nombre = Signal::derive(move || {
        state
            .notebook
            .with(|nb| nb.tipo(ramo_id, seccion_id, tipo_id).map(|t| t.nombre.clone()))
            .unwrap_or_default()
    });
    let ponderacion = Signal::derive(move || {
        state
            .notebook
            .with(|nb| nb.tipo(ramo_id, seccion_id, tipo_id).map(|t| t.ponderacion))
            .unwrap_or(0.0)
    });
    let suma_ponderaciones = Signal::derive(move || {
        state
            .notebook
            .with(|nb| nb.tipo(ramo_id, seccion_id, tipo_id).map(|t| t.suma_ponderaciones()))
            .unwrap_or(0.0)
    });
    let promedio = Signal::derive(move || {
        state
            .notebook
            .with(|nb| nb.tipo(ramo_id, seccion_id, tipo_id).map(|t| t.promedio()))
            .unwrap_or(0.0)
    });
    let evaluacion_ids = Memo::new(move |_| {
        state
            .notebook
            .with(|nb| {
                nb.tipo(ramo_id, seccion_id, tipo_id)
                    .map(|t| t.evaluaciones.iter().map(|e| e.id).collect::<Vec<_>>())
            })
            .unwrap_or_default()
    });

    let set_nombre = move |v: String| {
        state.notebook.update(|nb| {
            if let Some(t) = nb.tipo_mut(ramo_id, seccion_id, tipo_id) {
                t.nombre = v;
            }
        });
    };
    let set_ponderacion = move |v: f64| {
        state.notebook.update(|nb| {
            if let Some(t) = nb.tipo_mut(ramo_id, seccion_id, tipo_id) {
                t.ponderacion = v;
            }
        });
    };
    let agregar_evaluacion = move |_| {
        state.notebook.update(|nb| {
            if let Some(t) = nb.tipo_mut(ramo_id, seccion_id, tipo_id) {
                t.evaluaciones.push(Evaluacion::nueva("Nueva evaluación"));
            }
        });
    };
    let promediar_tipo = move |_| {
        state.notebook.update(|nb| {
            if let Some(t) = nb.tipo_mut(ramo_id, seccion_id, tipo_id) {
                let partes = distribuir_100(t.evaluaciones.len());
                for (e, p) in t.evaluaciones.iter_mut().zip(partes) {
                    e.ponderacion = p;
                }
            }
        });
        mostrar_promedio.update(|v| *v = !*v);
    };
    let borrar = move |_| {
        state.notebook.update(|nb| {
            if let Some(s) = nb.seccion_mut(ramo_id, seccion_id) {
                s.tipos.retain(|t| t.id != tipo_id);
            }
        });
    };
    let mover_tipo = move |dir: isize| {
        state.notebook.update(|nb| {
            if let Some(s) = nb.seccion_mut(ramo_id, seccion_id) {
                mover(&mut s.tipos, idx, dir);
            }
        });
    };

    view! {
        <div class="tipo-card">
            <div class="tipo-card-header">
                <ReorderButtons
                    on_up=move || mover_tipo(-1)
                    on_down=move || mover_tipo(1)
                    disable_up={idx == 0}
                    disable_down={idx + 1 >= total}
                />
                <EditableText value=nombre on_change=set_nombre class="nombre-tipo" placeholder="Tipo de evaluación" />
                <EditableNumber value=ponderacion on_change=set_ponderacion class="ponderacion-tipo" />
                <span class="unidad">"%"</span>
                <button class="btn-borrar" title="Borrar tipo de evaluación" on:click=borrar>
                    "🗑"
                </button>
            </div>
            <div class="tipo-card-meta">
                <WeightBadge suma=suma_ponderaciones />
            </div>
            <button class="btn-promediar mini" title="Reparte el 100% en partes iguales entre las evaluaciones" on:click=promediar_tipo>
                "Promediar"
            </button>
            <Show when=move || mostrar_promedio.get()>
                <div class="resultado-tipo">"Nota: " <strong>{move || formatear(promedio.get())}</strong></div>
            </Show>
            <div class="evaluaciones-lista">
                <For each=move || evaluacion_ids.get() key=|id| *id let(evaluacion_id)>
                    {move || {
                        let total = evaluacion_ids.get().len();
                        let idx = evaluacion_ids.get().iter().position(|id| *id == evaluacion_id).unwrap_or(0);
                        view! {
                            <EvaluacionRow ramo_id seccion_id tipo_id evaluacion_id idx total />
                        }
                    }}
                </For>
            </div>
            <button class="btn-agregar" on:click=agregar_evaluacion>
                "+ Agregar evaluación"
            </button>
        </div>
    }
}
