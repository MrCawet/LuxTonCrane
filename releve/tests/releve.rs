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

#[test]
fn une_mesure_vaut_10_min_au_plus() {
    let historique = vec![
        mesure(50.0, COUCHER_MS - 60 * MINUTE_MS),
        mesure(5.0, COUCHER_MS - 30 * MINUTE_MS),
    ];

    let relevé = relevé_du_jour(&heures(), &historique);

    assert_eq!(
        relevé.lumière_du_soir.temps_au_dessus_du_seuil_ms,
        10 * MINUTE_MS
    );
}

#[test]
fn le_soir_donne_la_valeur_la_plus_haute() {
    let historique = vec![
        mesure(30.0, COUCHER_MS - 60 * MINUTE_MS),
        mesure(80.0, COUCHER_MS - 55 * MINUTE_MS),
        mesure(5.0, COUCHER_MS - 50 * MINUTE_MS),
    ];

    let relevé = relevé_du_jour(&heures(), &historique);

    assert_eq!(relevé.lumière_du_soir.valeur_la_plus_haute_lux, Some(80.0));
}

#[test]
fn l_objectif_du_soir_est_tenu_quand_la_lumière_reste_à_10_lx_ou_moins() {
    let historique = vec![mesure(10.0, COUCHER_MS - 60 * MINUTE_MS)];

    let relevé = relevé_du_jour(&heures(), &historique);

    assert!(relevé.lumière_du_soir.objectif_tenu);
}

#[test]
fn une_seule_minute_au_dessus_de_10_lx_suffit_à_ne_pas_tenir_l_objectif_du_soir() {
    let historique = vec![
        mesure(11.0, COUCHER_MS - 60 * MINUTE_MS),
        mesure(5.0, COUCHER_MS - 59 * MINUTE_MS),
    ];

    let relevé = relevé_du_jour(&heures(), &historique);

    assert!(!relevé.lumière_du_soir.objectif_tenu);
}

#[test]
fn la_couverture_du_soir_est_la_part_de_ses_3_h_couverte_par_une_mesure() {
    let mut historique = Vec::new();
    for minutes_avant_le_coucher in [90, 80, 70, 60, 50, 40, 30, 20, 10] {
        historique.push(mesure(
            5.0,
            COUCHER_MS - minutes_avant_le_coucher * MINUTE_MS,
        ));
    }

    let relevé = relevé_du_jour(&heures(), &historique);

    assert_eq!(relevé.lumière_du_soir.couverture, 0.5);
}

#[test]
fn la_nuit_compte_le_temps_au_dessus_de_1_lx_jusqu_au_lever() {
    let historique = vec![mesure(2.0, LEVER_MS - 5 * MINUTE_MS)];

    let relevé = relevé_du_jour(&heures(), &historique);

    assert_eq!(
        relevé.lumière_de_la_nuit.temps_au_dessus_du_seuil_ms,
        5 * MINUTE_MS
    );
}

#[test]
fn le_matin_compte_le_temps_à_10_000_lx_ou_plus() {
    let historique = vec![
        mesure(10_000.0, LEVER_MS + 30 * MINUTE_MS),
        mesure(9_999.0, LEVER_MS + 35 * MINUTE_MS),
    ];

    let relevé = relevé_du_jour(&heures(), &historique);

    assert_eq!(
        relevé.lumière_du_matin.temps_à_10_000_lx_ou_plus_ms,
        5 * MINUTE_MS
    );
}

#[test]
fn l_objectif_du_matin_est_tenu_avec_30_min_à_10_000_lx_ou_plus() {
    let historique = vec![
        mesure(10_000.0, LEVER_MS),
        mesure(10_000.0, LEVER_MS + 10 * MINUTE_MS),
        mesure(10_000.0, LEVER_MS + 20 * MINUTE_MS),
    ];

    let relevé = relevé_du_jour(&heures(), &historique);

    assert!(relevé.lumière_du_matin.objectif_tenu);
}

#[test]
fn l_objectif_du_matin_n_est_pas_tenu_avec_moins_de_30_min_à_10_000_lx_ou_plus() {
    let historique = vec![
        mesure(10_000.0, LEVER_MS),
        mesure(10_000.0, LEVER_MS + 10 * MINUTE_MS),
        mesure(10_000.0, LEVER_MS + 20 * MINUTE_MS),
        mesure(500.0, LEVER_MS + 29 * MINUTE_MS),
    ];

    let relevé = relevé_du_jour(&heures(), &historique);

    assert!(!relevé.lumière_du_matin.objectif_tenu);
}

#[test]
fn la_couverture_du_matin_est_la_part_de_ses_2_h_couverte_par_une_mesure() {
    let mut historique = Vec::new();
    for minutes_après_le_lever in [0, 10, 20] {
        historique.push(mesure(500.0, LEVER_MS + minutes_après_le_lever * MINUTE_MS));
    }

    let relevé = relevé_du_jour(&heures(), &historique);

    assert_eq!(relevé.lumière_du_matin.couverture, 0.25);
}

#[test]
fn la_journée_donne_la_part_du_temps_mesuré_à_250_lx_ou_plus() {
    let historique = vec![
        mesure(250.0, LEVER_MS + 3 * HEURE_MS),
        mesure(249.0, LEVER_MS + 3 * HEURE_MS + 10 * MINUTE_MS),
    ];

    let relevé = relevé_du_jour(&heures(), &historique);

    assert_eq!(
        relevé
            .lumière_de_la_journée
            .part_du_temps_mesuré_à_250_lx_ou_plus,
        Some(0.5)
    );
}

#[test]
fn l_objectif_de_la_journée_est_tenu_si_tout_son_temps_mesuré_est_à_250_lx_ou_plus() {
    let historique = vec![
        mesure(300.0, LEVER_MS + HEURE_MS),
        mesure(5.0, COUCHER_SUIVANT_MS - 3 * HEURE_MS),
    ];

    let relevé = relevé_du_jour(&heures(), &historique);

    assert!(relevé.lumière_de_la_journée.objectif_tenu);
}

#[test]
fn une_seule_minute_sous_250_lx_suffit_à_ne_pas_tenir_l_objectif_de_la_journée() {
    let historique = vec![
        mesure(300.0, LEVER_MS + HEURE_MS),
        mesure(249.0, LEVER_MS + HEURE_MS + 10 * MINUTE_MS),
        mesure(300.0, LEVER_MS + HEURE_MS + 11 * MINUTE_MS),
    ];

    let relevé = relevé_du_jour(&heures(), &historique);

    assert!(!relevé.lumière_de_la_journée.objectif_tenu);
}

#[test]
fn la_couverture_de_la_journée_est_la_part_couverte_du_lever_à_3_h_avant_le_coucher_suivant() {
    let historique = vec![mesure(300.0, LEVER_MS + HEURE_MS)];

    let relevé = relevé_du_jour(&heures(), &historique);

    let durée_de_la_journée_ms = COUCHER_SUIVANT_MS - 3 * HEURE_MS - LEVER_MS;
    let attendue = (10 * MINUTE_MS) as f64 / durée_de_la_journée_ms as f64;
    assert_eq!(relevé.lumière_de_la_journée.couverture, attendue);
}

#[test]
fn la_journée_sans_mesure_ne_donne_aucune_part_à_250_lx_ou_plus() {
    let historique = Vec::new();

    let relevé = relevé_du_jour(&heures(), &historique);

    assert_eq!(
        relevé
            .lumière_de_la_journée
            .part_du_temps_mesuré_à_250_lx_ou_plus,
        None
    );
}
