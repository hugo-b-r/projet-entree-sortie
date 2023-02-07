use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args);

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
