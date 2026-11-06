#![forbid(unsafe_code)]

#[derive(Debug, PartialEq)]
pub struct Mesure {
    pub valeur_lux: f32,
    /// Depuis le 1er janvier 1970 à minuit (UTC).
    pub instant_ms: i64,
    /// Inconnu si le capteur de proximité n'avait encore rien donné, et pour les mesures d'avant la proximité.
    pub téléphone_couvert: Option<bool>,
}
