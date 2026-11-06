# Backlog LuxTonCrane

Backlog partagé **opérateur humain ↔ agent IA**. Un ticket = un fichier Markdown versionné dans ce dépôt.
Pas d'outil externe : le Markdown + git font foi.

[CAP.md](../CAP.md) porte la mission et la vision, [CONCEPTION.md](../CONCEPTION.md) les notes de conception,
[RECHERCHE.md](../RECHERCHE.md) ce que dit la science, [GLOSSAIRE.md](../GLOSSAIRE.md) les termes du métier : c'est de
la matière brute. Ici, les tickets qui en sont tirés, avec leur priorité.

---

## 1. Structure

```
backlog/
├── README.md              ← ce document (la norme)
├── equipe.md              personnes qui travaillent sur le projet et leur rôle
├── epiques/               EP-xxxx   regroupement de tickets autour d'un même objectif
├── requetes/              REQ-xxxx  demandes brutes, NON qualifiées
├── anomalies/             ANO-xxxx  le produit ne fait pas ce qu'il devrait
├── us/                    US-xxxx   nouveau comportement / évolution fonctionnelle
├── dette-technique/       DT-xxxx   coût technique interne, invisible pour l'utilisateur
├── _assets/               captures d'écran et pièces jointes (nommées <ticket>-<sujet>.<ext>)
├── _source/               matière brute archivée (notes, exports) dont les tickets ont été extraits
└── _archives/             tickets terminés et leurs annexes, même arborescence (us/, _assets/…), cf. § Archives
```

Le cap du projet (mission, vision, principes, objectifs en cours, roadmap) n'est pas dans ce dossier : il vit dans
[CAP.md](../CAP.md), à la racine du dépôt.

Un dossier n'existe qu'une fois son premier fichier créé.
`backlog/` vit dans le même dépôt git que le code : un ticket évolue dans les mêmes commits que le code — voir §7.3.

### Données sensibles

Tout ce qui entre dans le backlog (ticket, `_source/`, `_assets/`) est **nettoyé avant le commit**, par l'humain comme par l'agent :
- informations personnelles (téléphone, e-mail, adresse, données de santé, notes et heures de sommeil réelles, relevés
  de lumière horodatés réels…) → `[téléphone]`, `[e-mail]`, `[données de santé]`… ;
  une personne est désignée par son prénom ou son rôle ; un exemple de relevé ou de nuit s'écrit avec des valeurs inventées ;
- secrets (clé de signature, mot de passe, token, URL signée) → `[clé de signature]`, `[mot de passe de la clé]`… ;
- captures d'écran : floutées ou recadrées avant d'être rangées.

Une donnée sensible déjà commitée reste dans l'historique git : la signaler à l'humain, et révoquer tout secret exposé.

### Archives

`_archives/` reçoit les tickets en statut `termine`, épiques comprises, **quand l'opérateur demande l'archivage** :
jamais d'office. Archiver, c'est déplacer : rien n'est supprimé, et le ticket garde son contenu, son identifiant et
son statut.

- `_archives/` reprend l'arborescence du backlog : `us/US-0001-….md` devient `_archives/us/US-0001-….md`,
  `_assets/US-0001-….png` devient `_archives/_assets/US-0001-….png`.
- **Les annexes suivent leur ticket** (`_assets/`, `_source/`) : celles nommées d'après son identifiant, et celles que
  son corps cite. Une annexe qu'un ticket resté cite encore reste en place ; elle partira avec le dernier ticket qui
  s'en sert.
- Le déplacement se fait avec `git mv`. Les liens relatifs qu'il casse sont réparés, dans les fichiers restés comme
  dans les fichiers archivés ; les identifiants du front-matter ne changent pas.
- Un `abandonne`, ou une requête close dont tous les tickets dérivés sont archivés, ne s'archive que sur accord de
  l'opérateur.
- Un ticket archivé qu'on rouvre revient à sa place, avec ses annexes.

## 2. Les 4 types

