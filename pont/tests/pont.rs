use std::path::PathBuf;

use pont::Journal;
use rusqlite::Connection;

fn fresh_database_path(test_name: &str) -> PathBuf {
    let file_name = format!("pont_{}_{}.db", test_name, std::process::id());
    let path = std::env::temp_dir().join(file_name);
    let _ = std::fs::remove_file(&path);
    path
}

#[test]
fn ajouter_une_mesure_depuis_kotlin_l_écrit_dans_la_base() {
    let path = fresh_database_path("ajouter");
    let journal = Journal::ouvrir(path.to_string_lossy().into_owned()).unwrap();

    journal.ajouter(320.5, 1_793_000_000_000, None).unwrap();

    let database = Connection::open(&path).unwrap();
    let écrite: (f32, i64) = database
        .query_row("SELECT valeur_lux, instant_ms FROM mesure", [], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })
        .unwrap();
    assert_eq!(écrite, (320.5, 1_793_000_000_000));
}

#[test]
fn les_mesures_à_envoyer_sont_celles_d_après_la_dernière_reçue_par_le_serveur() {
    let path = fresh_database_path("a_envoyer");
    let journal = Journal::ouvrir(path.to_string_lossy().into_owned()).unwrap();
    journal.ajouter(320.5, 1_793_000_000_000, None).unwrap();
    journal.ajouter(80.0, 1_793_000_060_000, None).unwrap();
    journal.ajouter(5.0, 1_793_000_120_000, None).unwrap();

    let après_la_première = journal.mesures_apres(Some(1_793_000_000_000)).unwrap();
    let sans_mesure_reçue = journal.mesures_apres(None).unwrap();

    let instants_après_la_première: Vec<i64> = après_la_première
        .iter()
        .map(|mesure| mesure.instant_ms)
        .collect();
    assert_eq!(
        instants_après_la_première,
        vec![1_793_000_060_000, 1_793_000_120_000]
    );
    assert_eq!(sans_mesure_reçue.len(), 3);
}

#[test]
fn ajouter_une_mesure_depuis_kotlin_écrit_si_le_téléphone_était_couvert() {
    let path = fresh_database_path("couvert");
    let journal = Journal::ouvrir(path.to_string_lossy().into_owned()).unwrap();

    journal.ajouter(0.0, 1_793_000_000_000, Some(true)).unwrap();

    let database = Connection::open(&path).unwrap();
    let écrit: Option<bool> = database
        .query_row("SELECT telephone_couvert FROM mesure", [], |row| row.get(0))
        .unwrap();
    assert_eq!(écrit, Some(true));
}

#[test]
fn les_mesures_à_envoyer_disent_si_le_téléphone_était_couvert() {
    let path = fresh_database_path("a_envoyer_couvert");
    let journal = Journal::ouvrir(path.to_string_lossy().into_owned()).unwrap();
    journal.ajouter(0.0, 1_793_000_000_000, Some(true)).unwrap();
    journal.ajouter(320.5, 1_793_000_060_000, None).unwrap();

    let à_envoyer = journal.mesures_apres(None).unwrap();

    let proximités: Vec<Option<bool>> = à_envoyer
        .iter()
        .map(|mesure| mesure.telephone_couvert)
        .collect();
    assert_eq!(proximités, vec![Some(true), None]);
}
