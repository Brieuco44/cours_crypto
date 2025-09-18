// Fonction principale du programme
fn main() {
    // Définition de deux matrice 2x2
    let matrix1 = vec![vec![1.0, 2.0], vec![4.0, 5.0]];
    let matrix2 = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    // Définition d'un vecteur de taille 2
    let vector = vec![7.0, 8.0];

    // Multiplication de la matrice par le vecteur
    let result_multiply = multiply_matrix_vector(&matrix1, &vector);
    // Affichage du résultat
    println!(
        "Multiplication de la matrice {:?} par un vecteur {:?} :",
        matrix1, vector
    );
    println!("Résultat : {:?}", result_multiply);
    println!("------------------------------");
    // Addition de deux matrices
    println!(
        "Addition de la matrice {:?} et de la matrice {:?} :",
        matrix1, matrix2
    );
    let result_add = add_matrices(&[matrix1.clone(), matrix2.clone()]);
    println!("Résultat : {:?}", result_add);
    println!("------------------------------");
    // Calcul du déterminant d'une matrice 2x2
    let det = determinant_2x2(&matrix1);
    println!("Déterminant de la matrice {:?} : {:?}", matrix1, det);
}

// Multiplie une matrice par un vecteur
// matrix : référence vers la matrice (Vec<Vec<f64>>)
// vector : référence vers le vecteur (Vec<f64>)
// Retourne le vecteur résultant de la multiplication
fn multiply_matrix_vector(matrix: &Vec<Vec<f64>>, vector: &Vec<f64>) -> Vec<f64> {
    let rows = matrix.len(); // Nombre de lignes de la matrice
    let cols = if rows > 0 { matrix[0].len() } else { 0 }; // Nombre de colonnes
    // Vérification que la taille du vecteur correspond au nombre de colonnes
    assert_eq!(
        cols,
        vector.len(),
        "Le nombre de colonnes doit correspondre à la taille du vecteur."
    );

    let mut result = vec![0.0; rows]; // Initialisation du vecteur résultat
    for i in 0..rows {
        for j in 0..cols {
            // Calcul de la valeur de la ligne i
            result[i] += matrix[i][j] * vector[j];
        }
    }
    result // Retourne le vecteur résultat
}

// Additionne deux matrices ou plus de même taille
fn add_matrices(matrices: &[Vec<Vec<f64>>]) -> Vec<Vec<f64>> {
    assert!(
        !matrices.is_empty(),
        "La liste des matrices ne doit pas être vide."
    );
    let rows = matrices[0].len();
    let cols = if rows > 0 { matrices[0][0].len() } else { 0 };

    // Vérifie que toutes les matrices ont la même taille
    for m in matrices {
        assert_eq!(
            m.len(),
            rows,
            "Toutes les matrices doivent avoir le même nombre de lignes."
        );
        for row in m {
            assert_eq!(
                row.len(),
                cols,
                "Toutes les matrices doivent avoir le même nombre de colonnes."
            );
        }
    }
    //  Initialisation de la matrice résultat avec des zéros
    let mut result = vec![vec![0.0; cols]; rows];

    // Additionne les matrices
    for m in matrices {
        for i in 0..rows {
            for j in 0..cols {
                result[i][j] += m[i][j];
            }
        }
    }
    result
}

// Calcule le déterminant d'une matrice 2x2
fn determinant_2x2(matrix: &Vec<Vec<f64>>) -> f64 {
    assert_eq!(matrix.len(), 2, "La matrice doit avoir 2 lignes.");
    assert_eq!(matrix[0].len(), 2, "La matrice doit avoir 2 colonnes.");
    assert_eq!(matrix[1].len(), 2, "La matrice doit avoir 2 colonnes.");
    // Calcul du déterminant
    matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_matrix_vector() {
        let matrix = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let vector = vec![5.0, 6.0];
        let result = multiply_matrix_vector(&matrix, &vector);
        assert_eq!(result, vec![17.0, 39.0]);
    }

    #[test]
    fn test_add_matrices() {
        let m1 = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let m2 = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
        let result = add_matrices(&[m1.clone(), m2.clone()]);
        assert_eq!(result, vec![vec![6.0, 8.0], vec![10.0, 12.0]]);
    }

    #[test]
    fn test_determinant_2x2() {
        let matrix = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let det = determinant_2x2(&matrix);
        assert_eq!(det, -2.0);
    }
}
