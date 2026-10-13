fn main() {
    let mut resultat: Vec<i16> = vec![0; 100000];
    resultat[0] = 1;
    let mut somme: i64 = 0;

    for i in 2..=100 {
        for nombre in resultat.iter_mut() {
            *nombre = *nombre * i;
        }
        for i in 0..=resultat.len() - 1 {
            while resultat[i] > 9 {
                resultat[i] -= 10;
                resultat[i + 1] += 1;
            }
        }
    }

    for nombre in resultat.iter() {
        somme += *nombre as i64;
    }

    println!("le resultat est {somme}");
}
