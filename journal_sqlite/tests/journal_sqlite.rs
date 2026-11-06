use std::path::PathBuf;

use journal::ajouter;
use journal_sqlite::SqliteJournal;
use mesure::Mesure;
use rusqlite::Connection;

fn fresh_database_path(test_name: &str) -> PathBuf {
    let file_name = format!("journal_sqlite_{}_{}.db", test_name, std::process::id());
    let path = std::env::temp_dir().join(file_name);
    let _ = std::fs::remove_file(&path);
    path
}

#[test]
fn ajouter_une_mesure_l_écrit_dans_la_base() {
    let path = fresh_database_path("ajouter");
    let mut journal = SqliteJournal::open(&path).unwrap();

    ajouter(
        Mesure {
            valeur_lux: 320.5,
            instant_ms: 1_793_000_000_000,
            téléphone_couvert: None,
        },
        &mut journal,
    )
    .unwrap();

    let database = Connection::open(&path).unwrap();
    let écrite: (f32, i64) = database
        .query_row("SELECT valeur_lux, instant_ms FROM mesure", [], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })
        .unwrap();
    assert_eq!(écrite, (320.5, 1_793_000_000_000));
}

#[test]
fn l_historique_rend_les_mesures_dans_l_ordre_chronologique() {
    let path = fresh_database_path("historique");
    let mut journal = SqliteJournal::open(&path).unwrap();
    let plus_récente = Mesure {
        valeur_lux: 80.0,
        instant_ms: 1_793_000_060_000,
        téléphone_couvert: None,
    };
    let plus_ancienne = Mesure {
        valeur_lux: 320.5,
        instant_ms: 1_793_000_000_000,
        téléphone_couvert: None,
    };
    ajouter(plus_récente, &mut journal).unwrap();
    ajouter(plus_ancienne, &mut journal).unwrap();

    let historique = journal.historique().unwrap();

    let attendu = vec![
        Mesure {
            valeur_lux: 320.5,
            instant_ms: 1_793_000_000_000,
            téléphone_couvert: None,
        },
        Mesure {
            valeur_lux: 80.0,
            instant_ms: 1_793_000_060_000,
            téléphone_couvert: None,
        },
    ];
    assert_eq!(historique, attendu);
}

#[test]
fn l_historique_rend_si_le_téléphone_était_couvert() {
    let path = fresh_database_path("couvert");
    let mut journal = SqliteJournal::open(&path).unwrap();
    let couverte = Mesure {
        valeur_lux: 0.0,
        instant_ms: 1_793_000_000_000,
        téléphone_couvert: Some(true),
    };
    let découverte = Mesure {
        valeur_lux: 320.5,
        instant_ms: 1_793_000_060_000,
        téléphone_couvert: Some(false),
    };
    ajouter(couverte, &mut journal).unwrap();
    ajouter(découverte, &mut journal).unwrap();

    let historique = journal.historique().unwrap();

    let attendu = vec![
        Mesure {
            valeur_lux: 0.0,
            instant_ms: 1_793_000_000_000,
            téléphone_couvert: Some(true),
        },
        Mesure {
            valeur_lux: 320.5,
            instant_ms: 1_793_000_060_000,
            téléphone_couvert: Some(false),
        },
    ];
    assert_eq!(historique, attendu);
}

#[test]
fn une_mesure_gardée_avant_la_proximité_a_un_téléphone_couvert_inconnu() {
    let path = fresh_database_path("avant_la_proximite");
    let ancienne_base = Connection::open(&path).unwrap();
    ancienne_base
        .execute_batch(
            "CREATE TABLE mesure (
                valeur_lux REAL NOT NULL,
                instant_ms INTEGER NOT NULL
            );
            INSERT INTO mesure (valeur_lux, instant_ms) VALUES (320.5, 1793000000000);",
        )
        .unwrap();
    drop(ancienne_base);

    let journal = SqliteJournal::open(&path).unwrap();
    let historique = journal.historique().unwrap();

    let attendu = vec![Mesure {
        valeur_lux: 320.5,
        instant_ms: 1_793_000_000_000,
        téléphone_couvert: None,
    }];
    assert_eq!(historique, attendu);
}
