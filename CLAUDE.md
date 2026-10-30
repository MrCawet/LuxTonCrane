# CLAUDE.md

## Repères

- [CAP.md](CAP.md) : la vision du projet.
- [CONCEPTION.md](CONCEPTION.md) : les notes de conception, provisoires.
- [GLOSSAIRE.md](GLOSSAIRE.md) : les termes du métier.
- L'application est en Kotlin (capteur de lumière, tâche de fond, toutes les API Android) et en Rust (le métier : mesures, rapports).
- [.claude/rules/rust.md](.claude/rules/rust.md) : règles propres au Rust, à lire avant d'écrire du Rust. Chargées automatiquement dès qu'un fichier `.rs` ou `Cargo.toml` est touché.
- Le métier est un workspace Cargo à la racine, une crate par sujet (`mesure/`, `journal/`) : `cargo test` à la racine.

Mettre à jour cette section à chaque changement qu'une nouvelle session perdrait du temps à retrouver : commandes fréquentes, particularités du projet, etc.

## Façon de travailler

L'agent et l'opérateur travaillent de manière itérative :
1. l'opérateur cadre le besoin ;
2. l'agent implémente et livre ;
3. on recommence autant que nécessaire.

L'agent fait donc le plus petit changement possible qui répond à la demande, sans faire de suppositions. Ce changement est complet par rapport à la demande et conforme aux règles de ce fichier.

- **Une branche nommée d'après son sujet.** En mots français entiers, séparés par des tirets, sans accents : `mesure-de-la-lumiere`. Si la session impose un nom généré (`ccr-…`), l'agent renomme la branche avant le premier push.
- **Pas d'anticipation.** Aucun cas limite, test, code ou contrat qui n'a pas été demandé : l'agent le propose, il ne l'écrit pas. De même, il ne pose que les questions nécessaires à la demande en cours.
- **Relire avant de livrer.** Avant de montrer un changement, l'agent relit son diff au regard de [Nommer](#nommer), des règles de code, de lisibilité et de [.claude/rules/rust.md](.claude/rules/rust.md), et corrige ce qui n'y répond pas.
- **Trier les commits avant de fusionner dans `main`.** L'agent recrée la branche depuis `main` en y reprenant par `git cherry-pick` seulement les commits utiles : il écarte un commit qu'un commit suivant défait, avec celui qui le défait (un test ajouté puis retiré), ou fusionne la correction dans le commit d'origine (un terme écrit faux puis corrigé), en gardant le cycle `test:`, `feat:`, `refactor:` lisible. Un changement de besoin n'est pas une erreur : ses commits restent, ils racontent les décisions de l'opérateur. Il pousse avec `--force-with-lease`, jamais sur `main`, et dit à l'opérateur quels commits il a écartés ou fusionnés.
- **Éprouver un garde-fou.** Un garde-fou (lint, hook, CI) n'est déclaré en place qu'après l'avoir vu bloquer en conditions réelles.

## Réponses à l'opérateur

- **Courtes.** L'essentiel d'abord, le détail seulement sur demande.
- **Les questions à la fin, avec l'outil de questions à choix.** Chaque question rappelle son contexte en une phrase, chaque option dit ce qui change concrètement. Une option « Explique-moi d'abord » est toujours présente.
- **Une recommandation seulement pour un petit choix réversible**, en première option : sans réponse, l'agent l'applique. Une décision importante (difficile à défaire, ou qui touche au métier) n'en a aucune : l'opérateur doit lire pour choisir.
- **Les liens des fichiers après chaque push.** La réponse se termine, après une ligne vide, par le lien GitHub de chaque fichier modifié sur la branche par rapport à `main`, un par ligne. Le libellé du lien est le chemin du fichier, pas l'adresse : `[CAP.md](https://github.com/MrCawet/LuxTonCrane/blob/<branche>/CAP.md)`. Au-delà de 3 fichiers, seulement le lien de la racine du dépôt sur la branche : `[LuxTonCrane](https://github.com/MrCawet/LuxTonCrane/tree/<branche>)`.
- **Une sous-question à la fois.** Si une réponse en soulève une autre, elle est traitée seule, avant la question suivante.

## Règles de code

1. Écrire du code facile à lire et explicite, plutôt que compact.
2. La `logique` du métier ou d'un comportement s'exprime en français. Le code exprime la même chose, c'est en cela qu'il est explicite. Une traduction dans une autre langue s'éloignerait toujours du sens premier : les identifiants de cette partie du code ne sont donc pas traduits (les accents sont toujours préférables, sauf quand ils ne sont pas possibles).
3. Les `détails d'implémentation` sont toute la partie du code nécessaire au fonctionnement de la `logique`. Cette partie peut varier à l'infini, tant qu'elle n'influe pas sur la `logique`. Les patterns techniques étant toujours en anglais, les identifiants de cette partie du code sont en anglais.
4. N'optimiser que si un problème de performance est signalé explicitement.
5. Rendre le code explicite plutôt que le commenter. Ne commenter que ce que le code ne peut pas dire : pourquoi il existe, ou pourquoi il est écrit d'une manière qui peut sembler étrange.
6. La documentation d'une variable ou d'un attribut n'indique que ce que son nom ne dit pas. Si le nom suffit, ne pas en écrire.
7. Aucune dépendance sans justification : la bibliothèque standard d'abord.
8. Tout ce qui sert à une même idée est regroupé au même endroit, effets compris ; le nom annonce ces effets.

