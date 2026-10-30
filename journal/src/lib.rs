#![forbid(unsafe_code)]

use mesure::Mesure;

/// Port vers lequel partent les mesures pour être gardées.
pub trait Journal {
    fn enregistrer(&mut self, mesure: Mesure);
}
