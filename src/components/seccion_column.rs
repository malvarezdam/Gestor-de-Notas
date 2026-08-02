use crate::components::editable_number::EditableNumber;
use crate::components::editable_text::EditableText;
use crate::components::reorder_buttons::ReorderButtons;
use crate::components::tipo_card::TipoCard;
use crate::components::weight_badge::WeightBadge;
use crate::models::{distribuir_100, mover, TipoEvaluacion};
use crate::state::use_app_state;
use crate::utils::formatear;
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn SeccionColumn(ramo_id: Uuid, seccion_id: Uuid, idx: usize, total: usize) -> impl IntoView {
    let state = use_app_state();
    let mostrar_resultado = RwSignal::new(false);

    let nombre = Signal::derive(move || {
        state
            .notebook
            .with(|nb| nb.seccion(ramo_id, seccion_id).map(|s| s.nombre.clone()))
            .unwrap_or_default()
    });
    let ponderacion = Signal::derive(move || {
        state
            .notebook
            .with(|nb| nb.seccion(ramo_id, seccion_id).map(|s| s.ponderacion))
            .unwrap_or(0.0)
    });
    let factor_eta = Signal::derive(move || {
        state
            .notebook
            .with(|nb| nb.seccion(ramo_id, seccion_id).map(|s| s.factor_eta))
            .unwrap_or(1.0)
    });
    let promedio = Signal::derive(move || {
        state
            .notebook
            .with(|nb| nb.seccion(ramo_id, seccion_id).map(|s| s.promedio()))
            .unwrap_or(0.0)
    });
    let suma_ponderaciones = Signal::derive(move || {
        state
            .notebook
            .with(|nb| nb.seccion(ramo_id, seccion_id).map(|s| s.suma_ponderaciones()))
            .unwrap_or(0.0)
    });
    let incompleto = Signal::derive(move || {
        state
            .notebook
            .with(|nb| nb.seccion(ramo_id, seccion_id).map(|s| !s.completo()))
            .unwrap_or(false)
    });
    let tipo_ids = Memo::new(move |_| {
        state
            .notebook
            .with(|nb| nb.seccion(ramo_id, seccion_id).map(|s| s.tipos.iter().map(|t| t.id).collect::<Vec<_>>()))
            .unwrap_or_default()
    });

    let set_nombre = move |v: String| {
        state.notebook.update(|nb| {
            if let Some(s) = nb.seccion_mut(ramo_id, seccion_id) {
                s.nombre = v;
            }
        });
    };
    let set_ponderacion = move |v: f64| {
        state.notebook.update(|nb| {
            if let Some(s) = nb.seccion_mut(ramo_id, seccion_id) {
                s.ponderacion = v;
            }
        });
    };
    let set_factor_eta = move |v: f64| {
        state.notebook.update(|nb| {
            if let Some(s) = nb.seccion_mut(ramo_id, seccion_id) {
                s.factor_eta = v;
            }
        });
    };
    let agregar_tipo = move |_| {
        state.notebook.update(|nb| {
            if let Some(s) = nb.seccion_mut(ramo_id, seccion_id) {
                s.tipos.push(TipoEvaluacion::nuevo("Nuevo tipo"));
            }
        });
    };
    let promediar_seccion = move |_| {
        state.notebook.update(|nb| {
            if let Some(s) = nb.seccion_mut(ramo_id, seccion_id) {
                let partes = distribuir_100(s.tipos.len());
                for (t, p) in s.tipos.iter_mut().zip(partes) {
                    t.ponderacion = p;
                }
            }
        });
        mostrar_resultado.update(|v| *v = !*v);
    };
    let borrar = move |_| {
        let confirmado = web_sys::window()
            .and_then(|w| w.confirm_with_message("¿Borrar esta sección y todo su contenido?").ok())
            .unwrap_or(false);
        if !confirmado {
            return;
        }
        state.notebook.update(|nb| {
            if let Some(r) = nb.ramo_mut(ramo_id) {
                r.secciones.retain(|s| s.id != seccion_id);
            }
        });
    };
    let mover_seccion = move |dir: isize| {
        state.notebook.update(|nb| {
            if let Some(r) = nb.ramo_mut(ramo_id) {
                mover(&mut r.secciones, idx, dir);
            }
        });
    };

    view! {
        <div class="seccion-column">
            <div class="seccion-header">
                <ReorderButtons
                    on_up=move || mover_seccion(-1)
                    on_down=move || mover_seccion(1)
                    disable_up={idx == 0}
                    disable_down={idx + 1 >= total}
                />
                <EditableText value=nombre on_change=set_nombre class="nombre-seccion" placeholder="Sección" />
                <button class="btn-borrar" title="Borrar sección" on:click=borrar>
                    "🗑"
                </button>
            </div>
            <div class="seccion-config">
                <label>
                    "Ponderación " <EditableNumber value=ponderacion on_change=set_ponderacion class="ponderacion-seccion" />
                    "%"
                </label>
                <label>
                    "Factor η " <EditableNumber value=factor_eta on_change=set_factor_eta class="factor-eta" />
                </label>
            </div>
            <button class="btn-promediar" title="Reparte el 100% en partes iguales entre los tipos de evaluación" on:click=promediar_seccion>
                "Promediar"
            </button>
            <Show when=move || mostrar_resultado.get()>
                <div class="resultado-seccion">
                    "Nota sección: " <strong>{move || formatear(promedio.get())}</strong>
                    <Show when=move || incompleto.get()>
                        <span class="hint-incompleto" title="Las notas sin ingresar cuentan como 0 mientras tanto">
                            " (provisional, faltan notas)"
                        </span>
                    </Show>
                </div>
            </Show>
            <div class="tipos-header">
                <span>"Tipos de evaluación"</span>
                <WeightBadge suma=suma_ponderaciones />
            </div>
            <div class="tipos-lista">
                <For each=move || tipo_ids.get() key=|id| *id let(tipo_id)>
                    {move || {
                        let total = tipo_ids.get().len();
                        let idx = tipo_ids.get().iter().position(|id| *id == tipo_id).unwrap_or(0);
                        view! { <TipoCard ramo_id seccion_id tipo_id idx total /> }
                    }}
                </For>
            </div>
            <button class="btn-agregar" on:click=agregar_tipo>
                "+ Agregar tipo de evaluación"
            </button>
        </div>
    }
}
