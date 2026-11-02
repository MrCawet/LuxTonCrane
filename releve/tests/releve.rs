use mesure::Mesure;
use releve::{HeuresDuCoucherEtDuLever, relevé_du_jour};

const MINUTE_MS: i64 = 60_000;
const HEURE_MS: i64 = 60 * MINUTE_MS;

const COUCHER_MS: i64 = 1_793_000_000_000;
const LEVER_MS: i64 = COUCHER_MS + 8 * HEURE_MS;
const COUCHER_SUIVANT_MS: i64 = LEVER_MS + 16 * HEURE_MS;

fn heures() -> HeuresDuCoucherEtDuLever {
    HeuresDuCoucherEtDuLever {
        coucher_ms: COUCHER_MS,
        lever_ms: LEVER_MS,
        coucher_suivant_ms: COUCHER_SUIVANT_MS,
    }
}

fn mesure(valeur_lux: f32, instant_ms: i64) -> Mesure {
    Mesure {
        valeur_lux,
        instant_ms,
    }
}

#[test]
fn le_soir_une_mesure_au_dessus_de_10_lx_compte_jusqu_à_la_suivante() {
    let historique = vec![
        mesure(50.0, COUCHER_MS - 60 * MINUTE_MS),
        mesure(5.0, COUCHER_MS - 55 * MINUTE_MS),
    ];

    let relevé = relevé_du_jour(&heures(), &historique);

    assert_eq!(
        relevé.lumière_du_soir.temps_au_dessus_du_seuil_ms,
        5 * MINUTE_MS
    );
}
