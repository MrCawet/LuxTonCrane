#![forbid(unsafe_code)]

use std::path::Path;

use journal::{Journal, ÉchecDEnregistrement};
use mesure::Mesure;
use rusqlite::{Connection, params};

pub struct SqliteJournal {
    connection: Connection,
}

/// Ouverture de la base, qui crée la table des mesures si elle manque.
impl SqliteJournal {
    pub fn open(path: &Path) -> rusqlite::Result<SqliteJournal> {
        let connection = Connection::open(path)?;
        connection.execute_batch(
            "CREATE TABLE IF NOT EXISTS mesure (
                valeur_lux REAL NOT NULL,
                instant_ms INTEGER NOT NULL
            )",
        )?;
        Ok(SqliteJournal { connection })
    }
}

/// Enregistrement d'une mesure, une ligne par mesure.
impl Journal for SqliteJournal {
    fn enregistrer(&mut self, mesure: Mesure) -> Result<(), ÉchecDEnregistrement> {
        let insertion = self.connection.execute(
            "INSERT INTO mesure (valeur_lux, instant_ms) VALUES (?1, ?2)",
            params![mesure.valeur_lux, mesure.instant_ms],
        );
        match insertion {
            Ok(_) => Ok(()),
            Err(error) => Err(ÉchecDEnregistrement {
                raison: error.to_string(),
            }),
        }
    }
}
