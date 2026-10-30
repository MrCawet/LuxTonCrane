#![forbid(unsafe_code)]

#[derive(Debug, PartialEq)]
pub struct Mesure {
    pub valeur_lux: f32,
    /// Depuis le 1er janvier 1970 à minuit (UTC).
    pub instant_ms: i64,
}
