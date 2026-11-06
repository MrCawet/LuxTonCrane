#![forbid(unsafe_code)]

//! Reçoit les mesures du téléphone et les rend à la page web. Toute requête exige le jeton.

use journal_sqlite::SqliteJournal;

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

pub struct Serveur {
    jeton: String,
    journal: SqliteJournal,
}

/// Création du serveur, avec le jeton attendu et le journal où garder les mesures.
impl Serveur {
    pub fn new(jeton: String, journal: SqliteJournal) -> Serveur {
        Serveur { jeton, journal }
    }
}

/// Réponse à une requête.
impl Serveur {
    pub fn répondre(&mut self, _requête: Requête) -> Réponse {
        Réponse {
            statut: 501,
            corps: String::new(),
        }
    }
}
