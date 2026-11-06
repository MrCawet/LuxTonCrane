#![forbid(unsafe_code)]

//! Ce que Kotlin peut appeler du métier. Les noms qui traversent le pont n'ont pas d'accents.

use std::fmt;
use std::path::Path;
use std::sync::{Arc, Mutex};

use journal::ajouter;
use journal_sqlite::SqliteJournal;
use mesure::Mesure;
use releve::{
    HeuresDuCoucherEtDuLever, LumièreDeLaJournée, LumièreDuMatin, LumièreÀÉviter, RelevéDuJour,
    relevé_du_jour,
};

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

/// La lumière du soir ou de la nuit, vue de Kotlin.
#[derive(uniffi::Record)]
pub struct LumiereAEviter {
    pub temps_au_dessus_du_seuil_ms: i64,
    pub valeur_la_plus_haute_lux: Option<f32>,
    pub objectif_tenu: bool,
    pub couverture: f64,
}

impl From<LumièreÀÉviter> for LumiereAEviter {
    fn from(lumière: LumièreÀÉviter) -> LumiereAEviter {
        LumiereAEviter {
            temps_au_dessus_du_seuil_ms: lumière.temps_au_dessus_du_seuil_ms,
            valeur_la_plus_haute_lux: lumière.valeur_la_plus_haute_lux,
            objectif_tenu: lumière.objectif_tenu,
            couverture: lumière.couverture,
        }
    }
}

/// La lumière du matin, vue de Kotlin.
#[derive(uniffi::Record)]
pub struct LumiereDuMatin {
    pub temps_a_10_000_lx_ou_plus_ms: i64,
    pub objectif_tenu: bool,
    pub couverture: f64,
}

impl From<LumièreDuMatin> for LumiereDuMatin {
    fn from(lumière: LumièreDuMatin) -> LumiereDuMatin {
        LumiereDuMatin {
            temps_a_10_000_lx_ou_plus_ms: lumière.temps_à_10_000_lx_ou_plus_ms,
            objectif_tenu: lumière.objectif_tenu,
            couverture: lumière.couverture,
        }
    }
}

/// La lumière de la journée, vue de Kotlin.
#[derive(uniffi::Record)]
pub struct LumiereDeLaJournee {
    pub part_du_temps_mesure_a_250_lx_ou_plus: Option<f64>,
    pub objectif_tenu: bool,
    pub couverture: f64,
}

impl From<LumièreDeLaJournée> for LumiereDeLaJournee {
    fn from(lumière: LumièreDeLaJournée) -> LumiereDeLaJournee {
        LumiereDeLaJournee {
            part_du_temps_mesure_a_250_lx_ou_plus: lumière.part_du_temps_mesuré_à_250_lx_ou_plus,
            objectif_tenu: lumière.objectif_tenu,
            couverture: lumière.couverture,
        }
    }
}

/// Le relevé du jour, vu de Kotlin.
#[derive(uniffi::Record)]
pub struct ReleveDuJour {
    pub lumiere_du_soir: LumiereAEviter,
    pub lumiere_de_la_nuit: LumiereAEviter,
    pub lumiere_du_matin: LumiereDuMatin,
    pub lumiere_de_la_journee: LumiereDeLaJournee,
}

impl From<RelevéDuJour> for ReleveDuJour {
    fn from(relevé: RelevéDuJour) -> ReleveDuJour {
        ReleveDuJour {
            lumiere_du_soir: LumiereAEviter::from(relevé.lumière_du_soir),
            lumiere_de_la_nuit: LumiereAEviter::from(relevé.lumière_de_la_nuit),
            lumiere_du_matin: LumiereDuMatin::from(relevé.lumière_du_matin),
            lumiere_de_la_journee: LumiereDeLaJournee::from(relevé.lumière_de_la_journée),
        }
    }
}

/// Une mesure du journal, vue de Kotlin.
#[derive(uniffi::Record)]
pub struct MesureDuJournal {
    pub valeur_lux: f32,
    pub instant_ms: i64,
}

/// Le journal SQLite, vu de Kotlin.
#[derive(uniffi::Object)]
pub struct Journal {
    // UniFFI partage l'objet entre les fils d'exécution de Kotlin : le verrou est exigé.
    sqlite: Mutex<SqliteJournal>,
}

/// Ouverture du journal, ajout des mesures et relevé du jour.
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

    /// `coucher_souhaite_ms` : l'heure du coucher souhaitée, qui fixe la fin de la journée.
    pub fn releve_du_jour(
        &self,
        coucher_ms: i64,
        lever_ms: i64,
        coucher_souhaite_ms: i64,
    ) -> Result<ReleveDuJour, EchecPont> {
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
        let heures = HeuresDuCoucherEtDuLever {
            coucher_ms,
            lever_ms,
            coucher_suivant_ms: coucher_souhaite_ms,
        };
        let relevé = relevé_du_jour(&heures, &historique);
        Ok(ReleveDuJour::from(relevé))
    }

    /// `dernier_instant_recu_ms` : l'instant de la dernière mesure que le serveur a reçue, s'il en a une.
    pub fn mesures_apres(
        &self,
        dernier_instant_recu_ms: Option<i64>,
    ) -> Result<Vec<MesureDuJournal>, EchecPont> {
        let _ = dernier_instant_recu_ms;
        Ok(Vec::new())
    }
}
