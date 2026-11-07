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
    /// Une base créée avant la proximité n'a pas sa colonne : ses mesures gardent une proximité inconnue.
    fn add_missing_proximity_column(connection: &Connection) -> rusqlite::Result<()> {
        let proximity_column_count: i64 = connection.query_row(
            "SELECT COUNT(*) FROM pragma_table_info('mesure') WHERE name = 'telephone_couvert'",
            [],
            |row| row.get(0),
        )?;
        if proximity_column_count == 0 {
            connection.execute_batch("ALTER TABLE mesure ADD COLUMN telephone_couvert INTEGER")?;
        }
        Ok(())
    }

    pub fn open(path: &Path) -> rusqlite::Result<SqliteJournal> {
        let connection = Connection::open(path)?;
        connection.execute_batch(
            "CREATE TABLE IF NOT EXISTS mesure (
                valeur_lux REAL NOT NULL,
                instant_ms INTEGER NOT NULL,
                telephone_couvert INTEGER
            )",
        )?;
        SqliteJournal::add_missing_proximity_column(&connection)?;
        Ok(SqliteJournal { connection })
    }
}

/// Lecture de toutes les mesures enregistrées.
impl SqliteJournal {
    pub fn historique(&self) -> rusqlite::Result<Vec<Mesure>> {
        let mut statement = self.connection.prepare(
            "SELECT valeur_lux, instant_ms, telephone_couvert FROM mesure ORDER BY instant_ms",
        )?;
        let rows = statement.query_map([], |row| {
            Ok(Mesure {
                valeur_lux: row.get(0)?,
                instant_ms: row.get(1)?,
                téléphone_couvert: row.get(2)?,
            })
        })?;
        rows.collect()
    }
}

/// Enregistrement d'une mesure, une ligne par mesure.
impl Journal for SqliteJournal {
    fn enregistrer(&mut self, mesure: Mesure) -> Result<(), ÉchecDEnregistrement> {
        let insertion = self.connection.execute(
            "INSERT INTO mesure (valeur_lux, instant_ms, telephone_couvert) VALUES (?1, ?2, ?3)",
            params![
                mesure.valeur_lux,
                mesure.instant_ms,
                mesure.téléphone_couvert
            ],
        );
        match insertion {
            Ok(_) => Ok(()),
            Err(error) => Err(ÉchecDEnregistrement {
                raison: error.to_string(),
            }),
        }
    }
}
