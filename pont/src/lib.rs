#![forbid(unsafe_code)]

//! Ce que Kotlin peut appeler du métier. Les noms qui traversent le pont n'ont pas d'accents.

use std::fmt;
use std::path::Path;
use std::sync::{Arc, Mutex};

use journal::ajouter;
use journal_sqlite::SqliteJournal;
use mesure::Mesure;

uniffi::setup_scaffolding!();

#[derive(Debug, uniffi::Error)]
pub enum EchecPont {
    Ouverture { raison: String },
    Enregistrement { raison: String },
    Lecture { raison: String },
}

/// Message lisible de l'échec, exigé par UniFFI pour en faire une exception Kotlin.
impl fmt::Display for EchecPont {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EchecPont::Ouverture { raison } => {
                write!(formatter, "ouverture du journal impossible : {raison}")
            }
            EchecPont::Enregistrement { raison } => {
                write!(formatter, "enregistrement impossible : {raison}")
            }
            EchecPont::Lecture { raison } => {
                write!(formatter, "lecture du journal impossible : {raison}")
            }
        }
    }
}

/// Fait de l'échec une erreur Rust standard, exigée elle aussi par UniFFI.
impl std::error::Error for EchecPont {}

/// Une mesure du journal, vue de Kotlin.
#[derive(uniffi::Record)]
pub struct MesureDuJournal {
    pub valeur_lux: f32,
    pub instant_ms: i64,
    pub telephone_couvert: Option<bool>,
}

/// Le journal SQLite, vu de Kotlin.
#[derive(uniffi::Object)]
pub struct Journal {
    // UniFFI partage l'objet entre les fils d'exécution de Kotlin : le verrou est exigé.
    sqlite: Mutex<SqliteJournal>,
}

/// Ouverture du journal, ajout des mesures et lecture de celles à envoyer au serveur.
#[uniffi::export]
impl Journal {
    #[uniffi::constructor]
    pub fn ouvrir(chemin: String) -> Result<Arc<Journal>, EchecPont> {
        let ouverture = SqliteJournal::open(Path::new(&chemin));
        match ouverture {
            Ok(sqlite) => Ok(Arc::new(Journal {
                sqlite: Mutex::new(sqlite),
            })),
            Err(error) => Err(EchecPont::Ouverture {
                raison: error.to_string(),
            }),
        }
    }

    /// `telephone_couvert` : aucun si le capteur de proximité n'a encore rien donné.
    pub fn ajouter(
        &self,
        valeur_lux: f32,
        instant_ms: i64,
        telephone_couvert: Option<bool>,
    ) -> Result<(), EchecPont> {
        let mesure = Mesure {
            valeur_lux,
            instant_ms,
            téléphone_couvert: telephone_couvert,
        };
        let mut sqlite = match self.sqlite.lock() {
            Ok(sqlite) => sqlite,
            Err(error) => {
                return Err(EchecPont::Enregistrement {
                    raison: error.to_string(),
                });
            }
        };
        match ajouter(mesure, &mut *sqlite) {
            Ok(()) => Ok(()),
            Err(échec) => Err(EchecPont::Enregistrement {
                raison: échec.raison,
            }),
        }
    }

    /// `dernier_instant_recu_ms` : l'instant de la dernière mesure que le serveur a reçue, s'il en a une.
    pub fn mesures_apres(
        &self,
        dernier_instant_recu_ms: Option<i64>,
    ) -> Result<Vec<MesureDuJournal>, EchecPont> {
        let sqlite = match self.sqlite.lock() {
            Ok(sqlite) => sqlite,
            Err(error) => {
                return Err(EchecPont::Lecture {
                    raison: error.to_string(),
                });
            }
        };
        let historique = match sqlite.historique() {
            Ok(historique) => historique,
            Err(error) => {
                return Err(EchecPont::Lecture {
                    raison: error.to_string(),
                });
            }
        };
        let mut à_envoyer = Vec::new();
        for mesure in historique {
            let déjà_reçue = match dernier_instant_recu_ms {
                Some(dernier_instant_reçu_ms) => mesure.instant_ms <= dernier_instant_reçu_ms,
                None => false,
            };
            if !déjà_reçue {
                à_envoyer.push(MesureDuJournal {
                    valeur_lux: mesure.valeur_lux,
                    instant_ms: mesure.instant_ms,
                    telephone_couvert: mesure.téléphone_couvert,
                });
            }
        }
        Ok(à_envoyer)
    }
}