| Type | Dossier | Préfixe | Définition | Qui l'écrit |
|------|---------|---------|------------|-------------|
| **Requête** | `requetes/` | `REQ` | Demande **non qualifiée** telle qu'elle arrive (note, conversation, retour d'usage). Décrit un *besoin* ou une *gêne*, pas une solution. | Opérateur humain, à la réception |
| **Anomalie** | `anomalies/` | `ANO` | Écart entre le comportement attendu et le comportement observé. Reproductible. | Humain ou agent, après qualification |
| **User Story** | `us/` | `US` | Évolution fonctionnelle : le produit ne l'a jamais fait, on veut qu'il le fasse. | Humain ou agent, après qualification |
| **Dette technique** | `dette-technique/` | `DT` | Travail technique sans valeur fonctionnelle directe, qui réduit le risque ou le coût futur. | Humain ou agent |

> **La frontière ANO / US** : si le comportement actuel contredit une spec, une demande déjà livrée ou une attente légitime → **ANO**. Si personne ne l'a jamais demandé ni implémenté → **US**.

### Épiques

Une épique (`epiques/`, préfixe `EP`) n'est pas un ticket : elle **regroupe** des tickets de tout type autour d'un même objectif,
souvent issu d'une requête trop large pour un seul ticket. Elle ne s'implémente pas, ne porte ni priorité ni critères d'acceptation.

- Un ticket appartient au plus à une épique, déclarée par le champ `epique: EP-xxxx`.
- L'épique liste ses tickets et, dans `## À découper`, les sujets **gardés sous le coude** : identifiés, mais pas encore
  transformés en tickets, parce qu'un ticket de l'épique doit d'abord être terminé et peut les changer. Chaque sujet
  indique ce qu'il attend.
- Statut : `nouveau` → `en_cours` (un de ses tickets est commencé) → `termine` (tous terminés, plus rien à découper) ou `abandonne`.

## 3. Les requêtes fonctionnent différemment

Une requête **n'est jamais implémentée directement**. C'est une boîte de réception.

```
REQ (nouvelle)
   └─ qualification ─┬─→ 1..n tickets ANO / US / DT   → REQ close, statut `qualifiee`
                     ├─→ action d'exploitation (aucun code)  → REQ close, statut `qualifiee`
                     ├─→ doublon d'un ticket existant        → REQ close, statut `doublon`
                     └─→ hors périmètre / refusée            → REQ close, statut `refusee`
```

Règles :

1. **On ne réécrit pas le texte de l'émetteur.** Le verbatim est recopié tel quel dans la section `## Verbatim`, seules les données sensibles en sont retirées (§1). La reformulation va dans `## Analyse`.
2. **Une requête peut contenir plusieurs sujets.** Elle est alors découpée en plusieurs tickets ; la requête reste une seule et liste ses enfants.
3. **Une requête ne porte ni estimation ni priorité métier définitive** avant qualification.
4. **Les zones d'ombre se tracent, elles ne s'inventent pas.** Toute ambiguïté va dans `## Questions ouvertes` et doit être levée auprès de l'émetteur avant de passer le ticket dérivé en `pret`.
5. La requête garde l'**émetteur** (prénom ou rôle) : c'est lui qu'on recontacte et qui valide la livraison. Ses coordonnées restent hors du dépôt.

### Équipe

`equipe.md` liste les personnes qui travaillent sur le projet : prénom, rôle, ce qu'on leur demande (valider, tester,
arbitrer…). Pas de coordonnées. Une personne citée dans une requête et absente de la liste y est ajoutée, rôle
`à préciser` tant que l'opérateur ne l'a pas donné.

## 4. Nommage et numérotation

```
<PREFIXE>-<NNNN>-<slug-kebab-case>.md      ex: ANO-0002-releve-vide-apres-minuit.md
```

- Compteur **indépendant par type**, incrémental, jamais réutilisé. Les tickets archivés (`_archives/`) comptent.
- Un ticket supprimé ou abandonné garde son numéro (statut `abandonne`).
- Le slug est court (3 à 6 mots), en minuscules, sans accent, en mots entiers.

