/** Un niveau d'une échelle de l'agenda : `nom` est celui que le serveur écrit. */
export interface Niveau {
  nom: string
  titre: string
  cequiSestPassé: string
}

/** Le titre du niveau noté, tel que l'échelle l'écrit. */
export function titreDuNiveau(échelle: Niveau[], nom: string): string {
  const niveau = échelle.find((candidat) => candidat.nom === nom)
  return niveau === undefined ? nom : niveau.titre
}

// Les échelles de CONCEPTION.md, section « Agenda du sommeil ».
export const ENDORMISSEMENT: Niveau[] = [
  { nom: 'immediat', titre: 'Immédiat', cequiSestPassé: "je ne me souviens pas d'avoir attendu" },
  {
    nom: 'calme',
    titre: 'Calme',
    cequiSestPassé: "j'ai attendu un peu, sans m'agiter ni me dire « je n'arrive pas à dormir »",
  },
  {
    nom: 'agite',
    titre: 'Agité',
    cequiSestPassé: "je me suis retourné plusieurs fois, ou je me suis dit « je n'arrive pas à dormir »",
  },
  {
    nom: 'abandonne',
    titre: 'Abandonné',
    cequiSestPassé:
      "j'ai renoncé un moment : je me suis levé, j'ai repris le téléphone ou j'ai rallumé la lumière",
  },
]

export const CONTINUITÉ: Niveau[] = [
  { nom: 'sans_reveil', titre: 'Sans réveil', cequiSestPassé: 'je ne me souviens d\'aucun réveil' },
  {
    nom: 'reveil_bref',
    titre: 'Réveil bref',
    cequiSestPassé: 'réveillé, puis rendormi aussitôt, sans avoir le temps de penser',
  },
  {
    nom: 'reveil_long',
    titre: 'Réveil long',
    cequiSestPassé: "réveillé, et j'ai mis du temps à me rendormir : j'ai pensé, je me suis retourné",
  },
  {
    nom: 'nuit_interrompue',
    titre: 'Nuit interrompue',
    cequiSestPassé: "je me suis levé, j'ai pris le téléphone, ou je suis resté éveillé longtemps",
  },
]

// Section « Fatigue de la journée ».
export const FATIGUE: Niveau[] = [
  { nom: 'aucune', titre: 'Aucune', cequiSestPassé: "je n'ai pas pensé à ma fatigue" },
  { nom: 'legere', titre: 'Légère', cequiSestPassé: 'je l\'ai sentie, sans que ça change ce que je faisais' },
  {
    nom: 'genante',
    titre: 'Gênante',
    cequiSestPassé:
      "elle m'a ralenti : j'ai dû faire une pause, prendre un café ou relire, ou je me suis forcé à continuer",
  },
  {
    nom: 'somnolence',
    titre: 'Somnolence',
    cequiSestPassé: "j'ai lutté pour ne pas m'endormir, ou je me suis endormi sans le vouloir",
  },
]
