use journal::{Journal, ajouter, ÉchecDEnregistrement};
use mesure::Mesure;

struct JournalEnMémoire {
    enregistrées: Vec<Mesure>,
}

impl Journal for JournalEnMémoire {
    fn enregistrer(&mut self, mesure: Mesure) -> Result<(), ÉchecDEnregistrement> {
        self.enregistrées.push(mesure);
        Ok(())
    }
}

struct JournalEnPanne;

impl Journal for JournalEnPanne {
    fn enregistrer(&mut self, _mesure: Mesure) -> Result<(), ÉchecDEnregistrement> {
        Err(ÉchecDEnregistrement {
            raison: String::from("disque plein"),
        })
    }
}

#[test]
fn le_journal_garde_la_mesure_enregistrée() {
    let mut journal = JournalEnMémoire {
        enregistrées: Vec::new(),
    };

    journal
        .enregistrer(Mesure {
            valeur_lux: 320.5,
            instant_ms: 1_793_000_000_000,
        })
        .unwrap();

    let attendue = Mesure {
        valeur_lux: 320.5,
        instant_ms: 1_793_000_000_000,
    };
    assert_eq!(journal.enregistrées, vec![attendue]);
}

#[test]
fn ajouter_une_mesure_l_enregistre_dans_le_journal() {
    let mut journal = JournalEnMémoire {
        enregistrées: Vec::new(),
    };

    ajouter(
        Mesure {
            valeur_lux: 120.0,
            instant_ms: 1_793_000_060_000,
        },
        &mut journal,
    )
    .unwrap();

    let attendue = Mesure {
        valeur_lux: 120.0,
        instant_ms: 1_793_000_060_000,
    };
    assert_eq!(journal.enregistrées, vec![attendue]);
}

#[test]
fn ajouter_une_mesure_renvoie_l_échec_du_journal() {
    let mut journal = JournalEnPanne;

    let résultat = ajouter(
        Mesure {
            valeur_lux: 120.0,
            instant_ms: 1_793_000_060_000,
        },
        &mut journal,
    );

    let attendu = Err(ÉchecDEnregistrement {
        raison: String::from("disque plein"),
    });
    assert_eq!(résultat, attendu);
}
