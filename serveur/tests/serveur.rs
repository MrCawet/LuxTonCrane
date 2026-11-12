use std::path::PathBuf;

use agenda::AgendaSqlite;
use journal_sqlite::SqliteJournal;
use serde_json::{Value, json};
use serveur::{Méthode, Requête, Serveur};

const JETON: &str = "jeton-de-test";

fn fresh_database_path(test_name: &str) -> PathBuf {
    let file_name = format!("serveur_{}_{}.db", test_name, std::process::id());
    let path = std::env::temp_dir().join(file_name);
    let _ = std::fs::remove_file(&path);
    path
}

fn serveur_vide(test_name: &str) -> Serveur {
    let path = fresh_database_path(test_name);
    let journal = SqliteJournal::open(&path).unwrap();
    Serveur::new(
        JETON.to_string(),
        journal,
        AgendaSqlite::open(&path).unwrap(),
    )
}

fn envoi<'a>(autorisation: Option<&'a str>, corps: &'a str) -> Requête<'a> {
    Requête {
        méthode: Méthode::Post,
        chemin: "/api/mesures",
        autorisation,
        corps,
    }
}

fn lecture(autorisation: Option<&str>) -> Requête<'_> {
    Requête {
        méthode: Méthode::Get,
        chemin: "/api/mesures",
        autorisation,
        corps: "",
    }
}

fn résumé_de(serveur: &mut Serveur) -> Value {
    let autorisation = format!("Bearer {JETON}");
    let réponse = serveur.répondre(lecture(Some(&autorisation)));
    assert_eq!(réponse.statut, 200);
    serde_json::from_str(&réponse.corps).unwrap()
}

#[test]
fn une_requête_sans_le_bon_jeton_est_refusée() {
    let mut serveur = serveur_vide("refus");
    let corps = r#"[{"valeur_lux": 320.5, "instant_ms": 1793000000000}]"#;

    let envoi_sans_jeton = serveur.répondre(envoi(None, corps));
    let envoi_avec_un_faux_jeton = serveur.répondre(envoi(Some("Bearer faux"), corps));
    let lecture_sans_jeton = serveur.répondre(lecture(None));

    assert_eq!(envoi_sans_jeton.statut, 401);
    assert_eq!(envoi_avec_un_faux_jeton.statut, 401);
    assert_eq!(lecture_sans_jeton.statut, 401);
    assert_eq!(
        résumé_de(&mut serveur),
        json!({"nombre": 0, "derniere": null})
    );
}

#[test]
fn les_mesures_reçues_sont_gardées_et_résumées() {
    let mut serveur = serveur_vide("reception");
    let autorisation = format!("Bearer {JETON}");
    let corps = r#"[
        {"valeur_lux": 80.0, "instant_ms": 1793000060000},
        {"valeur_lux": 320.5, "instant_ms": 1793000000000}
    ]"#;

    let réponse = serveur.répondre(envoi(Some(&autorisation), corps));

    assert_eq!(réponse.statut, 204);
    assert_eq!(
        résumé_de(&mut serveur),
        json!({"nombre": 2, "derniere": {"valeur_lux": 80.0, "instant_ms": 1793000060000_i64}})
    );
}

#[test]
fn une_mesure_déjà_reçue_n_est_pas_gardée_deux_fois() {
    let mut serveur = serveur_vide("doublon");
    let autorisation = format!("Bearer {JETON}");
    let premier_envoi = r#"[{"valeur_lux": 320.5, "instant_ms": 1793000000000}]"#;
    let second_envoi = r#"[
        {"valeur_lux": 320.5, "instant_ms": 1793000000000},
        {"valeur_lux": 80.0, "instant_ms": 1793000060000}
    ]"#;

    serveur.répondre(envoi(Some(&autorisation), premier_envoi));
    serveur.répondre(envoi(Some(&autorisation), second_envoi));

    assert_eq!(
        résumé_de(&mut serveur),
        json!({"nombre": 2, "derniere": {"valeur_lux": 80.0, "instant_ms": 1793000060000_i64}})
    );
}

#[test]
fn le_serveur_garde_si_le_téléphone_était_couvert() {
    let path = fresh_database_path("couvert");
    let mut serveur = Serveur::new(
        JETON.to_string(),
        SqliteJournal::open(&path).unwrap(),
        AgendaSqlite::open(&path).unwrap(),
    );
    let autorisation = format!("Bearer {JETON}");
    let corps = r#"[
        {"valeur_lux": 0.0, "instant_ms": 1793000000000, "telephone_couvert": true},
        {"valeur_lux": 320.5, "instant_ms": 1793000060000}
    ]"#;

    serveur.répondre(envoi(Some(&autorisation), corps));

    let journal = SqliteJournal::open(&path).unwrap();
    let proximités: Vec<Option<bool>> = journal
        .historique()
        .unwrap()
        .iter()
        .map(|mesure| mesure.téléphone_couvert)
        .collect();
    assert_eq!(proximités, vec![Some(true), None]);
}

