#![forbid(unsafe_code)]

//! Reçoit les mesures du téléphone et les rend à la page web. Toute requête exige le jeton.

use std::collections::HashSet;

use agenda::{AgendaSqlite, Continuité, Endormissement, Fatigue, Nuit, QuantitéDeSommeil};
use journal::{Journal, ajouter};
use journal_sqlite::SqliteJournal;
use mesure::Mesure;
use releve::{
    Fenêtre, HeuresDuCoucherEtDuLever, LumièreDeLaJournée, LumièreDuMatin, LumièreÀÉviter,
    PériodeMesurée, RelevéDuJour, relevé_du_jour,
};
use serde::{Deserialize, Serialize};

/// Les jours de la tendance : une semaine.
const NOMBRE_DE_NUITS_DE_LA_TENDANCE: u32 = 7;

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

#[derive(Serialize)]
struct FenêtreJson {
    debut_ms: i64,
    fin_ms: i64,
}

#[derive(Serialize)]
struct PériodeMesuréeJson {
    debut_ms: i64,
    fin_ms: i64,
    valeur_lux: f32,
}

impl From<Fenêtre> for FenêtreJson {
    fn from(fenêtre: Fenêtre) -> FenêtreJson {
        FenêtreJson {
            debut_ms: fenêtre.début_ms,
            fin_ms: fenêtre.fin_ms,
        }
    }
}

impl From<PériodeMesurée> for PériodeMesuréeJson {
    fn from(période: PériodeMesurée) -> PériodeMesuréeJson {
        PériodeMesuréeJson {
            debut_ms: période.début_ms,
            fin_ms: période.fin_ms,
            valeur_lux: période.valeur_lux,
        }
    }
}

fn périodes_en_json(périodes: Vec<PériodeMesurée>) -> Vec<PériodeMesuréeJson> {
    périodes.into_iter().map(PériodeMesuréeJson::from).collect()
}

/// La lumière du soir ou de la nuit, telle que la page web la reçoit.
#[derive(Serialize)]
struct LumièreÀÉviterJson {
    temps_au_dessus_du_seuil_ms: i64,
    valeur_la_plus_haute_lux: Option<f32>,
    objectif_tenu: bool,
    couverture: f64,
    fenetre: FenêtreJson,
    periodes_mesurees: Vec<PériodeMesuréeJson>,
}

#[derive(Serialize)]
struct LumièreDuMatinJson {
    temps_a_10_000_lx_ou_plus_ms: i64,
    objectif_tenu: bool,
    couverture: f64,
    fenetre: FenêtreJson,
    periodes_mesurees: Vec<PériodeMesuréeJson>,
}

#[derive(Serialize)]
struct LumièreDeLaJournéeJson {
    part_du_temps_mesure_a_250_lx_ou_plus: Option<f64>,
    objectif_tenu: bool,
    couverture: f64,
    fenetre: FenêtreJson,
    periodes_mesurees: Vec<PériodeMesuréeJson>,
}

/// Une note de l'agenda, telle que la page web la reçoit dans le relevé.
#[derive(Serialize)]
struct NoteJson {
    niveau: &'static str,
    objectif_tenu: bool,
}

#[derive(Serialize)]
struct QuantitéDeSommeilJson {
    duree_ms: i64,
    objectif_tenu: bool,
}

/// Le relevé du jour, tel que la page web le reçoit : les noms des champs sont sans accents.
#[derive(Serialize)]
struct RelevéDuJourJson {
    endormissement: NoteJson,
    continuite: NoteJson,
    quantite_de_sommeil: QuantitéDeSommeilJson,
    lumiere_du_soir: LumièreÀÉviterJson,
    lumiere_de_la_nuit: LumièreÀÉviterJson,
    lumiere_du_matin: LumièreDuMatinJson,
    lumiere_de_la_journee: LumièreDeLaJournéeJson,
    /// Aucune tant qu'elle n'est pas notée, le soir.
    fatigue: Option<NoteJson>,
}

/// Un relevé parmi ceux des dernières nuits, avec le jour de son lever.
#[derive(Serialize)]
struct RelevéDatéJson<'a> {
    jour: &'a str,
    #[serde(flatten)]
    relevé: RelevéDuJourJson,
}

