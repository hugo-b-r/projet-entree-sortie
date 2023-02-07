use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problème rencontré lors de l'interprétation des arguments :
            {}", err);
        process::exit(1);
    });

    println!("On recherche : {}", config.recherche);
    println!("Dans le fichier : {}", config.nom_fichier);

    if let Err(e) = run(config) {
        println!("Erreur applicative : {}", e);

        process::exit(1);
    }
}


fn run(config:Config) -> Result <(), Box<dyn Error>> {
    let contenu = fs::read_to_string(config.nom_fichier)?;

            
    println!("Dans le texte: \n{}", contenu);

    Ok(())
}

struct Config {
    recherche:String,
    nom_fichier: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("il n'y a pas assez d'arguments");
        }
        let recherche = args[1].clone();
        let nom_fichier = args[2].clone();

        Ok(Config { recherche, nom_fichier })
    }
}
