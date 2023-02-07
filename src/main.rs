use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = interpreter_config(&args);

    println!("On recherche : {}", config.recherche);
    println!("Dans le fichier : {}", config.nom_fichier);

    let contenu = fs::read_to_string(config.nom_fichier)
        .expect("Quelquechose ne s'est pas passé comme prévu lors
            de la lecture du fchier");
    println!("Dans le texte: \n{}", contenu);
}

struct Config {
    recherche:String,
    nom_fichier: String,
}

fn interpreter_config(args: &[String]) -> Config {
    let recherche = args[1].clone();
    let nom_fichier = args[2].clone();

    Config { recherche, nom_fichier }
}