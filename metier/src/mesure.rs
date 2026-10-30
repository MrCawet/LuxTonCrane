#[derive(Debug, PartialEq)]
pub struct Mesure {
    pub valeur_lux: f32,
    /// Depuis le 1er janvier 1970 à minuit (UTC).
    pub instant_ms: i64,
}

/// Port vers lequel partent les mesures pour être gardées.
pub trait Journal {
    fn enregistrer(&mut self, mesure: Mesure);
}
