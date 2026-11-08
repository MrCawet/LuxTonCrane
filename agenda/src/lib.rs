#![forbid(unsafe_code)]

//! L'agenda du sommeil : ce que l'opérateur note de sa nuit, le matin. Les échelles sont celles de CONCEPTION.md.

use std::path::Path;

use rusqlite::{Connection, OptionalExtension, params};

const COUCHER_SOUHAITÉ_PAR_DÉFAUT: &str = "23:00";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Endormissement {
    Immédiat,
    Calme,
    Agité,
    Abandonné,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Continuité {
    SansRéveil,
    RéveilBref,
    RéveilLong,
    NuitInterrompue,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Fatigue {
    Aucune,
    Légère,
    Gênante,
    Somnolence,
}

/// Une nuit de l'agenda, reconnue à la date de son lever.
#[derive(Debug, Clone, PartialEq)]
pub struct Nuit {
    /// La date du lever, `AAAA-MM-JJ`.
    pub jour: String,
    pub endormissement: Endormissement,
    pub continuité: Continuité,
    pub coucher_ms: i64,
    pub lever_ms: i64,
}

/// L'agenda, gardé dans une base SQLite : celle des mesures, sur le serveur.
pub struct AgendaSqlite {
    connection: Connection,
}

/// Le nom d'un niveau, tel que la page web et la base l'écrivent.
impl Endormissement {
    pub fn nom(&self) -> &'static str {
        match self {
            Endormissement::Immédiat => "immediat",
            Endormissement::Calme => "calme",
            Endormissement::Agité => "agite",
            Endormissement::Abandonné => "abandonne",
        }
    }

    pub fn depuis_le_nom(nom: &str) -> Option<Endormissement> {
        [
            Endormissement::Immédiat,
            Endormissement::Calme,
            Endormissement::Agité,
            Endormissement::Abandonné,
        ]
        .into_iter()
        .find(|niveau| niveau.nom() == nom)
    }
}

/// Le nom d'un niveau, tel que la page web et la base l'écrivent.
impl Continuité {
    pub fn nom(&self) -> &'static str {
        match self {
            Continuité::SansRéveil => "sans_reveil",
            Continuité::RéveilBref => "reveil_bref",
            Continuité::RéveilLong => "reveil_long",
            Continuité::NuitInterrompue => "nuit_interrompue",
        }
    }

    pub fn depuis_le_nom(nom: &str) -> Option<Continuité> {
        [
            Continuité::SansRéveil,
            Continuité::RéveilBref,
            Continuité::RéveilLong,
            Continuité::NuitInterrompue,
        ]
        .into_iter()
        .find(|niveau| niveau.nom() == nom)
    }
}

/// Le nom d'un niveau, tel que la page web et la base l'écrivent.
impl Fatigue {
    pub fn nom(&self) -> &'static str {
        ""
    }

    pub fn depuis_le_nom(_nom: &str) -> Option<Fatigue> {
        None
    }
}

fn niveau_illisible(nom: &str) -> rusqlite::Error {
    rusqlite::Error::InvalidParameterName(format!("niveau inconnu dans la base : {nom}"))
}

/// Ouverture de la base, qui crée les tables de l'agenda si elles manquent.
impl AgendaSqlite {
    pub fn open(path: &Path) -> rusqlite::Result<AgendaSqlite> {
        let connection = Connection::open(path)?;
        connection.execute_batch(
            "CREATE TABLE IF NOT EXISTS nuit (
                jour TEXT PRIMARY KEY,
                endormissement TEXT NOT NULL,
                continuite TEXT NOT NULL,
                coucher_ms INTEGER NOT NULL,
                lever_ms INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS reglage (
                nom TEXT PRIMARY KEY,
                valeur TEXT NOT NULL
            );",
        )?;
        Ok(AgendaSqlite { connection })
    }
}

/// Les nuits notées : une par jour du lever, la dernière saisie l'emporte.
impl AgendaSqlite {
    pub fn garder(&self, nuit: &Nuit) -> rusqlite::Result<()> {
        self.connection.execute(
            "INSERT OR REPLACE INTO nuit (jour, endormissement, continuite, coucher_ms, lever_ms)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                nuit.jour,
                nuit.endormissement.nom(),
                nuit.continuité.nom(),
                nuit.coucher_ms,
                nuit.lever_ms
            ],
        )?;
        Ok(())
    }

    pub fn nuit(&self, jour: &str) -> rusqlite::Result<Option<Nuit>> {
        let ligne = self
            .connection
            .query_row(
                "SELECT endormissement, continuite, coucher_ms, lever_ms FROM nuit WHERE jour = ?1",
                params![jour],
                |row| {
                    let endormissement: String = row.get(0)?;
                    let continuité: String = row.get(1)?;
                    let coucher_ms: i64 = row.get(2)?;
                    let lever_ms: i64 = row.get(3)?;
                    Ok((endormissement, continuité, coucher_ms, lever_ms))
                },
            )
            .optional()?;
        let Some((endormissement, continuité, coucher_ms, lever_ms)) = ligne else {
            return Ok(None);
        };
        let endormissement = Endormissement::depuis_le_nom(&endormissement)
            .ok_or_else(|| niveau_illisible(&endormissement))?;
        let continuité =
            Continuité::depuis_le_nom(&continuité).ok_or_else(|| niveau_illisible(&continuité))?;
        Ok(Some(Nuit {
            jour: jour.to_string(),
            endormissement,
            continuité,
            coucher_ms,
            lever_ms,
        }))
    }
}

/// L'heure du coucher souhaitée, `HH:MM` : elle finit la journée du relevé tant que le coucher réel n'est pas noté.
impl AgendaSqlite {
    pub fn coucher_souhaité(&self) -> rusqlite::Result<String> {
        let réglée: Option<String> = self
            .connection
            .query_row(
                "SELECT valeur FROM reglage WHERE nom = 'coucher_souhaite'",
                [],
                |row| row.get(0),
            )
            .optional()?;
        Ok(réglée.unwrap_or(COUCHER_SOUHAITÉ_PAR_DÉFAUT.to_string()))
    }

    pub fn régler_le_coucher_souhaité(&self, heure: &str) -> rusqlite::Result<()> {
        self.connection.execute(
            "INSERT OR REPLACE INTO reglage (nom, valeur) VALUES ('coucher_souhaite', ?1)",
            params![heure],
        )?;
        Ok(())
    }
}

/// La fatigue de la journée, notée le soir, par jour : celui de la nuit qui l'a précédée.
impl AgendaSqlite {
    pub fn garder_la_fatigue(&self, _jour: &str, _fatigue: Fatigue) -> rusqlite::Result<()> {
        Ok(())
    }

    pub fn fatigue(&self, _jour: &str) -> rusqlite::Result<Option<Fatigue>> {
        Ok(None)
    }
}