## 5. Front-matter obligatoire

Toujours en tête du fichier, YAML. C'est ce que lisent les agents ; le corps est pour les humains.

```yaml
---
id: ANO-0002
type: anomalie            # epique | requete | anomalie | us | dette-technique
titre: Le relevé du jour est vide après minuit
statut: qualifie
priorite: essentiel       # vital | essentiel | confort | luxe, cf. § Priorités
cree_le: 2026-11-06
maj_le: 2026-11-06
source: REQ-0003          # requête d'origine, ou "interne"
emetteur: <prénom|rôle>  # pour les requêtes et tickets issus d'une requête
composants: [releve]      # mesure | journal | journal_sqlite | releve | pont | android | serveur | web | ci
tickets_lies: [REQ-0003]
objectif: O1              # objectif du cap servi par le ticket, ou « aucun »
niveau: 1                 # 0 direct | 1 fonctionnalité | 2 direction | 3 investigation, cf. §6 Niveaux
epique: EP-0001           # facultatif, cf. §2 Épiques
---
```

Champs supplémentaires selon le type :

- **epique** : `tickets: [...]` ; pas de `priorite` ni de `niveau`
- **requete** : `canal` (conversation / note / usage / autre), `recue_le`, `tickets_derives: [...]`, `resolution` (qualifiee / doublon / refusee / exploitation) ; pas d'`objectif` ni de `niveau`, qui se fixent à la qualification
- **us** : `valeur` (une phrase : pour qui, pourquoi)
- **dette-technique** : `risque` (faible / moyen / eleve), `impact` (une phrase)

### Statuts

| Statut | Signification |
|--------|---------------|
| `nouveau` | Créé, pas encore instruit |
| `qualifie` | Analysé, périmètre et impact compris ; pour une REQ : découpée, close |
| `pret` | Prêt selon son niveau (§6 Niveaux) : aucune question ouverte dont la réponse change le code à écrire → implémentable |
| `en_cours` | Quelqu'un (humain ou agent) travaille dessus |
| `a_valider` | Livré sur une branche ou en APK de test, attente de validation |
| `termine` | Validé, livré (release GitHub) |
| `bloque` | Attente d'une décision ou d'un tiers — la raison est obligatoire dans le corps |
| `abandonne` | Ne sera pas fait — la raison est obligatoire dans le corps |
| `doublon` / `refusee` | Réservés aux requêtes |

### Priorités

Une MoSCoW, en français. Elle range les **tickets** ; les objectifs, eux, se rangent par horizon dans la roadmap
([CAP.md](../CAP.md)).

| Priorité | Signification | Attente |
|---|---|---|
| `vital` | Sans lui, le produit n'a pas de raison d'être, ou le service s'arrête : panne, perte de données, faille. Chaque heure est une perte sèche | avant tout le reste |
| `essentiel` | Son absence compromet le bon fonctionnement : le service tourne, mais mal. Mauvaise productivité, contournement pénible, manque à gagner | prochaine itération |
| `confort` | Améliore l'usage ; on s'en passe sans dommage | quand la place se libère |
| `luxe` | Pas maintenant. On attend : s'il est vraiment utile, il remontera ; sinon il sera abandonné à une revue | jamais planifié |

Un ticket qui sert un objectif du court terme n'est pas vital pour autant : sa priorité se juge sur cette table.
L'agent la propose avec une ligne de motif, l'opérateur la tranche.

## 6. Corps du ticket

**`## Provenance` ouvre tout ticket, sans exception.** Le front-matter donne les identifiants
(`source`, `emetteur`) ; cette section donne ce qu'un identifiant ne dit pas : d'où sort le
ticket et ce qu'on a décidé en l'écrivant. Elle contient, en une à quatre lignes :

1. **L'origine** — la requête dont il dérive (lien relatif), ou « interne » avec ce qui l'a
   déclenché (analyse du code, incident, note de conception, décision).
