use crate::components::editable_opcional::EditableOpcional;
use crate::components::editable_text::EditableText;
use crate::components::seccion_column::SeccionColumn;
use crate::components::weight_badge::WeightBadge;
use crate::models::{distribuir_100, Seccion};
use crate::state::use_app_state;
use crate::utils::formatear;
use leptos::prelude::*;

#[derive(Clone, PartialEq)]
enum EstadoCalculadora {
    NecesitaNota { necesaria: f64, ubicacion: (String, String, String) },
    SinPonderacion { ubicacion: (String, String, String) },
}

#[component]
pub fn RamoView() -> impl IntoView {
    let state = use_app_state();

    view! {
        <main class="ramo-view">
            {move || match state.ramo_seleccionado.get() {
                None => view! {
                    <div class="estado-vacio">
                        <p>"Selecciona un ramo en la barra lateral, o agrega uno nuevo para comenzar."</p>
                    </div>
                }
                .into_any(),
                Some(ramo_id) => {
                    let existe = state.notebook.with_untracked(|nb| nb.ramo(ramo_id).is_some());
                    if !existe {
                        return view! {
                            <div class="estado-vacio">
                                <p>"Selecciona un ramo en la barra lateral."</p>
                            </div>
                        }
                        .into_any();
                    }
                    let mostrar_final = RwSignal::new(false);

                    let nombre = Signal::derive(move || {
                        state.notebook.with(|nb| nb.ramo(ramo_id).map(|r| r.nombre.clone())).unwrap_or_default()
                    });
                    let promedio_final = Signal::derive(move || {
                        state.notebook.with(|nb| nb.ramo(ramo_id).map(|r| r.promedio_final())).unwrap_or(0.0)
                    });
                    let suma_ponderaciones = Signal::derive(move || {
                        state
                            .notebook
                            .with(|nb| nb.ramo(ramo_id).map(|r| r.suma_ponderaciones()))
                            .unwrap_or(0.0)
                    });
                    let seccion_ids = Memo::new(move |_| {
                        state
                            .notebook
                            .with(|nb| nb.ramo(ramo_id).map(|r| r.secciones.iter().map(|s| s.id).collect::<Vec<_>>()))
                            .unwrap_or_default()
                    });
                    let nota_objetivo = Signal::derive(move || {
                        state.notebook.with(|nb| nb.ramo(ramo_id).and_then(|r| r.nota_objetivo))
                    });
                    let faltantes = Signal::derive(move || {
                        state
                            .notebook
                            .with(|nb| nb.ramo(ramo_id).map(|r| r.evaluaciones_faltantes()))
                            .unwrap_or_default()
                    });
                    let nota_maxima = Signal::derive(move || state.notebook.with(|nb| nb.nota_maxima));
                    let nota_minima = Signal::derive(move || state.notebook.with(|nb| nb.nota_minima));
                    let calculadora = Signal::derive(move || -> Option<EstadoCalculadora> {
                        let f = faltantes.get();
                        if f.len() != 1 {
                            return None;
                        }
                        let objetivo = nota_objetivo.get()?;
                        let eval_id = f[0];
                        state.notebook.with(|nb| {
                            let r = nb.ramo(ramo_id)?;
                            let ubicacion = r.ubicacion_evaluacion(eval_id)?;
                            Some(match r.nota_necesaria(eval_id, objetivo) {
                                Some(necesaria) => EstadoCalculadora::NecesitaNota { necesaria, ubicacion },
                                None => EstadoCalculadora::SinPonderacion { ubicacion },
                            })
                        })
                    });

                    let set_nombre = move |v: String| {
                        state.notebook.update(|nb| {
                            if let Some(r) = nb.ramo_mut(ramo_id) {
                                r.nombre = v;
                            }
                        });
                    };
                    let set_nota_objetivo = move |v: Option<f64>| {
                        state.notebook.update(|nb| {
                            if let Some(r) = nb.ramo_mut(ramo_id) {
                                r.nota_objetivo = v;
                            }
                        });
                    };
                    let agregar_seccion = move |_| {
                        state.notebook.update(|nb| {
                            if let Some(r) = nb.ramo_mut(ramo_id) {
                                r.secciones.push(Seccion::nueva("Nueva sección"));
                            }
                        });
                    };
                    let promediar_ramo = move |_| {
                        state.notebook.update(|nb| {
                            if let Some(r) = nb.ramo_mut(ramo_id) {
                                let partes = distribuir_100(r.secciones.len());
                                for (s, p) in r.secciones.iter_mut().zip(partes) {
                                    s.ponderacion = p;
                                }
                            }
                        });
                        mostrar_final.update(|v| *v = !*v);
                    };

                    view! {
                        <div class="ramo-contenido">
                            <header class="ramo-header">
                                <EditableText value=nombre on_change=set_nombre class="nombre-ramo" placeholder="Nombre del ramo" />
                                <div class="ramo-header-derecha">
                                    <WeightBadge suma=suma_ponderaciones />
                                    <button class="btn-promediar" title="Reparte el 100% en partes iguales entre las secciones" on:click=promediar_ramo>
                                        "Promediar ramo"
                                    </button>
                                    <Show when=move || mostrar_final.get()>
                                        <span class="resultado-ramo">"Nota final: " <strong>{move || formatear(promedio_final.get())}</strong></span>
                                    </Show>
                                </div>
                            </header>
                            <div class="ramo-objetivo">
                                <label>
                                    "Nota objetivo"
                                    <EditableOpcional
                                        value=nota_objetivo
                                        on_change=set_nota_objetivo
                                        class="nota-objetivo"
                                        placeholder="ej: 5.5"
                                    />
                                </label>
                                {move || {
                                    let n_faltan = faltantes.get().len();
                                    if n_faltan == 0 {
                                        view! { <span class="objetivo-hint">"Todas las notas están ingresadas."</span> }.into_any()
                                    } else if n_faltan == 1 && nota_objetivo.get().is_none() {
                                        view! {
                                            <span class="objetivo-hint">"Te falta 1 nota: define tu nota objetivo para saber cuánto necesitas."</span>
                                        }
                                        .into_any()
                                    } else if n_faltan == 1 {
                                        ().into_any()
                                    } else {
                                        view! { <span class="objetivo-hint">{format!("Te faltan {n_faltan} notas por ingresar.")}</span> }.into_any()
                                    }
                                }}
                            </div>
                            <Show when=move || calculadora.get().is_some()>
                                {move || {
                                    let max = nota_maxima.get();
                                    let min = nota_minima.get();
                                    match calculadora.get() {
                                        Some(EstadoCalculadora::NecesitaNota { necesaria, ubicacion: (seccion, tipo, eval) }) => {
                                            let mensaje = if necesaria > max {
                                                format!(
                                                    "No es posible alcanzar tu objetivo: necesitarías un {} en \"{eval}\" ({seccion} · {tipo}), y tu nota máxima es {}.",
                                                    formatear(necesaria), formatear(max)
                                                )
                                            } else if necesaria <= min {
                                                format!("¡Tu objetivo ya está asegurado! No importa qué te saques en \"{eval}\" ({seccion} · {tipo}).")
                                            } else {
                                                format!(
                                                    "Necesitas un {} en \"{eval}\" ({seccion} · {tipo}) para lograr tu nota objetivo.",
                                                    formatear(necesaria)
                                                )
                                            };
                                            view! { <div class="calculadora-objetivo">{mensaje}</div> }.into_any()
                                        }
                                        Some(EstadoCalculadora::SinPonderacion { ubicacion: (seccion, tipo, eval) }) => {
                                            view! {
                                                <div class="calculadora-objetivo">
                                                    {format!("\"{eval}\" ({seccion} · {tipo}) no pondera en tu nota final (ponderación 0), así que no afecta tu objetivo.")}
                                                </div>
                                            }
                                            .into_any()
                                        }
                                        None => ().into_any(),
                                    }
                                }}
                            </Show>
                            <div class="secciones-row">
                                <For each=move || seccion_ids.get() key=|id| *id let(seccion_id)>
                                    {move || {
                                        let total = seccion_ids.get().len();
                                        let idx = seccion_ids.get().iter().position(|id| *id == seccion_id).unwrap_or(0);
                                        view! { <SeccionColumn ramo_id seccion_id idx total /> }
                                    }}
                                </For>
                                <button class="btn-agregar-columna" on:click=agregar_seccion>
                                    "+ Agregar sección"
                                </button>
                            </div>
                        </div>
                    }
                    .into_any()
                }
            }}
        </main>
    }
}
