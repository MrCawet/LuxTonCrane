#![forbid(unsafe_code)]

//! L'agenda du sommeil : ce que l'opérateur note de sa nuit, le matin.

use std::path::Path;

use rusqlite::Connection;

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

pub struct AgendaSqlite {
    connection: Connection,
}

impl AgendaSqlite {
    pub fn open(path: &Path) -> rusqlite::Result<AgendaSqlite> {
        let connection = Connection::open(path)?;
        Ok(AgendaSqlite { connection })
    }

    pub fn garder(&self, _nuit: &Nuit) -> rusqlite::Result<()> {
        Ok(())
    }

    pub fn nuit(&self, _jour: &str) -> rusqlite::Result<Option<Nuit>> {
        Ok(None)
    }

    pub fn coucher_souhaité(&self) -> rusqlite::Result<String> {
        Ok(String::new())
    }

    pub fn régler_le_coucher_souhaité(&self, _heure: &str) -> rusqlite::Result<()> {
        Ok(())
    }
}

/// Le nom d'un niveau, tel que la page web et la base l'écrivent.
impl Endormissement {
    pub fn nom(&self) -> &'static str {
        ""
    }

    pub fn depuis_le_nom(_nom: &str) -> Option<Endormissement> {
        None
    }
}

/// Le nom d'un niveau, tel que la page web et la base l'écrivent.
impl Continuité {
    pub fn nom(&self) -> &'static str {
        ""
    }

    pub fn depuis_le_nom(_nom: &str) -> Option<Continuité> {
        None
    }
}