impl From<LumièreÀÉviter> for LumièreÀÉviterJson {
    fn from(lumière: LumièreÀÉviter) -> LumièreÀÉviterJson {
        LumièreÀÉviterJson {
            temps_au_dessus_du_seuil_ms: lumière.temps_au_dessus_du_seuil_ms,
            valeur_la_plus_haute_lux: lumière.valeur_la_plus_haute_lux,
            objectif_tenu: lumière.objectif_tenu,
            couverture: lumière.couverture,
            fenetre: FenêtreJson::from(lumière.fenêtre),
            periodes_mesurees: périodes_en_json(lumière.périodes_mesurées),
        }
    }
}

impl From<LumièreDuMatin> for LumièreDuMatinJson {
    fn from(lumière: LumièreDuMatin) -> LumièreDuMatinJson {
        LumièreDuMatinJson {
            temps_a_10_000_lx_ou_plus_ms: lumière.temps_à_10_000_lx_ou_plus_ms,
            objectif_tenu: lumière.objectif_tenu,
            couverture: lumière.couverture,
            fenetre: FenêtreJson::from(lumière.fenêtre),
            periodes_mesurees: périodes_en_json(lumière.périodes_mesurées),
        }
    }
}

impl From<LumièreDeLaJournée> for LumièreDeLaJournéeJson {
    fn from(lumière: LumièreDeLaJournée) -> LumièreDeLaJournéeJson {
        LumièreDeLaJournéeJson {
            part_du_temps_mesure_a_250_lx_ou_plus: lumière.part_du_temps_mesuré_à_250_lx_ou_plus,
            objectif_tenu: lumière.objectif_tenu,
            couverture: lumière.couverture,
            fenetre: FenêtreJson::from(lumière.fenêtre),
            periodes_mesurees: périodes_en_json(lumière.périodes_mesurées),
        }
    }
}

impl From<QuantitéDeSommeil> for QuantitéDeSommeilJson {
    fn from(quantité: QuantitéDeSommeil) -> QuantitéDeSommeilJson {
        QuantitéDeSommeilJson {
            duree_ms: quantité.durée_ms,
            objectif_tenu: quantité.objectif_tenu,
        }
    }
}

/// Le relevé de la lumière, avec les notes de la nuit et la fatigue de la journée qui suit.
impl RelevéDuJourJson {
    fn new(relevé: RelevéDuJour, nuit: &Nuit, fatigue: Option<Fatigue>) -> RelevéDuJourJson {
        let fatigue = fatigue.map(|fatigue| NoteJson {
            niveau: fatigue.nom(),
            objectif_tenu: fatigue.objectif_tenu(),
        });
        RelevéDuJourJson {
            endormissement: NoteJson {
                niveau: nuit.endormissement.nom(),
                objectif_tenu: nuit.endormissement.objectif_tenu(),
            },
            continuite: NoteJson {
                niveau: nuit.continuité.nom(),
                objectif_tenu: nuit.continuité.objectif_tenu(),
            },
            quantite_de_sommeil: QuantitéDeSommeilJson::from(nuit.quantité_de_sommeil()),
            lumiere_du_soir: LumièreÀÉviterJson::from(relevé.lumière_du_soir),
            lumiere_de_la_nuit: LumièreÀÉviterJson::from(relevé.lumière_de_la_nuit),
            lumiere_du_matin: LumièreDuMatinJson::from(relevé.lumière_du_matin),
            lumiere_de_la_journee: LumièreDeLaJournéeJson::from(relevé.lumière_de_la_journée),
            fatigue,
        }
    }
}

/// La valeur du paramètre `nom` dans `paramètres` (`a=1&b=2`), si elle y est.
fn paramètre<'a>(paramètres: &'a str, nom: &str) -> Option<&'a str> {
    paramètres.split('&').find_map(|paire| {
        let (clé, valeur) = paire.split_once('=')?;
        if clé != nom {
            return None;
        }
        Some(valeur)
    })
}

