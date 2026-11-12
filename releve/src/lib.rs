#![forbid(unsafe_code)]

//! Le relevé du jour : la lumière reçue autour d'une nuit et de la journée qui suit.

use mesure::Mesure;

const MINUTE_MS: i64 = 60 * 1000;
const HEURE_MS: i64 = 60 * MINUTE_MS;

/// Couvre l'écart entre deux réveils écran éteint, d'environ 9 min.
const DURÉE_DE_VALIDITÉ_MAXIMALE_MS: i64 = 10 * MINUTE_MS;

const SEUIL_DU_SOIR_LX: f32 = 10.0;
const DURÉE_DU_SOIR_MS: i64 = 3 * HEURE_MS;

const SEUIL_DE_LA_NUIT_LX: f32 = 1.0;

const SEUIL_DU_MATIN_LX: f32 = 10_000.0;
const DURÉE_DU_MATIN_MS: i64 = 2 * HEURE_MS;
const OBJECTIF_DU_MATIN_MS: i64 = 30 * MINUTE_MS;

const SEUIL_DE_LA_JOURNÉE_LX: f32 = 250.0;

pub struct HeuresDuCoucherEtDuLever {
    pub coucher_ms: i64,
    pub lever_ms: i64,
    /// Le coucher qui suit la journée du relevé.
    pub coucher_suivant_ms: i64,
}

/// La lumière d'une fenêtre où l'objectif est de rester sous un seuil : le soir et la nuit.
pub struct LumièreÀÉviter {
    pub temps_au_dessus_du_seuil_ms: i64,
    /// Aucune si rien n'a été mesuré dans la fenêtre.
    pub valeur_la_plus_haute_lux: Option<f32>,
    /// Sans tolérance : une seule minute au-dessus du seuil suffit à ne pas le tenir.
    pub objectif_tenu: bool,
    /// La part de la fenêtre couverte par une mesure, de 0 à 1.
    pub couverture: f64,
    pub fenêtre: Fenêtre,
    /// Dans l'ordre chronologique ; le temps entre deux périodes n'est pas mesuré.
    pub périodes_mesurées: Vec<PériodeMesurée>,
}

/// La lumière vive reçue dans les 2 h après le lever.
pub struct LumièreDuMatin {
    pub temps_à_10_000_lx_ou_plus_ms: i64,
    /// Au moins 30 min à 10 000 lx ou plus.
    pub objectif_tenu: bool,
    /// La part de la fenêtre couverte par une mesure, de 0 à 1.
    pub couverture: f64,
    pub fenêtre: Fenêtre,
    /// Dans l'ordre chronologique ; le temps entre deux périodes n'est pas mesuré.
    pub périodes_mesurées: Vec<PériodeMesurée>,
}

/// La lumière reçue du lever jusqu'à 3 h avant le coucher suivant.
pub struct LumièreDeLaJournée {
    /// De 0 à 1, aucune si rien n'a été mesuré dans la fenêtre.
    pub part_du_temps_mesuré_à_250_lx_ou_plus: Option<f64>,
    /// Sans tolérance : une seule minute mesurée sous 250 lx suffit à ne pas le tenir.
    pub objectif_tenu: bool,
    /// La part de la fenêtre couverte par une mesure, de 0 à 1.
    pub couverture: f64,
    pub fenêtre: Fenêtre,
    /// Dans l'ordre chronologique ; le temps entre deux périodes n'est pas mesuré.
    pub périodes_mesurées: Vec<PériodeMesurée>,
}

pub struct RelevéDuJour {
    pub lumière_du_soir: LumièreÀÉviter,
    pub lumière_de_la_nuit: LumièreÀÉviter,
    pub lumière_du_matin: LumièreDuMatin,
    pub lumière_de_la_journée: LumièreDeLaJournée,
}

/// Une période de temps du relevé, du début inclus à la fin exclue.
#[derive(Clone, Copy)]
pub struct Fenêtre {
    pub début_ms: i64,
    pub fin_ms: i64,
}

/// Ce qu'une fenêtre dit d'une durée.
impl Fenêtre {
    /// La part de la fenêtre que représente `durée_ms`, de 0 à 1.
    fn part(&self, durée_ms: i64) -> f64 {
        let durée_de_la_fenêtre_ms = self.fin_ms - self.début_ms;
        durée_ms as f64 / durée_de_la_fenêtre_ms as f64
    }
}

/// Le temps pendant lequel une mesure valable vaut, à l'intérieur d'une fenêtre, du début inclus à la fin exclue.
pub struct PériodeMesurée {
    pub début_ms: i64,
    pub fin_ms: i64,
    pub valeur_lux: f32,
}

/// Ce que dure une période mesurée.
impl PériodeMesurée {
    fn durée_ms(&self) -> i64 {
        self.fin_ms - self.début_ms
    }
}

