pub fn redondear2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0 + 0.0
}

pub fn formatear(v: f64) -> String {
    format!("{:.2}", redondear2(v))
}

pub fn formatear_entero(v: f64) -> String {
    format!("{:.0}", v + 0.0)
}

pub fn parsear_numero(s: &str) -> Option<f64> {
    s.trim().replace(',', ".").parse::<f64>().ok()
}
