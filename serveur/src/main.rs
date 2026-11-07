#![forbid(unsafe_code)]

//! Lance le serveur HTTP. Sa configuration vient de l'environnement, pour que le jeton reste hors du dépôt :
//! `LUXTONCRANE_JETON` (obligatoire), `LUXTONCRANE_BASE` (le fichier SQLite) et `LUXTONCRANE_ADRESSE`.

use std::path::Path;
use std::process::ExitCode;

use journal_sqlite::SqliteJournal;
use serveur::{Méthode, Requête, Serveur};
use tiny_http::{Header, Method, Response, Server};

const DEFAULT_DATABASE: &str = "luxtoncrane.sqlite";
// Seul nginx, sur la même machine, doit pouvoir le joindre.
const DEFAULT_ADDRESS: &str = "127.0.0.1:8090";

fn méthode_de(method: &Method) -> Méthode {
    match method {
        Method::Get => Méthode::Get,
        Method::Post => Méthode::Post,
        _ => Méthode::Autre,
    }
}

fn header_value<'a>(request: &'a tiny_http::Request, name: &'static str) -> Option<&'a str> {
    let header = request
        .headers()
        .iter()
        .find(|header| header.field.equiv(name))?;
    Some(header.value.as_str())
}

fn serve(server: &Server, serveur: &mut Serveur) {
    for mut request in server.incoming_requests() {
        let mut corps = String::new();
        let lecture = request.as_reader().read_to_string(&mut corps);
        let réponse = match lecture {
            Ok(_) => {
                let méthode = méthode_de(request.method());
                let chemin = request.url().to_string();
                let autorisation = header_value(&request, "Authorization").map(str::to_string);
                serveur.répondre(Requête {
                    méthode,
                    chemin: &chemin,
                    autorisation: autorisation.as_deref(),
                    corps: &corps,
                })
            }
            Err(_) => serveur::Réponse {
                statut: 400,
                corps: String::new(),
            },
        };
        let mut response = Response::from_string(réponse.corps).with_status_code(réponse.statut);
        if let Ok(content_type) = Header::from_bytes("Content-Type", "application/json") {
            response = response.with_header(content_type);
        }
        if let Err(error) = request.respond(response) {
            eprintln!("réponse impossible à envoyer : {error}");
        }
    }
}

fn main() -> ExitCode {
    let Ok(jeton) = std::env::var("LUXTONCRANE_JETON") else {
        eprintln!("LUXTONCRANE_JETON manque : le serveur refuse de démarrer sans jeton.");
        return ExitCode::FAILURE;
    };
    let database = std::env::var("LUXTONCRANE_BASE").unwrap_or(DEFAULT_DATABASE.to_string());
    let address = std::env::var("LUXTONCRANE_ADRESSE").unwrap_or(DEFAULT_ADDRESS.to_string());

    let journal = match SqliteJournal::open(Path::new(&database)) {
        Ok(journal) => journal,
        Err(error) => {
            eprintln!("ouverture de {database} impossible : {error}");
            return ExitCode::FAILURE;
        }
    };
    let server = match Server::http(&address) {
        Ok(server) => server,
        Err(error) => {
            eprintln!("écoute sur {address} impossible : {error}");
            return ExitCode::FAILURE;
        }
    };
    let mut serveur = Serveur::new(jeton, journal);
    println!("LuxTonCrane écoute sur {address}, base {database}");
    serve(&server, &mut serveur);
    ExitCode::SUCCESS
}