## Lisibilité

L'opérateur est dyslexique. Ces règles valent pour les identifiants, les noms de fichiers et la documentation.

1. Des mots entiers, toujours séparés, sans abréviation : `timestamp_ms` plutôt que `ts_ms`, `historique` plutôt que `hist`. Deux exceptions : une unité s'écrit avec son symbole (`ms`, `s`, `lx`) ; un nom manipulé souvent peut être court, si sa documentation dit le reste.
2. Pas de noms qui se ressemblent dans un même voisinage, en particulier ceux qui ne diffèrent que d'une lettre (`mesure` et `mesures`) : préférer `mesure` et `historique`. Cette règle ne doit jamais nuire à la clarté : quand les noms les plus clairs se ressemblent parce qu'ils disent des choses symétriques, on les garde (`début` et `fin`).
3. Une idée par ligne : pas d'enchaînement de méthodes sur une seule ligne, des variables intermédiaires nommées plutôt qu'une expression imbriquée.

## Nommer

Avant d'écrire un nom, vérifier chaque point.

1. **Le terme du glossaire.** Un terme du métier s'écrit comme dans `GLOSSAIRE.md`. S'il n'y est pas, le proposer à l'opérateur et l'y ajouter (le fichier se crée avec le premier terme).
2. **Le terme exact.** Le terme établi quand il existe, et le plus précis : « numéro » et non « nombre » pour un identifiant.
3. **Ce que la chose exprime, pas comment elle est stockée.** Une variable qui porte la logique se nomme en français d'après son rôle, même si elle contient une liste ou une table. Un attribut qui ne sert qu'à des méthodes qui portent la logique est un détail technique, nommé d'après sa forme (`as_list`, `as_map`).
4. **Une méthode porte le nom de ce qu'elle renvoie.** Pas de `get_` (`moyenne_de(mesures)`, et non `get_moyenne`). Pas de verbe qui suggère une action qui n'a pas lieu (`calculer` pour une simple lecture).
5. **Pas de noms qui se ressemblent** (règle de lisibilité 2), y compris les mêmes mots dans l'ordre inverse (`mesure_par_heure` et `heure_par_mesure`) et le singulier à côté du pluriel (`mesure` et `mesures`).
6. **Pas de variable intermédiaire seulement pour la renommer** : construire directement sous le nom final.

## Stratégie de test : TDD, école de Détroit

Approche TDD classique (école de Détroit). Pas de framework de mocking de l'école de Londres : aucune vérification d'appels internes (`verify()`, `toHaveBeenCalledWith`…).

Règles non négociables :

- **Tester les comportements, pas l'implémentation.** Les tests s'écrivent du point de vue des contrats publics et des cas d'usage métier (de l'intérieur vers l'extérieur). Ils n'utilisent que l'API publique.
- **Le test d'abord.** Il échoue pour la bonne raison avant d'écrire le code. Un test existant ne se modifie que si le besoin change, jamais pour faire passer le code.
- **Le cycle se lit dans l'historique.** Un commit par étape :
  1. `test:` le test, qui échoue (rouge) ;
  2. `feat:` ou `fix:` le code qui le fait passer (vert) ;
  3. `refactor:` si besoin, le code amélioré, tests toujours verts.
- **Pas d'isolation au niveau des classes ou des fonctions.** Ne pas mocker ni stubber les classes internes, les fonctions ou les entités du domaine. Quand plusieurs classes collaborent pour réaliser un comportement, elles exécutent leur vrai code en mémoire.
- **Les doublures ne servent qu'aux frontières.** Mocks, stubs ou fakes en mémoire sont réservés aux frontières lentes, non déterministes ou externes au processus : capteur de lumière, base de données, réseau, API tierces, système de fichiers, horloge système.
- **Immunité au refactoring.** Les tests valident l'état final ou la valeur de retour, jamais la structure interne. Refactoriser un module ne doit jamais casser un test si le comportement métier reste identique.

## Architecture

L'arborescence crie le métier (mesure, rapport…), pas les outils (screaming architecture). Le métier ne dépend d'aucun détail technique : les frontières sont des adaptateurs branchés autour (clean architecture), et ce sont exactement elles qu'on double dans les tests.
