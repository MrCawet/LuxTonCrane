#![forbid(unsafe_code)]

//! Reçoit les mesures du téléphone et les rend à la page web. Toute requête exige le jeton.

use std::collections::HashSet;

use agenda::{AgendaSqlite, Continuité, Endormissement, Nuit};
use journal::{Journal, ajouter};
use journal_sqlite::SqliteJournal;
use mesure::Mesure;
use releve::{
    HeuresDuCoucherEtDuLever, LumièreDeLaJournée, LumièreDuMatin, LumièreÀÉviter, RelevéDuJour,
    relevé_du_jour,
};
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

/// La lumière du soir ou de la nuit, telle que la page web la reçoit.
#[derive(Serialize)]
struct LumièreÀÉviterJson {
    temps_au_dessus_du_seuil_ms: i64,
    valeur_la_plus_haute_lux: Option<f32>,
    objectif_tenu: bool,
    couverture: f64,
}

#[derive(Serialize)]
struct LumièreDuMatinJson {
    temps_a_10_000_lx_ou_plus_ms: i64,
    objectif_tenu: bool,
    couverture: f64,
}

#[derive(Serialize)]
struct LumièreDeLaJournéeJson {
    part_du_temps_mesure_a_250_lx_ou_plus: Option<f64>,
    objectif_tenu: bool,
    couverture: f64,
}

/// Le relevé du jour, tel que la page web le reçoit : les noms des champs sont sans accents.
#[derive(Serialize)]
struct RelevéDuJourJson {
    lumiere_du_soir: LumièreÀÉviterJson,
    lumiere_de_la_nuit: LumièreÀÉviterJson,
    lumiere_du_matin: LumièreDuMatinJson,
    lumiere_de_la_journee: LumièreDeLaJournéeJson,
}

impl From<LumièreÀÉviter> for LumièreÀÉviterJson {
    fn from(lumière: LumièreÀÉviter) -> LumièreÀÉviterJson {
        LumièreÀÉviterJson {
            temps_au_dessus_du_seuil_ms: lumière.temps_au_dessus_du_seuil_ms,
            valeur_la_plus_haute_lux: lumière.valeur_la_plus_haute_lux,
            objectif_tenu: lumière.objectif_tenu,
            couverture: lumière.couverture,
        }
    }
}

impl From<LumièreDuMatin> for LumièreDuMatinJson {
    fn from(lumière: LumièreDuMatin) -> LumièreDuMatinJson {
        LumièreDuMatinJson {
            temps_a_10_000_lx_ou_plus_ms: lumière.temps_à_10_000_lx_ou_plus_ms,
            objectif_tenu: lumière.objectif_tenu,
            couverture: lumière.couverture,
        }
    }
}

impl From<LumièreDeLaJournée> for LumièreDeLaJournéeJson {
    fn from(lumière: LumièreDeLaJournée) -> LumièreDeLaJournéeJson {
        LumièreDeLaJournéeJson {
            part_du_temps_mesure_a_250_lx_ou_plus: lumière.part_du_temps_mesuré_à_250_lx_ou_plus,
            objectif_tenu: lumière.objectif_tenu,
            couverture: lumière.couverture,
        }
    }
}

impl From<RelevéDuJour> for RelevéDuJourJson {
    fn from(relevé: RelevéDuJour) -> RelevéDuJourJson {
        RelevéDuJourJson {
            lumiere_du_soir: LumièreÀÉviterJson::from(relevé.lumière_du_soir),
            lumiere_de_la_nuit: LumièreÀÉviterJson::from(relevé.lumière_de_la_nuit),
            lumiere_du_matin: LumièreDuMatinJson::from(relevé.lumière_du_matin),
            lumiere_de_la_journee: LumièreDeLaJournéeJson::from(relevé.lumière_de_la_journée),
        }
    }
}

/// La valeur entière du paramètre `nom` dans `paramètres` (`a=1&b=2`), si elle y est.
fn paramètre(paramètres: &str, nom: &str) -> Option<i64> {
    paramètres.split('&').find_map(|paire| {
        let (clé, valeur) = paire.split_once('=')?;
        if clé != nom {
            return None;
        }
        valeur.parse().ok()
    })
}

/// Les trois instants du relevé, lus dans les paramètres de la demande.
fn heures_demandées(paramètres: &str) -> Option<HeuresDuCoucherEtDuLever> {
    Some(HeuresDuCoucherEtDuLever {
        coucher_ms: paramètre(paramètres, "coucher_ms")?,
        lever_ms: paramètre(paramètres, "lever_ms")?,
        coucher_suivant_ms: paramètre(paramètres, "coucher_souhaite_ms")?,
    })
}

/// Une nuit de l'agenda, telle que la page web l'envoie ; le jour est dans le chemin.
#[derive(Deserialize)]
struct NuitReçueJson {
    endormissement: String,
    continuite: String,
    coucher_ms: i64,
    lever_ms: i64,
}

/// Une nuit de l'agenda, telle que la page web la reçoit.
#[derive(Serialize)]
struct NuitJson<'a> {
    jour: &'a str,
    endormissement: &'static str,
    continuite: &'static str,
    coucher_ms: i64,
    lever_ms: i64,
}

