---
paths:
  - "**/*.rs"
  - "**/Cargo.toml"
---

# Règles Rust

Complètent [CLAUDE.md](../../CLAUDE.md). Chacune répond à une dérive connue d'un agent qui cherche à faire compiler à tout prix.

## Ne pas contourner le compilateur

- Pas de `unsafe` : le cœur porte `#![forbid(unsafe_code)]`.
- Pas de `unwrap()` ni d'`expect()` dans le code de la crate (`src/`) : traiter le `Result` ou l'`Option`. Ils sont permis dans `tests/`, doublures comprises.
- Pas de `#[allow(...)]` pour faire taire un avertissement : corriger la cause, ou demander à l'opérateur.
- Pas de `.clone()`, `Rc<RefCell<…>>` ou `Arc<Mutex<…>>` ajouté seulement pour satisfaire le vérificateur d'emprunts : revoir à qui appartient la donnée.

## Rester explicite

- Pas de macro ni de générique sans besoin réel.
- Une frontière est un trait, utilisé par défaut en `Box<dyn Trait>`.
- Le cœur est synchrone : pas d'`async` ni de `tokio`.

## Ranger le code

- Dans un type, les méthodes privées d'abord, puis les publiques.
- Un bloc `impl` par groupe de méthodes, chacun avec une ligne de documentation qui dit ce que le groupe fait. Pas de bandeau en commentaire (`// ----- Privé -----`).

## Tests

- Les tests sont dans `tests/` et n'utilisent que l'API publique de la crate : pas de `#[cfg(test)] mod tests` qui accède à l'interne.
- Pas de `mockall` ni d'autre crate de mocking : les doublures des frontières s'écrivent à la main.

## Accents

Les identifiants du métier gardent leurs accents (types, fonctions, variables). On ne les retire que là où Rust ou l'autre côté les refusent :

- noms de crate ;
- noms de fichiers et de modules (`mod état;` est refusé sans `#[path]`) ;
- ce qui traverse une frontière FFI (UniFFI, JNI, `#[no_mangle]`).

## Dépendances et versions

- La bibliothèque standard d'abord : une crate s'ajoute avec une justification.
- La chaîne est fixée par `rust-toolchain.toml`, `Cargo.lock` est versionné.
- Avant d'utiliser une crate, lire la documentation de la version présente dans `Cargo.lock`, pas celle dont on se souvient.

## Avant chaque livraison

Dire explicitement ce qui n'a pas pu être vérifié (cible Android…).
