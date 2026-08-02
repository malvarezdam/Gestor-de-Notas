use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Evaluacion {
    pub id: Uuid,
    pub nombre: String,
    pub ponderacion: f64,
    pub nota: Option<f64>,
}

impl Evaluacion {
    pub fn nueva(nombre: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            nombre: nombre.into(),
            ponderacion: 0.0,
            nota: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TipoEvaluacion {
    pub id: Uuid,
    pub nombre: String,
    pub ponderacion: f64,
    pub evaluaciones: Vec<Evaluacion>,
}

impl TipoEvaluacion {
    pub fn nuevo(nombre: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            nombre: nombre.into(),
            ponderacion: 0.0,
            evaluaciones: Vec::new(),
        }
    }

    pub fn promedio(&self) -> f64 {
        self.evaluaciones
            .iter()
            .map(|e| e.nota.unwrap_or(0.0) * e.ponderacion / 100.0)
            .sum()
    }

    pub fn suma_ponderaciones(&self) -> f64 {
        self.evaluaciones.iter().map(|e| e.ponderacion).sum()
    }

    pub fn completo(&self) -> bool {
        self.evaluaciones.iter().all(|e| e.nota.is_some())
    }

    pub fn evaluacion(&self, id: Uuid) -> Option<&Evaluacion> {
        self.evaluaciones.iter().find(|e| e.id == id)
    }
    pub fn evaluacion_mut(&mut self, id: Uuid) -> Option<&mut Evaluacion> {
        self.evaluaciones.iter_mut().find(|e| e.id == id)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Seccion {
    pub id: Uuid,
    pub nombre: String,
    pub ponderacion: f64,
    pub factor_eta: f64,
    pub tipos: Vec<TipoEvaluacion>,
}

impl Seccion {
    pub fn nueva(nombre: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            nombre: nombre.into(),
            ponderacion: 0.0,
            factor_eta: 1.0,
            tipos: Vec::new(),
        }
    }

    pub fn promedio(&self) -> f64 {
        let base: f64 = self
            .tipos
            .iter()
            .map(|t| t.promedio() * t.ponderacion / 100.0)
            .sum();
        base * self.factor_eta
    }

    pub fn suma_ponderaciones(&self) -> f64 {
        self.tipos.iter().map(|t| t.ponderacion).sum()
    }

    pub fn completo(&self) -> bool {
        self.tipos.iter().all(|t| t.completo())
    }

    pub fn tipo(&self, id: Uuid) -> Option<&TipoEvaluacion> {
        self.tipos.iter().find(|t| t.id == id)
    }
    pub fn tipo_mut(&mut self, id: Uuid) -> Option<&mut TipoEvaluacion> {
        self.tipos.iter_mut().find(|t| t.id == id)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ramo {
    pub id: Uuid,
    pub nombre: String,
    pub secciones: Vec<Seccion>,
    #[serde(default)]
    pub nota_objetivo: Option<f64>,
}

impl Ramo {
    pub fn nuevo(nombre: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            nombre: nombre.into(),
            secciones: Vec::new(),
            nota_objetivo: None,
        }
    }

    pub fn promedio_final(&self) -> f64 {
        self.secciones
            .iter()
            .map(|s| s.promedio() * s.ponderacion / 100.0)
            .sum()
    }

    pub fn suma_ponderaciones(&self) -> f64 {
        self.secciones.iter().map(|s| s.ponderacion).sum()
    }

    pub fn seccion(&self, id: Uuid) -> Option<&Seccion> {
        self.secciones.iter().find(|s| s.id == id)
    }
    pub fn seccion_mut(&mut self, id: Uuid) -> Option<&mut Seccion> {
        self.secciones.iter_mut().find(|s| s.id == id)
    }

    fn evaluacion_mut_en_ramo(&mut self, evaluacion_id: Uuid) -> Option<&mut Evaluacion> {
        self.secciones
            .iter_mut()
            .flat_map(|s| s.tipos.iter_mut())
            .flat_map(|t| t.evaluaciones.iter_mut())
            .find(|e| e.id == evaluacion_id)
    }

    pub fn evaluaciones_faltantes(&self) -> Vec<Uuid> {
        self.secciones
            .iter()
            .flat_map(|s| s.tipos.iter())
            .flat_map(|t| t.evaluaciones.iter())
            .filter(|e| e.nota.is_none())
            .map(|e| e.id)
            .collect()
    }

    pub fn ubicacion_evaluacion(&self, evaluacion_id: Uuid) -> Option<(String, String, String)> {
        for seccion in &self.secciones {
            for tipo in &seccion.tipos {
                if let Some(eval) = tipo.evaluacion(evaluacion_id) {
                    return Some((seccion.nombre.clone(), tipo.nombre.clone(), eval.nombre.clone()));
                }
            }
        }
        None
    }

    pub fn nota_necesaria(&self, evaluacion_id: Uuid, objetivo: f64) -> Option<f64> {
        let mut con_cero = self.clone();
        con_cero.evaluacion_mut_en_ramo(evaluacion_id)?.nota = Some(0.0);
        let f0 = con_cero.promedio_final();

        let mut con_uno = self.clone();
        con_uno.evaluacion_mut_en_ramo(evaluacion_id)?.nota = Some(1.0);
        let f1 = con_uno.promedio_final();

        let pendiente = f1 - f0;
        if pendiente.abs() < 1e-9 {
            return None;
        }
        Some((objetivo - f0) / pendiente)
    }
}

fn default_nota_maxima() -> f64 {
    7.0
}

fn default_nota_minima() -> f64 {
    1.0
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Notebook {
    pub ramos: Vec<Ramo>,
    #[serde(default)]
    pub drive_file_id: Option<String>,
    #[serde(default = "default_nota_maxima")]
    pub nota_maxima: f64,
    #[serde(default = "default_nota_minima")]
    pub nota_minima: f64,
}

impl Default for Notebook {
    fn default() -> Self {
        Self {
            ramos: Vec::new(),
            drive_file_id: None,
            nota_maxima: default_nota_maxima(),
            nota_minima: default_nota_minima(),
        }
    }
}

impl Notebook {
    pub fn ramo(&self, id: Uuid) -> Option<&Ramo> {
        self.ramos.iter().find(|r| r.id == id)
    }
    pub fn ramo_mut(&mut self, id: Uuid) -> Option<&mut Ramo> {
        self.ramos.iter_mut().find(|r| r.id == id)
    }
    pub fn seccion(&self, ramo_id: Uuid, seccion_id: Uuid) -> Option<&Seccion> {
        self.ramo(ramo_id)?.seccion(seccion_id)
    }
    pub fn tipo(&self, ramo_id: Uuid, seccion_id: Uuid, tipo_id: Uuid) -> Option<&TipoEvaluacion> {
        self.seccion(ramo_id, seccion_id)?.tipo(tipo_id)
    }
    pub fn evaluacion(
        &self,
        ramo_id: Uuid,
        seccion_id: Uuid,
        tipo_id: Uuid,
        evaluacion_id: Uuid,
    ) -> Option<&Evaluacion> {
        self.tipo(ramo_id, seccion_id, tipo_id)?.evaluacion(evaluacion_id)
    }
    pub fn seccion_mut(&mut self, ramo_id: Uuid, seccion_id: Uuid) -> Option<&mut Seccion> {
        self.ramo_mut(ramo_id)?.seccion_mut(seccion_id)
    }
    pub fn tipo_mut(
        &mut self,
        ramo_id: Uuid,
        seccion_id: Uuid,
        tipo_id: Uuid,
    ) -> Option<&mut TipoEvaluacion> {
        self.seccion_mut(ramo_id, seccion_id)?.tipo_mut(tipo_id)
    }
    pub fn evaluacion_mut(
        &mut self,
        ramo_id: Uuid,
        seccion_id: Uuid,
        tipo_id: Uuid,
        evaluacion_id: Uuid,
    ) -> Option<&mut Evaluacion> {
        self.tipo_mut(ramo_id, seccion_id, tipo_id)?.evaluacion_mut(evaluacion_id)
    }
}

pub fn mover<T>(vec: &mut [T], idx: usize, dir: isize) {
    let nuevo = idx as isize + dir;
    if nuevo >= 0 && (nuevo as usize) < vec.len() {
        vec.swap(idx, nuevo as usize);
    }
}

pub fn distribuir_100(n: usize) -> Vec<f64> {
    if n == 0 {
        return Vec::new();
    }
    let total_centesimas = 10_000i64;
    let base = total_centesimas / n as i64;
    let resto = total_centesimas % n as i64;
    (0..n as i64)
        .map(|i| {
            let centesimas = if i < resto { base + 1 } else { base };
            centesimas as f64 / 100.0
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calcula_promedio_ascendente_con_eta() {
        let mut tipo = TipoEvaluacion::nuevo("Certamenes");
        tipo.ponderacion = 100.0;
        let mut e1 = Evaluacion::nueva("Certamen 1");
        e1.nota = Some(6.0);
        e1.ponderacion = 50.0;
        let mut e2 = Evaluacion::nueva("Certamen 2");
        e2.nota = Some(5.0);
        e2.ponderacion = 50.0;
        tipo.evaluaciones.push(e1);
        tipo.evaluaciones.push(e2);
        assert!((tipo.promedio() - 5.5).abs() < 1e-9);
        assert!(tipo.completo());

        let mut seccion = Seccion::nueva("Laboratorio");
        seccion.factor_eta = 1.1;
        seccion.tipos.push(tipo);
        assert!((seccion.promedio() - 6.05).abs() < 1e-9);

        let mut ramo = Ramo::nuevo("Calculo");
        seccion.ponderacion = 100.0;
        ramo.secciones.push(seccion);
        assert!((ramo.promedio_final() - 6.05).abs() < 1e-9);
    }

    #[test]
    fn calcula_nota_necesaria_para_evaluacion_faltante() {
        let mut tipo = TipoEvaluacion::nuevo("Certamenes");
        tipo.ponderacion = 100.0;
        let mut e1 = Evaluacion::nueva("Certamen 1");
        e1.nota = Some(6.0);
        e1.ponderacion = 50.0;
        let e2 = Evaluacion::nueva("Certamen 2");
        let e2_id = e2.id;
        let mut e2 = e2;
        e2.ponderacion = 50.0;
        tipo.evaluaciones.push(e1);
        tipo.evaluaciones.push(e2);

        let mut seccion = Seccion::nueva("Cátedra");
        seccion.ponderacion = 100.0;
        seccion.factor_eta = 1.0;
        seccion.tipos.push(tipo);

        let mut ramo = Ramo::nuevo("Calculo");
        ramo.secciones.push(seccion);

        assert_eq!(ramo.evaluaciones_faltantes(), vec![e2_id]);
        let necesaria = ramo.nota_necesaria(e2_id, 5.5).unwrap();
        assert!((necesaria - 5.0).abs() < 1e-9);
    }

    #[test]
    fn distribuye_100_en_partes_iguales_sumando_exacto() {
        let partes = distribuir_100(3);
        assert_eq!(partes, vec![33.34, 33.33, 33.33]);
        assert!((partes.iter().sum::<f64>() - 100.0).abs() < 1e-9);

        let partes4 = distribuir_100(4);
        assert_eq!(partes4, vec![25.0, 25.0, 25.0, 25.0]);

        assert_eq!(distribuir_100(0), Vec::<f64>::new());
    }
}