#[derive(Serialize, Deserialize)]
struct RéglagesJson {
    coucher_souhaite: String,
}

/// Un jour `AAAA-MM-JJ` : quatre chiffres, deux, deux, séparés par des tirets.
fn jour_lisible(jour: &str) -> bool {
    let parties: Vec<&str> = jour.split('-').collect();
    let longueurs: Vec<usize> = parties.iter().map(|partie| partie.len()).collect();
    let chiffres = parties
        .iter()
        .all(|partie| partie.chars().all(|caractère| caractère.is_ascii_digit()));
    longueurs == [4, 2, 2] && chiffres
}

/// Une heure `HH:MM`, de 00:00 à 23:59.
fn heure_lisible(heure: &str) -> bool {
    let Some((heures, minutes)) = heure.split_once(':') else {
        return false;
    };
    if heures.len() != 2 || minutes.len() != 2 {
        return false;
    }
    match (heures.parse::<u8>(), minutes.parse::<u8>()) {
        (Ok(heures), Ok(minutes)) => heures < 24 && minutes < 60,
        _ => false,
    }
}

fn en_json<T: Serialize>(valeur: &T) -> Réponse {
    match serde_json::to_string(valeur) {
        Ok(corps) => Réponse { statut: 200, corps },
        Err(_) => Réponse::vide(500),
    }
}

pub struct Serveur {
    jeton: String,
    journal: SqliteJournal,
    agenda: AgendaSqlite,
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
    pub fn new(jeton: String, journal: SqliteJournal, agenda: AgendaSqlite) -> Serveur {
        Serveur {
            jeton,
            journal,
            agenda,
        }
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

    fn relevé(&self, paramètres: &str) -> Réponse {
        let Some(heures) = heures_demandées(paramètres) else {
            return Réponse::vide(400);
        };
        let historique = match self.journal.historique() {
            Ok(historique) => historique,
            Err(_) => return Réponse::vide(500),
        };
        let relevé = RelevéDuJourJson::from(relevé_du_jour(&heures, &historique));
        match serde_json::to_string(&relevé) {
            Ok(corps) => Réponse { statut: 200, corps },
            Err(_) => Réponse::vide(500),
        }
    }

    fn nuit(&self, jour: &str) -> Réponse {
        match self.agenda.nuit(jour) {
            Ok(Some(nuit)) => en_json(&NuitJson {
                jour: &nuit.jour,
                endormissement: nuit.endormissement.nom(),
                continuite: nuit.continuité.nom(),
                coucher_ms: nuit.coucher_ms,
                lever_ms: nuit.lever_ms,
            }),
            Ok(None) => Réponse::vide(404),
            Err(_) => Réponse::vide(500),
        }
    }

    fn noter_la_nuit(&self, jour: &str, corps: &str) -> Réponse {
        let Ok(reçue) = serde_json::from_str::<NuitReçueJson>(corps) else {
            return Réponse::vide(400);
        };
        let endormissement = Endormissement::depuis_le_nom(&reçue.endormissement);
        let continuité = Continuité::depuis_le_nom(&reçue.continuite);
        let (Some(endormissement), Some(continuité)) = (endormissement, continuité) else {
            return Réponse::vide(400);
        };
        let nuit = Nuit {
            jour: jour.to_string(),
            endormissement,
            continuité,
            coucher_ms: reçue.coucher_ms,
            lever_ms: reçue.lever_ms,
        };
        match self.agenda.garder(&nuit) {
            Ok(()) => Réponse::vide(204),
            Err(_) => Réponse::vide(500),
        }
    }

    fn réglages(&self) -> Réponse {
        match self.agenda.coucher_souhaité() {
            Ok(coucher_souhaite) => en_json(&RéglagesJson { coucher_souhaite }),
            Err(_) => Réponse::vide(500),
        }
    }

    fn régler(&self, corps: &str) -> Réponse {
        let Ok(réglages) = serde_json::from_str::<RéglagesJson>(corps) else {
            return Réponse::vide(400);
        };
        if !heure_lisible(&réglages.coucher_souhaite) {
            return Réponse::vide(400);
        }
        match self
            .agenda
            .régler_le_coucher_souhaité(&réglages.coucher_souhaite)
        {
            Ok(()) => Réponse::vide(204),
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
        let (chemin, paramètres) = requête
            .chemin
            .split_once('?')
            .unwrap_or((requête.chemin, ""));
        if let Some(jour) = chemin.strip_prefix("/api/nuits/") {
            if !jour_lisible(jour) {
                return Réponse::vide(400);
            }
            return match requête.méthode {
                Méthode::Get => self.nuit(jour),
                Méthode::Post => self.noter_la_nuit(jour, requête.corps),
                Méthode::Autre => Réponse::vide(404),
            };
        }
        match (requête.méthode, chemin) {
            (Méthode::Get, "/api/mesures") => self.résumé(),
            (Méthode::Post, "/api/mesures") => self.réception(requête.corps),
            (Méthode::Get, "/api/releve") => self.relevé(paramètres),
            (Méthode::Get, "/api/reglages") => self.réglages(),
            (Méthode::Post, "/api/reglages") => self.régler(requête.corps),
            _ => Réponse::vide(404),
        }
    }
}
