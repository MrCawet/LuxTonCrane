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
