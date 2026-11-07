use std::path::PathBuf;

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
    Serveur::new(JETON.to_string(), journal)
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
    let mut serveur = Serveur::new(JETON.to_string(), SqliteJournal::open(&path).unwrap());
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
        "/api/releve?coucher_ms={}&lever_ms={}&coucher_souhaite_ms={}",
        COUCHER_MS,
        COUCHER_MS + 8 * HEURE_MS,
        COUCHER_MS + 24 * HEURE_MS
    )
}

#[test]
fn le_serveur_calcule_le_relevé_du_jour_sur_les_mesures_reçues() {
    let mut serveur = serveur_vide("releve");
    let autorisation = format!("Bearer {JETON}");
    // Une lampe à 50 lx une heure avant le coucher : le soir n'est pas tenu, la nuit n'a rien de mesuré.
    let corps = format!(r#"[{{"valeur_lux": 50.0, "instant_ms": {}}}]"#, COUCHER_MS - HEURE_MS);
    serveur.répondre(envoi(Some(&autorisation), &corps));

    let chemin = chemin_du_relevé();
    let réponse = serveur.répondre(demande_du_relevé(Some(&autorisation), &chemin));

    assert_eq!(réponse.statut, 200);
    let relevé: Value = serde_json::from_str(&réponse.corps).unwrap();
    assert_eq!(relevé["lumiere_du_soir"]["valeur_la_plus_haute_lux"], json!(50.0));
    assert_eq!(relevé["lumiere_du_soir"]["objectif_tenu"], json!(false));
    assert_eq!(relevé["lumiere_de_la_nuit"]["valeur_la_plus_haute_lux"], Value::Null);
    assert_eq!(relevé["lumiere_du_matin"]["temps_a_10_000_lx_ou_plus_ms"], json!(0));
    assert_eq!(relevé["lumiere_de_la_journee"]["part_du_temps_mesure_a_250_lx_ou_plus"], Value::Null);
}

#[test]
fn le_relevé_est_refusé_sans_le_bon_jeton_ou_s_il_manque_un_instant() {
    let mut serveur = serveur_vide("releve_refus");
    let autorisation = format!("Bearer {JETON}");
    let chemin = chemin_du_relevé();
    let chemin_incomplet = format!("/api/releve?coucher_ms={COUCHER_MS}");

    let sans_jeton = serveur.répondre(demande_du_relevé(None, &chemin));
    let incomplet = serveur.répondre(demande_du_relevé(Some(&autorisation), &chemin_incomplet));

    assert_eq!(sans_jeton.statut, 401);
    assert_eq!(incomplet.statut, 400);
}