/// Le jour (`AAAA-MM-JJ`) et le coucher souhaité d'une demande de relevé, s'ils sont lisibles.
fn jour_et_coucher_souhaité(paramètres: &str) -> Option<(&str, i64)> {
    let jour = paramètre(paramètres, "jour")?;
    let coucher_souhaité_ms = paramètre(paramètres, "coucher_souhaite_ms")?
        .parse::<i64>()
        .ok()?;
    if !jour_lisible(jour) {
        return None;
    }
    Some((jour, coucher_souhaité_ms))
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

#[derive(Deserialize)]
struct FatigueReçueJson {
    fatigue: String,
}

/// La fatigue d'une journée, telle que la page web la reçoit.
#[derive(Serialize)]
struct JournéeJson<'a> {
    jour: &'a str,
    fatigue: &'static str,
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

    /// Le relevé de la nuit qui finit le `jour` demandé : ses heures viennent de l'agenda, la fin de la journée du
    /// coucher souhaité.
    fn relevé(&self, paramètres: &str) -> Réponse {
        let Some((jour, coucher_souhaité_ms)) = jour_et_coucher_souhaité(paramètres) else {
            return Réponse::vide(400);
        };
        let nuit = match self.agenda.nuit(jour) {
            Ok(Some(nuit)) => nuit,
            Ok(None) => return Réponse::vide(404),
            Err(_) => return Réponse::vide(500),
        };
        let Ok(fatigue) = self.agenda.fatigue(jour) else {
            return Réponse::vide(500);
        };
        let historique = match self.journal.historique() {
            Ok(historique) => historique,
            Err(_) => return Réponse::vide(500),
        };
        let heures = HeuresDuCoucherEtDuLever {
            coucher_ms: nuit.coucher_ms,
            lever_ms: nuit.lever_ms,
            coucher_suivant_ms: coucher_souhaité_ms,
        };
        let relevé = relevé_du_jour(&heures, &historique);
        en_json(&RelevéDuJourJson::new(relevé, &nuit, fatigue))
    }

    /// Les relevés des dernières nuits notées, jusqu'au `jour` : chacun finit au coucher noté le lendemain.
    fn relevés(&self, paramètres: &str) -> Réponse {
        let Some((jour, coucher_souhaité_ms)) = jour_et_coucher_souhaité(paramètres) else {
            return Réponse::vide(400);
        };
        let Ok(nuits) = self
            .agenda
            .dernières_nuits(jour, NOMBRE_DE_NUITS_DE_LA_TENDANCE)
        else {
            return Réponse::vide(500);
        };
        let Ok(historique) = self.journal.historique() else {
            return Réponse::vide(500);
        };
        let mut relevés = Vec::new();
        for (position, nuit) in nuits.iter().enumerate() {
            let Ok(fatigue) = self.agenda.fatigue(&nuit.jour) else {
                return Réponse::vide(500);
            };
            let nuit_suivante = nuits.get(position + 1);
            let heures = HeuresDuCoucherEtDuLever {
                coucher_ms: nuit.coucher_ms,
                lever_ms: nuit.lever_ms,
                coucher_suivant_ms: nuit.coucher_suivant_ms(nuit_suivante, coucher_souhaité_ms),
            };
            let relevé = relevé_du_jour(&heures, &historique);
            relevés.push(RelevéDatéJson {
                jour: &nuit.jour,
                relevé: RelevéDuJourJson::new(relevé, nuit, fatigue),
            });
        }
        en_json(&relevés)
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

    fn journée(&self, jour: &str) -> Réponse {
        match self.agenda.fatigue(jour) {
            Ok(Some(fatigue)) => en_json(&JournéeJson {
                jour,
                fatigue: fatigue.nom(),
            }),
            Ok(None) => Réponse::vide(404),
            Err(_) => Réponse::vide(500),
        }
    }

    fn noter_la_fatigue(&self, jour: &str, corps: &str) -> Réponse {
        let Ok(reçue) = serde_json::from_str::<FatigueReçueJson>(corps) else {
            return Réponse::vide(400);
        };
        let Some(fatigue) = Fatigue::depuis_le_nom(&reçue.fatigue) else {
            return Réponse::vide(400);
        };
        match self.agenda.garder_la_fatigue(jour, fatigue) {
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
        if let Some(jour) = chemin.strip_prefix("/api/journees/") {
            if !jour_lisible(jour) {
                return Réponse::vide(400);
            }
            return match requête.méthode {
                Méthode::Get => self.journée(jour),
                Méthode::Post => self.noter_la_fatigue(jour, requête.corps),
                Méthode::Autre => Réponse::vide(404),
            };
        }
        match (requête.méthode, chemin) {
            (Méthode::Get, "/api/mesures") => self.résumé(),
            (Méthode::Post, "/api/mesures") => self.réception(requête.corps),
            (Méthode::Get, "/api/releve") => self.relevé(paramètres),
            (Méthode::Get, "/api/releves") => self.relevés(paramètres),
            (Méthode::Get, "/api/reglages") => self.réglages(),
            (Méthode::Post, "/api/reglages") => self.régler(requête.corps),
            _ => Réponse::vide(404),
        }
    }
}
