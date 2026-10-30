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
        }
    }
}

/// Fait de l'échec une erreur Rust standard, exigée elle aussi par UniFFI.
impl std::error::Error for EchecPont {}

/// Le journal SQLite, vu de Kotlin.
#[derive(uniffi::Object)]
pub struct Journal {
    // UniFFI partage l'objet entre les fils d'exécution de Kotlin : le verrou est exigé.
    sqlite: Mutex<SqliteJournal>,
}

/// Ouverture du journal et ajout des mesures.
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

    pub fn ajouter(&self, valeur_lux: f32, instant_ms: i64) -> Result<(), EchecPont> {
        let mesure = Mesure {
            valeur_lux,
            instant_ms,
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
}
