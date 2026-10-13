fn main() {
    let mut results: Vec<i32> = vec![0; 10001];
    let mut somme_final = 0;
    for nombre in 1..=10000 {
        let n_boucles: f32 = (nombre as f32) / 2f32;
        let n_boucles_arrondi: i32 = n_boucles.floor() as i32;
        let mut somme = 1;
        for i in 2..=n_boucles_arrondi {
            let division = (nombre as f64) / (i as f64);
            if division == division.round() {
                somme += i;
            }
        }
        if somme < nombre && somme > 1 && results[somme as usize] == nombre {
            somme_final += somme;
            somme_final += nombre;
        }
        results[nombre as usize] = somme;
    }
    println!("la somme des nombres amicaux est {somme_final}");
}
