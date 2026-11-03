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

    journal.ajouter(320.5, 1_793_000_000_000).unwrap();

    let database = Connection::open(&path).unwrap();
    let écrite: (f32, i64) = database
        .query_row("SELECT valeur_lux, instant_ms FROM mesure", [], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })
        .unwrap();
    assert_eq!(écrite, (320.5, 1_793_000_000_000));
}

#[test]
fn le_relevé_du_jour_se_calcule_avec_les_mesures_du_journal() {
    let path = fresh_database_path("releve");
    let journal = Journal::ouvrir(path.to_string_lossy().into_owned()).unwrap();
    let minute_ms = 60_000;
    let heure_ms = 60 * minute_ms;
    let coucher_ms = 1_793_000_000_000;
    let lever_ms = coucher_ms + 8 * heure_ms;
    let coucher_souhaite_ms = lever_ms + 16 * heure_ms;
    journal.ajouter(50.0, coucher_ms - 60 * minute_ms).unwrap();
    journal.ajouter(5.0, coucher_ms - 55 * minute_ms).unwrap();

    let relevé = journal
        .releve_du_jour(coucher_ms, lever_ms, coucher_souhaite_ms)
        .unwrap();

    assert_eq!(
        relevé.lumiere_du_soir.temps_au_dessus_du_seuil_ms,
        5 * minute_ms
    );
}