2. **La matière brute** — canal, émetteur, date de réception, et le fichier (`CONCEPTION.md`,
   `_source/`…) où le texte d'origine est archivé, **avec sa section exacte**. C'est ce qui permet de remonter au
   mot près à ce qui a été dit, des mois plus tard.
3. **La décision de qualification**, datée, dès qu'il y en a eu une : requalification ANO ↔ US,
   découpage d'une requête en plusieurs tickets, fusion, changement de priorité. **Tout écart
   entre ce que demandait l'émetteur et ce que dit le ticket se justifie ici** — c'est la seule
   trace de l'interprétation faite, et elle doit pouvoir être contestée.

Un ticket sans provenance vérifiable ne passe pas en `pret`.

La provenance ne se transforme pas en journal :
- une décision prise en conversation s'inscrit dans le fil `## Questions ouvertes` (question et réponse), qui est la
  seule archive des échanges ; sa conséquence va dans le besoin, le comportement ou les critères ;
- un choix d'implémentation va dans le commit du code ;
- un ticket modifié par un autre voit son corps mis à jour et le lien ajouté dans `tickets_lies`, sans récit du
  changement : l'historique est dans git.

Sections suivantes (celles qui ne s'appliquent pas sont omises, pas laissées vides) :

- **Épique** : `## Provenance` · `## Objectif` · `## Tickets` · `## À découper` · `## Questions ouvertes`
- **Requête** : `## Provenance` · `## Verbatim` · `## Analyse` · `## Questions ouvertes` · `## Suite donnée`
- **Anomalie** : `## Provenance` · `## Contexte` · `## Comportement observé` · `## Comportement attendu` · `## Reproduction` · `## Piste technique` · `## Critères d'acceptation`
- **US** : `## Provenance` · `## Besoin` · `## Comportement attendu` · `## Hors périmètre` · `## Piste technique` · `## Critères d'acceptation` · `## Questions ouvertes`
- **Dette technique** : `## Provenance` · `## Constat` · `## Risque` · `## Cible` · `## Étapes` · `## Critères d'acceptation`

Le `## Besoin` d'une US s'ouvre sur une phrase qui dit pour qui, quoi et pourquoi :
**En tant que** <rôle>, **je veux** <capacité>, **afin de** <bénéfice>.
Un développement suit seulement si cette phrase ne suffit pas à exprimer le besoin.

Les critères d'acceptation sont des cases à cocher vérifiables, pas des intentions. Les termes du métier s'y écrivent
comme dans [GLOSSAIRE.md](../GLOSSAIRE.md).

### Niveaux

Le champ `niveau` (anomalie, US, dette) dit combien le ticket est cadré. Il ne dépend pas du type ; l'agent le
propose, l'opérateur le tranche.

| Niveau | Ce qu'on sait | Sections | Prêt quand |
|---|---|---|---|
| `0` direct | La demande est la solution : une action, un endroit, rien à trancher | `## Provenance` · `## Critères d'acceptation` (1 à 3) | Tout de suite |
| `1` fonctionnalité | Fonctionnalité claire, périmètre identifiable, des règles à trancher | Celles du type | Critères écrits, aucune question ouverte qui change le code à écrire |
| `2` direction | Problème clair, solution vague : la direction est connue | Une épique porte la direction (`## Objectif`) ; seule la première tranche est un ticket, de niveau 0 ou 1 | La tranche est prête à son niveau |
| `3` investigation | Problème mal défini, ou solution en réflexion | `## Provenance` · `## Problème` · `## Hypothèses` · `## Expérience` · `## Critères d'acceptation` · `## Questions ouvertes` | L'expérience est décrite |

Au niveau 2, les tranches suivantes restent une ligne dans `## À découper` de l'épique jusqu'à la revue de la
précédente. Un ticket de niveau 3 livre un problème bien posé et les sujets de niveau 2 qui en sortent, pas une
architecture ; il ne bloque pas les autres tickets.

Une hypothèse (« Hypothèse : X ») est un choix par défaut annoncé dans le ticket, que l'opérateur peut renverser et
que l'usage confirmera ; ce n'est pas un besoin inventé.

### Questions ouvertes : un fil d'échange

`## Questions ouvertes` est le lieu de l'échange entre l'agent et l'émetteur (ou l'opérateur), dans le ticket même :

