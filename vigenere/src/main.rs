fn main() {
    // Message à chiffrer
    let message = "Bonjour, monde!";
    // Clé de chiffrement (doit être composée de lettres)
    let clef = "CLEF";
    // Chiffrement du message avec la clé
    let chiffre = vigenere(message, clef);
    // Affichage du message original et du message chiffré
    println!("Message original : {}", message);
    println!("Message chiffré : {}", chiffre);
}

/// Chiffre un texte avec le chiffre de Vigenère.
///
/// # Arguments
/// * `text` - Le texte à chiffrer.
/// * `key` - La clé de chiffrement (lettres uniquement).
///
/// # Retour
/// Le texte chiffré.
fn vigenere(text: &str, key: &str) -> String {
    // Met la clé en majuscules pour simplifier le calcul
    let key = key.to_ascii_uppercase();
    // Crée un itérateur cyclique sur la clé pour répéter la clé si besoin
    let mut key_iter = key.chars().cycle();
    text.chars()
        .map(|c| {
            // Vérifie si le caractère est une lettre
            if c.is_ascii_alphabetic() {
                // Détermine la base selon la casse (minuscule ou majuscule)
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                // Calcule le décalage à partir de la lettre de la clé
                let k = key_iter.next().unwrap() as u8 - b'A';
                // Applique le chiffrement Vigenère
                (((c as u8 - base + k) % 26) + base) as char
            } else {
                // Les caractères non alphabétiques restent inchangés
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vigenere() {
        let message = "Bonjour, monde!";
        let clef = "CLEF";
        let chiffre = vigenere(message, clef);
        assert_eq!(chiffre, "Dzroqfv, rqyhj!");
    }
}
