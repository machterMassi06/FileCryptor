use rand::Rng;
use std::{fs::File, io::Write};

#[derive(Debug)]
pub struct KeyError;

/// Génère une clé aléatoire de 16 octets visible en une chaîne lisible de caractères ASCII
pub fn generate_key() -> String {
    let mut rng = rand::thread_rng();
    let key: Vec<u8> = (0..16).map(|_| rng.gen_range(32..=126)) // Caractères ASCII imprimables (de 32 à 126)
        .collect();
    String::from_utf8(key).expect("La clé générée contient des caractères invalides!")
}

/// Sauvegarde une clé lisible dans un fichier
pub fn save_key_to_file(key: &str, file_path: &str) {
    let mut f = File::create(file_path).expect("Erreur lors de la création du fichier !");
    f.write_all(key.as_bytes()).expect("Erreur lors de l'écriture dans le fichier !");
    println!("La clé a été sauvegardée dans le fichier {}", file_path);
}

/// Vérifie si une clé fait bien 16 octets
pub fn check_key_is_16_o(key: &[u8]) -> Result<(), KeyError> {
    if key.len() == 16 {
        Ok(())
    } else {
        Err(KeyError)
    }
}
