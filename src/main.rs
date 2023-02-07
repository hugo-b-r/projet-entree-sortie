use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let recherche = &args[1];
    let nom_fichier = &args[2];

    println!("On recherche : {}", recherche);
    println!("Dans le fichier : {}", nom_fichier);

    let contenu = fs::read_to_string(nom_fichier)
        .expect("Quelquechose ne s'est pas passé comme prévu lors
            de la lecture du fchier");
    println!("Dans le texte: \n{}", contenu);
}