const HEURE_MS: i64 = 60 * 60 * 1000;
const COUCHER_MS: i64 = 1793000000000;

fn demande_du_relevé<'a>(autorisation: Option<&'a str>, chemin: &'a str) -> Requête<'a> {
    Requête {
        méthode: Méthode::Get,
        chemin,
        autorisation,
        corps: "",
    }
}

fn chemin_du_relevé() -> String {
    format!(
        "/api/releve?jour=2026-11-08&coucher_souhaite_ms={}",
        COUCHER_MS + 24 * HEURE_MS
    )
}

/// La nuit du 2026-11-08 : calme, réveil bref, 8 h au lit à partir de `COUCHER_MS`.
fn noter_la_nuit(serveur: &mut Serveur) {
    serveur.répondre(requête(Méthode::Post, "/api/nuits/2026-11-08", NUIT));
}

#[test]
fn le_serveur_calcule_le_relevé_du_jour_sur_les_mesures_reçues() {
    let mut serveur = serveur_vide("releve");
    let autorisation = format!("Bearer {JETON}");
    // Une lampe à 50 lx une heure avant le coucher : le soir n'est pas tenu, la nuit n'a rien de mesuré.
    let corps = format!(
        r#"[{{"valeur_lux": 50.0, "instant_ms": {}}}]"#,
        COUCHER_MS - HEURE_MS
    );
    serveur.répondre(envoi(Some(&autorisation), &corps));
    noter_la_nuit(&mut serveur);

    let chemin = chemin_du_relevé();
    let réponse = serveur.répondre(demande_du_relevé(Some(&autorisation), &chemin));

    assert_eq!(réponse.statut, 200);
    let relevé: Value = serde_json::from_str(&réponse.corps).unwrap();
    assert_eq!(
        relevé["lumiere_du_soir"]["valeur_la_plus_haute_lux"],
        json!(50.0)
    );
    assert_eq!(relevé["lumiere_du_soir"]["objectif_tenu"], json!(false));
    assert_eq!(
        relevé["lumiere_de_la_nuit"]["valeur_la_plus_haute_lux"],
        Value::Null
    );
    assert_eq!(
        relevé["lumiere_du_matin"]["temps_a_10_000_lx_ou_plus_ms"],
        json!(0)
    );
    assert_eq!(
        relevé["lumiere_de_la_journee"]["part_du_temps_mesure_a_250_lx_ou_plus"],
        Value::Null
    );
}

#[test]
fn le_relevé_donne_pour_chaque_fenêtre_ses_bornes_et_ses_périodes_mesurées() {
    let mut serveur = serveur_vide("releve_courbe");
    let autorisation = format!("Bearer {JETON}");
    let corps = format!(
        r#"[{{"valeur_lux": 50.0, "instant_ms": {}}}]"#,
        COUCHER_MS - HEURE_MS
    );
    serveur.répondre(envoi(Some(&autorisation), &corps));
    noter_la_nuit(&mut serveur);

    let chemin = chemin_du_relevé();
    let réponse = serveur.répondre(demande_du_relevé(Some(&autorisation), &chemin));

    let relevé: Value = serde_json::from_str(&réponse.corps).unwrap();
    let dix_minutes_ms = 10 * 60 * 1000;
    assert_eq!(
        relevé["lumiere_du_soir"]["fenetre"],
        json!({ "debut_ms": COUCHER_MS - 3 * HEURE_MS, "fin_ms": COUCHER_MS })
    );
    assert_eq!(
        relevé["lumiere_du_soir"]["periodes_mesurees"],
        json!([{
            "debut_ms": COUCHER_MS - HEURE_MS,
            "fin_ms": COUCHER_MS - HEURE_MS + dix_minutes_ms,
            "valeur_lux": 50.0
        }])
    );
    for fenêtre in [
        "lumiere_de_la_nuit",
        "lumiere_du_matin",
        "lumiere_de_la_journee",
    ] {
        assert_eq!(relevé[fenêtre]["periodes_mesurees"], json!([]));
        assert!(relevé[fenêtre]["fenetre"]["debut_ms"].is_i64());
    }
}

