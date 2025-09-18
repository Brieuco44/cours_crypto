fn main() {
    let message = "Bonjour, monde!";
    let decalage = 3;
    let chiffre = cesar(message, decalage, true);
    println!("Message original : {}", message);
    println!("Message chiffré : {}", chiffre);
}

// Chiffre ou déchiffre un texte avec l'algorithme de César.
//
// # Paramètres
// * `text` - Le texte à traiter.
// * `shift` - Le nombre de décalages (clé du chiffrement).
// * `cypher` - Si vrai, chiffre le texte ; si faux, le déchiffre.
//
// # Retour
// Une chaîne de caractères transformée selon l'algorithme de César.
fn cesar(text: &str, key: u8, cypher: bool) -> String {
    text.chars()
        .map(|c| {
            // Vérifie si le caractère est une lettre de l'alphabet
            if c.is_ascii_alphabetic() {
                // Détermine la base selon la casse (minuscule ou majuscule)
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                // Applique le décalage pour chiffrer ou déchiffrer
                let retour = if cypher {
                    ((c as u8 - base + key) % 26) + base
                } else {
                    ((c as u8 - base + 26 - key) % 26) + base
                };
                retour as char
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
    fn test_cesar_basic_cypher() {
        assert_eq!(cesar("abc", 1, true), "bcd");
        assert_eq!(cesar("xyz", 2, true), "zab");
    }

    #[test]
    fn test_cesar_basic_decipher() {
        assert_eq!(cesar("bcd", 1, false), "abc");
        assert_eq!(cesar("zab", 2, false), "xyz");
    }

    #[test]
    fn test_cesar_with_uppercase() {
        assert_eq!(cesar("ABC", 3, true), "DEF");
        assert_eq!(cesar("XYZ", 1, true), "YZA");
    }

    #[test]
    fn test_cesar_with_uppercase_decipher() {
        assert_eq!(cesar("DEF", 3, false), "ABC");
        assert_eq!(cesar("YZA", 1, false), "XYZ");
    }

    #[test]
    fn test_cesar_non_alpha() {
        assert_eq!(cesar("123!", 5, true), "123!");
        assert_eq!(cesar("Hello, World!", 13, true), "Uryyb, Jbeyq!");
    }
}
