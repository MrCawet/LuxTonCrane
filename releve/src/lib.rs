#![forbid(unsafe_code)]

//! Le relevé du jour : la lumière reçue autour d'une nuit et de la journée qui suit.

use mesure::Mesure;

const HEURE_MS: i64 = 60 * 60 * 1000;

const SEUIL_DU_SOIR_LX: f32 = 10.0;
const DURÉE_DU_SOIR_MS: i64 = 3 * HEURE_MS;

pub struct HeuresDuCoucherEtDuLever {
    pub coucher_ms: i64,
    pub lever_ms: i64,
    /// Le coucher qui suit la journée du relevé.
    pub coucher_suivant_ms: i64,
}

/// La lumière d'une fenêtre où l'objectif est de rester sous un seuil : le soir et la nuit.
pub struct LumièreÀÉviter {
    pub temps_au_dessus_du_seuil_ms: i64,
}

pub struct RelevéDuJour {
    pub lumière_du_soir: LumièreÀÉviter,
}

/// Une période de temps, du début inclus à la fin exclue.
struct Fenêtre {
    début_ms: i64,
    fin_ms: i64,
}

/// Le temps pendant lequel une mesure vaut, à l'intérieur d'une fenêtre.
struct PériodeMesurée {
    valeur_lux: f32,
    durée_ms: i64,
}

/// Une mesure vaut jusqu'à la suivante.
fn périodes_mesurées(fenêtre: &Fenêtre, historique: &[Mesure]) -> Vec<PériodeMesurée> {
    let mut périodes = Vec::new();
    for (position, mesure) in historique.iter().enumerate() {
        let fin_de_validité_ms = match historique.get(position + 1) {
            Some(suivante) => suivante.instant_ms,
            None => fenêtre.fin_ms,
        };
        let début_ms = mesure.instant_ms.max(fenêtre.début_ms);
        let fin_ms = fin_de_validité_ms.min(fenêtre.fin_ms);
        if fin_ms > début_ms {
            périodes.push(PériodeMesurée {
                valeur_lux: mesure.valeur_lux,
                durée_ms: fin_ms - début_ms,
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
    let mut temps_au_dessus_du_seuil_ms = 0;
    for période in périodes_mesurées(fenêtre, historique) {
        if période.valeur_lux > seuil_lx {
            temps_au_dessus_du_seuil_ms += période.durée_ms;
        }
    }
    LumièreÀÉviter {
        temps_au_dessus_du_seuil_ms,
    }
}

/// `historique` : les mesures dans l'ordre chronologique.
pub fn relevé_du_jour(heures: &HeuresDuCoucherEtDuLever, historique: &[Mesure]) -> RelevéDuJour {
    let soir = Fenêtre {
        début_ms: heures.coucher_ms - DURÉE_DU_SOIR_MS,
        fin_ms: heures.coucher_ms,
    };
    RelevéDuJour {
        lumière_du_soir: lumière_à_éviter(&soir, SEUIL_DU_SOIR_LX, historique),
    }
}