#[test]
fn le_relevé_est_refusé_sans_le_bon_jeton_sans_coucher_souhaité_ou_sans_la_nuit() {
    let mut serveur = serveur_vide("releve_refus");
    let autorisation = format!("Bearer {JETON}");
    let chemin = chemin_du_relevé();
    let chemin_incomplet = "/api/releve?jour=2026-11-08";

    let sans_la_nuit = serveur.répondre(demande_du_relevé(Some(&autorisation), &chemin));
    noter_la_nuit(&mut serveur);
    let sans_jeton = serveur.répondre(demande_du_relevé(None, &chemin));
    let incomplet = serveur.répondre(demande_du_relevé(Some(&autorisation), chemin_incomplet));

    assert_eq!(sans_la_nuit.statut, 404);
    assert_eq!(sans_jeton.statut, 401);
    assert_eq!(incomplet.statut, 400);
}

#[test]
fn le_relevé_montre_les_notes_de_l_agenda_et_la_quantité_de_sommeil() {
    let mut serveur = serveur_vide("releve_sommeil");
    let autorisation = format!("Bearer {JETON}");
    let nuit_agitée = NUIT.replace("calme", "agite");
    serveur.répondre(requête(
        Méthode::Post,
        "/api/nuits/2026-11-08",
        &nuit_agitée,
    ));
    let chemin = chemin_du_relevé();

    let sans_fatigue = serveur.répondre(demande_du_relevé(Some(&autorisation), &chemin));
    serveur.répondre(requête(
        Méthode::Post,
        "/api/journees/2026-11-08",
        r#"{"fatigue": "legere"}"#,
    ));
    let avec_fatigue = serveur.répondre(demande_du_relevé(Some(&autorisation), &chemin));

    let relevé: Value = serde_json::from_str(&sans_fatigue.corps).unwrap();
    assert_eq!(
        relevé["endormissement"],
        json!({"niveau": "agite", "objectif_tenu": false})
    );
    assert_eq!(
        relevé["continuite"],
        json!({"niveau": "reveil_bref", "objectif_tenu": true})
    );
    assert_eq!(
        relevé["quantite_de_sommeil"],
        json!({"duree_ms": 8 * HEURE_MS, "objectif_tenu": true})
    );
    assert_eq!(relevé["fatigue"], Value::Null);
    let relevé: Value = serde_json::from_str(&avec_fatigue.corps).unwrap();
    assert_eq!(
        relevé["fatigue"],
        json!({"niveau": "legere", "objectif_tenu": true})
    );
}

fn requête<'a>(méthode: Méthode, chemin: &'a str, corps: &'a str) -> Requête<'a> {
    Requête {
        méthode,
        chemin,
        autorisation: Some("Bearer jeton-de-test"),
        corps,
    }
}

const NUIT: &str = r#"{"endormissement": "calme", "continuite": "reveil_bref", "coucher_ms": 1793000000000, "lever_ms": 1793028800000}"#;

#[test]
fn une_nuit_notée_se_relit_à_la_date_de_son_lever() {
    let mut serveur = serveur_vide("nuit");

    let envoi = serveur.répondre(requête(Méthode::Post, "/api/nuits/2026-11-08", NUIT));
    let lecture = serveur.répondre(requête(Méthode::Get, "/api/nuits/2026-11-08", ""));
    let absente = serveur.répondre(requête(Méthode::Get, "/api/nuits/2026-11-07", ""));

    assert_eq!(envoi.statut, 204);
    assert_eq!(lecture.statut, 200);
    let nuit: Value = serde_json::from_str(&lecture.corps).unwrap();
    assert_eq!(
        nuit,
        json!({
            "jour": "2026-11-08",
            "endormissement": "calme",
            "continuite": "reveil_bref",
            "coucher_ms": 1793000000000_i64,
            "lever_ms": 1793028800000_i64
        })
    );
    assert_eq!(absente.statut, 404);
}

#[test]
fn une_nuit_avec_un_niveau_inconnu_ou_un_jour_illisible_est_refusée() {
    let mut serveur = serveur_vide("nuit_refus");
    let niveau_inconnu = NUIT.replace("calme", "serein");

    let mauvais_niveau = serveur.répondre(requête(
        Méthode::Post,
        "/api/nuits/2026-11-08",
        &niveau_inconnu,
    ));
    let mauvais_jour = serveur.répondre(requête(Méthode::Post, "/api/nuits/demain", NUIT));

    assert_eq!(mauvais_niveau.statut, 400);
    assert_eq!(mauvais_jour.statut, 400);
}

