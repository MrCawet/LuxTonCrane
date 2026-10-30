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
