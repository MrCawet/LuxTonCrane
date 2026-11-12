#![forbid(unsafe_code)]

//! L'agenda du sommeil : ce que l'opérateur note de sa nuit, le matin. Les échelles sont celles de CONCEPTION.md.

use std::path::Path;

use rusqlite::{Connection, OptionalExtension, params};

const COUCHER_SOUHAITÉ_PAR_DÉFAUT: &str = "23:00";

const HEURE_MS: i64 = 60 * 60 * 1000;
const JOUR_MS: i64 = 24 * HEURE_MS;

/// D'après le consensus AASM et SRS (2015) pour les adultes : à ajuster selon la fatigue de la journée.
const OBJECTIF_DE_QUANTITÉ_DE_SOMMEIL_MS: i64 = 7 * HEURE_MS;

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

/// Le temps au lit d'une nuit, du coucher au lever : il inclut l'endormissement et les réveils.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuantitéDeSommeil {
    pub durée_ms: i64,
    /// Plus de 7 h.
    pub objectif_tenu: bool,
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
        match self {
            Fatigue::Aucune => "aucune",
            Fatigue::Légère => "legere",
            Fatigue::Gênante => "genante",
            Fatigue::Somnolence => "somnolence",
        }
    }

    pub fn depuis_le_nom(nom: &str) -> Option<Fatigue> {
        [
            Fatigue::Aucune,
            Fatigue::Légère,
            Fatigue::Gênante,
            Fatigue::Somnolence,
        ]
        .into_iter()
        .find(|niveau| niveau.nom() == nom)
    }
}

/// Les objectifs de CONCEPTION.md : les deux premiers niveaux de chaque échelle.
impl Endormissement {
    pub fn objectif_tenu(&self) -> bool {
        matches!(self, Endormissement::Immédiat | Endormissement::Calme)
    }
}

/// Les objectifs de CONCEPTION.md : les deux premiers niveaux de chaque échelle.
impl Continuité {
    pub fn objectif_tenu(&self) -> bool {
        matches!(self, Continuité::SansRéveil | Continuité::RéveilBref)
    }
}

/// Les objectifs de CONCEPTION.md : les deux premiers niveaux de chaque échelle.
impl Fatigue {
    pub fn objectif_tenu(&self) -> bool {
        matches!(self, Fatigue::Aucune | Fatigue::Légère)
    }
}

/// Le coucher qui suit le lever d'une nuit, et la quantité de sommeil, tirée des heures du coucher et du lever.
impl Nuit {
    /// Le coucher de `nuit_suivante` s'il tombe dans les 24 h après ce lever : la nuit notée le lendemain.
    /// Sinon, l'heure du coucher souhaitée le même soir : `coucher_souhaité_ms` décalé de jours entiers.
    pub fn coucher_suivant_ms(
        &self,
        nuit_suivante: Option<&Nuit>,
        coucher_souhaité_ms: i64,
    ) -> i64 {
        if let Some(nuit_suivante) = nuit_suivante {
            let écart_ms = nuit_suivante.coucher_ms - self.lever_ms;
            if écart_ms > 0 && écart_ms <= JOUR_MS {
                return nuit_suivante.coucher_ms;
            }
        }
        let écart_ms = coucher_souhaité_ms - self.lever_ms;
        let jours_de_trop = (écart_ms - 1).div_euclid(JOUR_MS);
        coucher_souhaité_ms - jours_de_trop * JOUR_MS
    }

    pub fn quantité_de_sommeil(&self) -> QuantitéDeSommeil {
        let durée_ms = self.lever_ms - self.coucher_ms;
        QuantitéDeSommeil {
            durée_ms,
            objectif_tenu: durée_ms > OBJECTIF_DE_QUANTITÉ_DE_SOMMEIL_MS,
        }
    }
}

/// Une ligne de la table `nuit`, telle que SQLite la rend.
struct NightRow {
    jour: String,
    endormissement: String,
    continuité: String,
    coucher_ms: i64,
    lever_ms: i64,
}

fn read_row(row: &rusqlite::Row) -> rusqlite::Result<NightRow> {
    Ok(NightRow {
        jour: row.get(0)?,
        endormissement: row.get(1)?,
        continuité: row.get(2)?,
        coucher_ms: row.get(3)?,
        lever_ms: row.get(4)?,
    })
}

fn nuit_de_la_ligne(ligne: NightRow) -> rusqlite::Result<Nuit> {
    let endormissement = Endormissement::depuis_le_nom(&ligne.endormissement)
        .ok_or_else(|| niveau_illisible(&ligne.endormissement))?;
    let continuité = Continuité::depuis_le_nom(&ligne.continuité)
        .ok_or_else(|| niveau_illisible(&ligne.continuité))?;
    Ok(Nuit {
        jour: ligne.jour,
        endormissement,
        continuité,
        coucher_ms: ligne.coucher_ms,
        lever_ms: ligne.lever_ms,
    })
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
            CREATE TABLE IF NOT EXISTS journee (
                jour TEXT PRIMARY KEY,
                fatigue TEXT NOT NULL
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
                "SELECT jour, endormissement, continuite, coucher_ms, lever_ms FROM nuit WHERE jour = ?1",
                params![jour],
                read_row,
            )
            .optional()?;
        ligne.map(nuit_de_la_ligne).transpose()
    }

    /// Les `nombre` dernières nuits notées jusqu'au `jour` compris, de la plus ancienne à la plus récente.
    pub fn dernières_nuits(&self, jour: &str, nombre: u32) -> rusqlite::Result<Vec<Nuit>> {
        let mut statement = self.connection.prepare(
            "SELECT jour, endormissement, continuite, coucher_ms, lever_ms FROM nuit
             WHERE jour <= ?1 ORDER BY jour DESC LIMIT ?2",
        )?;
        let lignes = statement.query_map(params![jour, nombre], read_row)?;
        let mut nuits = Vec::new();
        for ligne in lignes {
            nuits.push(nuit_de_la_ligne(ligne?)?);
        }
        nuits.reverse();
        Ok(nuits)
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
    pub fn garder_la_fatigue(&self, jour: &str, fatigue: Fatigue) -> rusqlite::Result<()> {
        self.connection.execute(
            "INSERT OR REPLACE INTO journee (jour, fatigue) VALUES (?1, ?2)",
            params![jour, fatigue.nom()],
        )?;
        Ok(())
    }

    pub fn fatigue(&self, jour: &str) -> rusqlite::Result<Option<Fatigue>> {
        let nom: Option<String> = self
            .connection
            .query_row(
                "SELECT fatigue FROM journee WHERE jour = ?1",
                params![jour],
                |row| row.get(0),
            )
            .optional()?;
        let Some(nom) = nom else {
            return Ok(None);
        };
        let fatigue = Fatigue::depuis_le_nom(&nom).ok_or_else(|| niveau_illisible(&nom))?;
        Ok(Some(fatigue))
    }
}
