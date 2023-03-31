
fn euclidean_distance(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    let distance_squared = a
        .iter()
        .zip(b.iter())
        .map(|(&a_i, &b_i)| (a_i - b_i).powi(2))
        .sum::<f64>();
    distance_squared.sqrt()
}

fn knn(x_train: &Vec<Vec<f64>>, y_train: &Vec<i32>, x_test: &Vec<f64>, k: usize) -> i32 {
    let mut distances: Vec<(f64, i32)> = Vec::new();

    for (i, x) in x_train.iter().enumerate() {
        let distance = euclidean_distance(x, x_test);
        distances.push((distance, y_train[i]));
    }

    distances.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut counts = [0; 2];

    for &(_, y) in distances.iter().take(k) {
        counts[y as usize] += 1;
    }

    if counts[0] >= counts[1] {
        0
    } else {
        1
    }
}


/// Problema simple de clasificación con un data set de 2 caracteristicas y dos clases o etiquetas
fn main() {
    // conjunto de datos de entrenamiento x_train con dos caracsteristicas de pruebas (data set de juguete)
    let x_train = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];

    // salidas y_train
    let y_train = vec![0, 1, 1, 0];
    // nueva muestra
    let x_test = vec![0.5, 0.5];
    // tratamos de clasificarla o predecir a que clase pertenece
    let y_pred = knn(&x_train, &y_train, &x_test, 3);

    println!("La etiqueta predicha es {}", y_pred);
}