/// Une mesure vaut jusqu'à la suivante, pendant 10 min au plus.
/// Une mesure prise téléphone couvert ne vaut rien, mais termine quand même la précédente.
fn périodes_mesurées(fenêtre: &Fenêtre, historique: &[Mesure]) -> Vec<PériodeMesurée> {
    let mut périodes = Vec::new();
    for (position, mesure) in historique.iter().enumerate() {
        if mesure.téléphone_couvert == Some(true) {
            continue;
        }
        let fin_maximale_ms = mesure.instant_ms + DURÉE_DE_VALIDITÉ_MAXIMALE_MS;
        let fin_de_validité_ms = match historique.get(position + 1) {
            Some(suivante) => suivante.instant_ms.min(fin_maximale_ms),
            None => fin_maximale_ms,
        };
        let début_ms = mesure.instant_ms.max(fenêtre.début_ms);
        let fin_ms = fin_de_validité_ms.min(fenêtre.fin_ms);
        if fin_ms > début_ms {
            périodes.push(PériodeMesurée {
                début_ms,
                fin_ms,
                valeur_lux: mesure.valeur_lux,
            });
        }
    }
    périodes
}

fn lumière_à_éviter(
    fenêtre: &Fenêtre,
    seuil_lx: f32,
    historique: &[Mesure],
) -> LumièreÀÉviter {
    let périodes_mesurées = périodes_mesurées(fenêtre, historique);
    let mut temps_au_dessus_du_seuil_ms = 0;
    let mut temps_mesuré_ms = 0;
    let mut valeur_la_plus_haute_lux: Option<f32> = None;
    for période in &périodes_mesurées {
        if période.valeur_lux > seuil_lx {
            temps_au_dessus_du_seuil_ms += période.durée_ms();
        }
        temps_mesuré_ms += période.durée_ms();
        valeur_la_plus_haute_lux = match valeur_la_plus_haute_lux {
            Some(valeur_lux) => Some(valeur_lux.max(période.valeur_lux)),
            None => Some(période.valeur_lux),
        };
    }
    let objectif_tenu = temps_au_dessus_du_seuil_ms == 0;
    LumièreÀÉviter {
        temps_au_dessus_du_seuil_ms,
        valeur_la_plus_haute_lux,
        objectif_tenu,
        couverture: fenêtre.part(temps_mesuré_ms),
        fenêtre: *fenêtre,
        périodes_mesurées,
    }
}

fn lumière_du_matin(fenêtre: &Fenêtre, historique: &[Mesure]) -> LumièreDuMatin {
    let périodes_mesurées = périodes_mesurées(fenêtre, historique);
    let mut temps_à_10_000_lx_ou_plus_ms = 0;
    let mut temps_mesuré_ms = 0;
    for période in &périodes_mesurées {
        if période.valeur_lux >= SEUIL_DU_MATIN_LX {
            temps_à_10_000_lx_ou_plus_ms += période.durée_ms();
        }
        temps_mesuré_ms += période.durée_ms();
    }
    let objectif_tenu = temps_à_10_000_lx_ou_plus_ms >= OBJECTIF_DU_MATIN_MS;
    LumièreDuMatin {
        temps_à_10_000_lx_ou_plus_ms,
        objectif_tenu,
        couverture: fenêtre.part(temps_mesuré_ms),
        fenêtre: *fenêtre,
        périodes_mesurées,
    }
}

fn lumière_de_la_journée(fenêtre: &Fenêtre, historique: &[Mesure]) -> LumièreDeLaJournée {
    let périodes_mesurées = périodes_mesurées(fenêtre, historique);
    let mut temps_à_250_lx_ou_plus_ms = 0;
    let mut temps_mesuré_ms = 0;
    for période in &périodes_mesurées {
        if période.valeur_lux >= SEUIL_DE_LA_JOURNÉE_LX {
            temps_à_250_lx_ou_plus_ms += période.durée_ms();
        }
        temps_mesuré_ms += période.durée_ms();
    }
    let part_du_temps_mesuré_à_250_lx_ou_plus = if temps_mesuré_ms == 0 {
        None
    } else {
        Some(temps_à_250_lx_ou_plus_ms as f64 / temps_mesuré_ms as f64)
    };
    let objectif_tenu = temps_à_250_lx_ou_plus_ms == temps_mesuré_ms;
    LumièreDeLaJournée {
        part_du_temps_mesuré_à_250_lx_ou_plus,
        objectif_tenu,
        couverture: fenêtre.part(temps_mesuré_ms),
        fenêtre: *fenêtre,
        périodes_mesurées,
    }
}

/// `historique` : les mesures dans l'ordre chronologique.
pub fn relevé_du_jour(heures: &HeuresDuCoucherEtDuLever, historique: &[Mesure]) -> RelevéDuJour {
    let soir = Fenêtre {
        début_ms: heures.coucher_ms - DURÉE_DU_SOIR_MS,
        fin_ms: heures.coucher_ms,
    };
    let nuit = Fenêtre {
        début_ms: heures.coucher_ms,
        fin_ms: heures.lever_ms,
    };
    let matin = Fenêtre {
        début_ms: heures.lever_ms,
        fin_ms: heures.lever_ms + DURÉE_DU_MATIN_MS,
    };
    let journée = Fenêtre {
        début_ms: heures.lever_ms,
        fin_ms: heures.coucher_suivant_ms - DURÉE_DU_SOIR_MS,
    };
    RelevéDuJour {
        lumière_du_soir: lumière_à_éviter(&soir, SEUIL_DU_SOIR_LX, historique),
        lumière_de_la_nuit: lumière_à_éviter(&nuit, SEUIL_DE_LA_NUIT_LX, historique),
        lumière_du_matin: lumière_du_matin(&matin, historique),
        lumière_de_la_journée: lumière_de_la_journée(&journée, historique),
    }
}