#[test]
fn le_coucher_souhaité_se_règle_et_vaut_23_h_par_défaut() {
    let mut serveur = serveur_vide("reglages");

    let par_défaut = serveur.répondre(requête(Méthode::Get, "/api/reglages", ""));
    let réglage = serveur.répondre(requête(
        Méthode::Post,
        "/api/reglages",
        r#"{"coucher_souhaite": "22:30"}"#,
    ));
    let illisible = serveur.répondre(requête(
        Méthode::Post,
        "/api/reglages",
        r#"{"coucher_souhaite": "25:99"}"#,
    ));
    let relu = serveur.répondre(requête(Méthode::Get, "/api/reglages", ""));

    assert_eq!(
        serde_json::from_str::<Value>(&par_défaut.corps).unwrap(),
        json!({"coucher_souhaite": "23:00"})
    );
    assert_eq!(réglage.statut, 204);
    assert_eq!(illisible.statut, 400);
    assert_eq!(
        serde_json::from_str::<Value>(&relu.corps).unwrap(),
        json!({"coucher_souhaite": "22:30"})
    );
}

#[test]
fn la_fatigue_du_jour_se_note_et_se_relit() {
    let mut serveur = serveur_vide("fatigue");

    let absente = serveur.répondre(requête(Méthode::Get, "/api/journees/2026-11-08", ""));
    let envoi = serveur.répondre(requête(
        Méthode::Post,
        "/api/journees/2026-11-08",
        r#"{"fatigue": "genante"}"#,
    ));
    let inconnue = serveur.répondre(requête(
        Méthode::Post,
        "/api/journees/2026-11-08",
        r#"{"fatigue": "epuise"}"#,
    ));
    let lecture = serveur.répondre(requête(Méthode::Get, "/api/journees/2026-11-08", ""));

    assert_eq!(absente.statut, 404);
    assert_eq!(envoi.statut, 204);
    assert_eq!(inconnue.statut, 400);
    assert_eq!(
        serde_json::from_str::<Value>(&lecture.corps).unwrap(),
        json!({"jour": "2026-11-08", "fatigue": "genante"})
    );
}

fn nuit_levée_à(lever_ms: i64) -> String {
    format!(
        r#"{{"endormissement": "agite", "continuite": "sans_reveil", "coucher_ms": {}, "lever_ms": {}}}"#,
        lever_ms - 8 * HEURE_MS,
        lever_ms
    )
}

#[test]
fn les_relevés_des_dernières_nuits_notées_prennent_le_coucher_noté_le_lendemain() {
    let mut serveur = serveur_vide("releves");
    let autorisation = format!("Bearer {JETON}");
    let lever_du_7_ms = COUCHER_MS + 8 * HEURE_MS - 24 * HEURE_MS;
    let lever_du_8_ms = COUCHER_MS + 8 * HEURE_MS;
    let nuit_du_7 = nuit_levée_à(lever_du_7_ms);
    let nuit_du_8 = nuit_levée_à(lever_du_8_ms);
    serveur.répondre(requête(Méthode::Post, "/api/nuits/2026-11-07", &nuit_du_7));
    serveur.répondre(requête(Méthode::Post, "/api/nuits/2026-11-08", &nuit_du_8));
    let coucher_souhaité_ms = lever_du_8_ms + 15 * HEURE_MS;
    let chemin = format!("/api/releves?jour=2026-11-08&coucher_souhaite_ms={coucher_souhaité_ms}");

    let réponse = serveur.répondre(demande_du_relevé(Some(&autorisation), &chemin));

    assert_eq!(réponse.statut, 200);
    let relevés: Value = serde_json::from_str(&réponse.corps).unwrap();
    assert_eq!(relevés[0]["jour"], json!("2026-11-07"));
    assert_eq!(relevés[1]["jour"], json!("2026-11-08"));
    let trois_heures_ms = 3 * HEURE_MS;
    assert_eq!(
        relevés[0]["lumiere_de_la_journee"]["fenetre"]["fin_ms"],
        json!(lever_du_8_ms - 8 * HEURE_MS - trois_heures_ms)
    );
    assert_eq!(
        relevés[1]["lumiere_de_la_journee"]["fenetre"]["fin_ms"],
        json!(coucher_souhaité_ms - trois_heures_ms)
    );
    assert_eq!(relevés[1]["endormissement"]["niveau"], json!("agite"));
}

#[test]
fn les_relevés_sont_refusés_sans_le_bon_jeton_ou_sans_coucher_souhaité() {
    let mut serveur = serveur_vide("releves_refus");
    let autorisation = format!("Bearer {JETON}");

    let sans_jeton = serveur.répondre(demande_du_relevé(
        None,
        "/api/releves?jour=2026-11-08&coucher_souhaite_ms=1",
    ));
    let incomplet = serveur.répondre(demande_du_relevé(
        Some(&autorisation),
        "/api/releves?jour=2026-11-08",
    ));

    assert_eq!(sans_jeton.statut, 401);
    assert_eq!(incomplet.statut, 400);
}
