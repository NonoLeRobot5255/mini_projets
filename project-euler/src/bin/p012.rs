fn main() {
    let mut natural: i64 = 1;
    let mut somme: i64 = 0;
    loop {
        let mut diviseur: i64 = 0;
        let mut compteur = 1;
        while compteur < somme.isqrt() {
            let temp: f64 = somme as f64 / compteur as f64;
            if temp == temp.round() {
                diviseur += 2;
            }
            compteur += 1;
        }
        if diviseur > 500 {
            println!("nombre trouvé : {somme}");
            break;
        }

        somme += natural;
        natural += 1;
    }
}
