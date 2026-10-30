#![forbid(unsafe_code)]

use mesure::Mesure;

#[derive(Debug, PartialEq)]
pub struct ÉchecDEnregistrement {
    pub raison: String,
}

/// Port vers lequel partent les mesures pour être gardées.
pub trait Journal {
    fn enregistrer(&mut self, mesure: Mesure) -> Result<(), ÉchecDEnregistrement>;
}

pub fn ajouter(mesure: Mesure, journal: &mut dyn Journal) -> Result<(), ÉchecDEnregistrement> {
    journal.enregistrer(mesure)
}
