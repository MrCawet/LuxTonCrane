use journal::{Journal, ajouter_une_mesure};
use mesure::Mesure;

struct JournalEnMémoire {
    enregistrées: Vec<Mesure>,
}

impl Journal for JournalEnMémoire {
    fn enregistrer(&mut self, mesure: Mesure) {
        self.enregistrées.push(mesure);
    }
}

#[test]
fn le_journal_garde_la_mesure_enregistrée() {
    let mut journal = JournalEnMémoire {
        enregistrées: Vec::new(),
    };

    journal.enregistrer(Mesure {
        valeur_lux: 320.5,
        instant_ms: 1_793_000_000_000,
    });

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

    ajouter_une_mesure(
        Mesure {
            valeur_lux: 120.0,
            instant_ms: 1_793_000_060_000,
        },
        &mut journal,
    );

    let attendue = Mesure {
        valeur_lux: 120.0,
        instant_ms: 1_793_000_060_000,
    };
    assert_eq!(journal.enregistrées, vec![attendue]);
}