- une question par puce, posée par l'agent ;
- la réponse s'écrit juste sous la question, en bloc de code (`` `…` `` ou ```` ``` ````), pour la distinguer ;
- une question soulevée par une réponse s'ajoute en sous-puce sous cette réponse, et appelle à son tour la sienne ;
- une question est close quand elle a sa réponse sans sous-question en attente. On ne supprime rien : le fil reste la trace de l'interprétation.

Les conclusions du fil se reportent là où elles servent : `## Analyse` et tickets dérivés pour une requête, `## Besoin` ou `## Critères d'acceptation` pour une US.

## 7. Règles pour l'agent IA

1. **L'opérateur désigne le ticket.** L'agent ne choisit pas ce sur quoi il travaille et n'ouvre pas un ticket de sa propre initiative : il traite celui qu'on lui confie, quand on le lui confie. Une fois le ticket désigné, le lire en entier avant de coder — et s'il est en `nouveau` ou porte une question ouverte bloquante, **ne pas implémenter** : qualifier et rendre la main.
2. **Ne jamais coder depuis une requête.** Créer d'abord le ou les tickets dérivés.
3. **Le ticket se met à jour avec le code, dans le même commit.** Le message porte l'id du ticket juste après le préfixe : `feat: US-0001 l'accueil affiche…`, `fix: ANO-0002 …`, `test: US-0001 …`. La mise à jour du ticket (`statut`, `maj_le`, cases cochées) accompagne le commit de code qui la justifie, jamais un commit à part.
4. **Ne jamais inventer un besoin.** Ce qui n'est pas dans le verbatim et n'a pas été confirmé va dans `## Questions ouvertes`.
5. **Tracer sa propre interprétation.** Toute requalification, tout découpage, toute priorité choisie par l'agent s'écrit dans `## Provenance` du ticket, datée et motivée. L'opérateur doit pouvoir la relire et la renverser sans rouvrir la note d'origine.
6. **Respecter [CLAUDE.md](../CLAUDE.md)** (façon de travailler, règles de code, lisibilité, nommage et glossaire, TDD école de Détroit, architecture) et [.claude/rules/rust.md](../.claude/rules/rust.md).
7. **Aucune livraison.** L'APK est livré par le workflow `livraison.yml`, que l'opérateur déclenche à la main depuis `main` : cela reste à l'humain. L'agent ne touche jamais aux secrets de signature.
8. Une pièce jointe se range dans `_assets/` sous `<ticket>-<sujet>.<ext>` et se référence en relatif depuis le ticket : `![légende](../_assets/<nom-du-fichier>)`.

## 8. Créer un ticket

Partir d'un ticket existant du même type (ou des §5 et §6), puis renseigner le front-matter
(le `id` doit correspondre au nom de fichier) et le corps.

## 9. Index

L'index est le système de fichiers. Pour lister, depuis `backlog/` :

```bash
grep -h "^id:\|^statut:\|^titre:" */[A-Z]*.md | paste - - -    # vue rapide
grep -rl "statut: pret" */[A-Z]*.md                              # implémentables
grep -rl "priorite: vital" */[A-Z]*.md                           # à traiter avant tout
grep -rl "epique: EP-0001" */[A-Z]*.md                           # tickets d'une épique
```

Ces commandes ne voient pas les archives. Pour les inclure (dernier numéro d'un type, recherche d'un ancien ticket) :

```bash
ls */US-* _archives/*/US-* 2>/dev/null                           # toutes les US, archivées comprises
grep -h "^id:\|^statut:\|^titre:" _archives/*/[A-Z]*.md | paste - - -    # vue rapide des archives
```
