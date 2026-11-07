#![forbid(unsafe_code)]

//! Reçoit les mesures du téléphone et les rend à la page web. Toute requête exige le jeton.

use std::collections::HashSet;

use journal::{Journal, ajouter};
use journal_sqlite::SqliteJournal;
use mesure::Mesure;
use serde::{Deserialize, Serialize};

pub enum Méthode {
    Get,
    Post,
    Autre,
}

pub struct Requête<'a> {
    pub méthode: Méthode,
    pub chemin: &'a str,
    /// La valeur de l'en-tête `Authorization`, s'il y en a un.
    pub autorisation: Option<&'a str>,
    pub corps: &'a str,
}

#[derive(Debug)]
pub struct Réponse {
    pub statut: u16,
    pub corps: String,
}

/// Une mesure telle que la page web la reçoit.
#[derive(Serialize)]
struct MesureJson {
    valeur_lux: f32,
    instant_ms: i64,
}

/// Une mesure telle que le téléphone l'envoie. Sans `telephone_couvert`, la proximité est inconnue.
#[derive(Deserialize)]
struct MesureReçueJson {
    valeur_lux: f32,
    instant_ms: i64,
    telephone_couvert: Option<bool>,
}

#[derive(Serialize)]
struct RésuméJson {
    nombre: usize,
    derniere: Option<MesureJson>,
}

pub struct Serveur {
    jeton: String,
    journal: SqliteJournal,
}

/// Réponses sans contenu ou en échec.
impl Réponse {
    fn vide(statut: u16) -> Réponse {
        Réponse {
            statut,
            corps: String::new(),
        }
    }
}

/// Création du serveur, avec le jeton attendu et le journal où garder les mesures.
impl Serveur {
    pub fn new(jeton: String, journal: SqliteJournal) -> Serveur {
        Serveur { jeton, journal }
    }
}

/// Lecture et enregistrement des mesures.
impl Serveur {
    fn jeton_valable(&self, autorisation: Option<&str>) -> bool {
        let attendu = format!("Bearer {}", self.jeton);
        autorisation == Some(attendu.as_str())
    }

    fn résumé(&self) -> Réponse {
        let historique = match self.journal.historique() {
            Ok(historique) => historique,
            Err(_) => return Réponse::vide(500),
        };
        let dernière = historique.last().map(|mesure| MesureJson {
            valeur_lux: mesure.valeur_lux,
            instant_ms: mesure.instant_ms,
        });
        let résumé = RésuméJson {
            nombre: historique.len(),
            derniere: dernière,
        };
        match serde_json::to_string(&résumé) {
            Ok(corps) => Réponse { statut: 200, corps },
            Err(_) => Réponse::vide(500),
        }
    }

    fn réception(&mut self, corps: &str) -> Réponse {
        let reçues: Vec<MesureReçueJson> = match serde_json::from_str(corps) {
            Ok(reçues) => reçues,
            Err(_) => return Réponse::vide(400),
        };
        // Le téléphone renvoie un lot dont il n'a pas eu la réponse : une mesure se reconnaît à son instant.
        let mut instants_déjà_reçus: HashSet<i64> = match self.journal.historique() {
            Ok(historique) => historique.iter().map(|mesure| mesure.instant_ms).collect(),
            Err(_) => return Réponse::vide(500),
        };
        for reçue in reçues {
            let nouvelle = instants_déjà_reçus.insert(reçue.instant_ms);
            if !nouvelle {
                continue;
            }
            let mesure = Mesure {
                valeur_lux: reçue.valeur_lux,
                instant_ms: reçue.instant_ms,
                téléphone_couvert: reçue.telephone_couvert,
            };
            let journal: &mut dyn Journal = &mut self.journal;
            if ajouter(mesure, journal).is_err() {
                return Réponse::vide(500);
            }
        }
        Réponse::vide(204)
    }
}

/// Réponse à une requête.
impl Serveur {
    pub fn répondre(&mut self, requête: Requête) -> Réponse {
        if !self.jeton_valable(requête.autorisation) {
            return Réponse::vide(401);
        }
        match (requête.méthode, requête.chemin) {
            (Méthode::Get, "/api/mesures") => self.résumé(),
            (Méthode::Post, "/api/mesures") => self.réception(requête.corps),
            _ => Réponse::vide(404),
        }
    }
}
