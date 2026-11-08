use std::path::PathBuf;

use agenda::{AgendaSqlite, Continuité, Endormissement, Fatigue, Nuit};

fn fresh_database_path(test_name: &str) -> PathBuf {
    let file_name = format!("agenda_{}_{}.db", test_name, std::process::id());
    let path = std::env::temp_dir().join(file_name);
    let _ = std::fs::remove_file(&path);
    path
}

fn nuit_du(jour: &str, endormissement: Endormissement) -> Nuit {
    Nuit {
        jour: jour.to_string(),
        endormissement,
        continuité: Continuité::RéveilBref,
        coucher_ms: 1793000000000,
        lever_ms: 1793028800000,
    }
}

#[test]
fn une_nuit_gardée_se_retrouve_à_la_date_de_son_lever() {
    let path = fresh_database_path("garder");
    let agenda = AgendaSqlite::open(&path).unwrap();
    let nuit = nuit_du("2026-11-08", Endormissement::Calme);

    agenda.garder(&nuit).unwrap();

    let relue = AgendaSqlite::open(&path)
        .unwrap()
        .nuit("2026-11-08")
        .unwrap();
    assert_eq!(relue, Some(nuit));
}

#[test]
fn une_nuit_pas_encore_notée_n_existe_pas() {
    let agenda = AgendaSqlite::open(&fresh_database_path("absente")).unwrap();

    assert_eq!(agenda.nuit("2026-11-08").unwrap(), None);
}

#[test]
fn une_nuit_notée_à_nouveau_remplace_l_ancienne() {
    let agenda = AgendaSqlite::open(&fresh_database_path("remplacer")).unwrap();
    agenda
        .garder(&nuit_du("2026-11-08", Endormissement::Agité))
        .unwrap();
    let corrigée = nuit_du("2026-11-08", Endormissement::Immédiat);

    agenda.garder(&corrigée).unwrap();

    assert_eq!(agenda.nuit("2026-11-08").unwrap(), Some(corrigée));
}

#[test]
fn le_coucher_souhaité_est_23_h_tant_qu_il_n_est_pas_réglé() {
    let path = fresh_database_path("coucher_souhaite");
    let agenda = AgendaSqlite::open(&path).unwrap();
    assert_eq!(agenda.coucher_souhaité().unwrap(), "23:00");

    agenda.régler_le_coucher_souhaité("22:30").unwrap();

    let relu = AgendaSqlite::open(&path)
        .unwrap()
        .coucher_souhaité()
        .unwrap();
    assert_eq!(relu, "22:30");
}

#[test]
fn chaque_niveau_se_retrouve_par_son_nom() {
    for endormissement in [
        Endormissement::Immédiat,
        Endormissement::Calme,
        Endormissement::Agité,
        Endormissement::Abandonné,
    ] {
        assert_eq!(
            Endormissement::depuis_le_nom(endormissement.nom()),
            Some(endormissement)
        );
    }
    for continuité in [
        Continuité::SansRéveil,
        Continuité::RéveilBref,
        Continuité::RéveilLong,
        Continuité::NuitInterrompue,
    ] {
        assert_eq!(
            Continuité::depuis_le_nom(continuité.nom()),
            Some(continuité)
        );
    }
    assert_eq!(Endormissement::nom(&Endormissement::Agité), "agite");
    assert_eq!(Continuité::nom(&Continuité::SansRéveil), "sans_reveil");
    assert_eq!(Endormissement::depuis_le_nom("inconnu"), None);
}

#[test]
fn la_fatigue_du_jour_se_garde_même_sans_la_nuit_et_se_corrige() {
    let path = fresh_database_path("fatigue");
    let agenda = AgendaSqlite::open(&path).unwrap();
    assert_eq!(agenda.fatigue("2026-11-08").unwrap(), None);

    agenda
        .garder_la_fatigue("2026-11-08", Fatigue::Gênante)
        .unwrap();
    agenda
        .garder_la_fatigue("2026-11-08", Fatigue::Légère)
        .unwrap();

    let relue = AgendaSqlite::open(&path)
        .unwrap()
        .fatigue("2026-11-08")
        .unwrap();
    assert_eq!(relue, Some(Fatigue::Légère));
}

#[test]
fn chaque_niveau_de_fatigue_se_retrouve_par_son_nom() {
    for fatigue in [
        Fatigue::Aucune,
        Fatigue::Légère,
        Fatigue::Gênante,
        Fatigue::Somnolence,
    ] {
        assert_eq!(Fatigue::depuis_le_nom(fatigue.nom()), Some(fatigue));
    }
    assert_eq!(Fatigue::Gênante.nom(), "genante");
}

#[test]
fn l_objectif_de_chaque_note_est_tenu_par_ses_deux_premiers_niveaux() {
    assert!(Endormissement::Immédiat.objectif_tenu());
    assert!(Endormissement::Calme.objectif_tenu());
    assert!(!Endormissement::Agité.objectif_tenu());
    assert!(!Endormissement::Abandonné.objectif_tenu());
    assert!(Continuité::SansRéveil.objectif_tenu());
    assert!(Continuité::RéveilBref.objectif_tenu());
    assert!(!Continuité::RéveilLong.objectif_tenu());
    assert!(!Continuité::NuitInterrompue.objectif_tenu());
    assert!(Fatigue::Aucune.objectif_tenu());
    assert!(Fatigue::Légère.objectif_tenu());
    assert!(!Fatigue::Gênante.objectif_tenu());
    assert!(!Fatigue::Somnolence.objectif_tenu());
}

const MINUTE_MS: i64 = 60 * 1000;
const HEURE_MS: i64 = 60 * MINUTE_MS;

fn nuit_au_lit_pendant(durée_ms: i64) -> Nuit {
    let coucher_ms = 1793000000000;
    Nuit {
        coucher_ms,
        lever_ms: coucher_ms + durée_ms,
        ..nuit_du("2026-11-08", Endormissement::Calme)
    }
}

#[test]
fn la_quantité_de_sommeil_est_le_temps_au_lit_et_doit_dépasser_7_h() {
    let longue = nuit_au_lit_pendant(7 * HEURE_MS + 15 * MINUTE_MS).quantité_de_sommeil();
    let tout_juste_7_h = nuit_au_lit_pendant(7 * HEURE_MS).quantité_de_sommeil();

    assert_eq!(longue.durée_ms, 7 * HEURE_MS + 15 * MINUTE_MS);
    assert!(longue.objectif_tenu);
    assert_eq!(tout_juste_7_h.durée_ms, 7 * HEURE_MS);
    assert!(!tout_juste_7_h.objectif_tenu);
}
