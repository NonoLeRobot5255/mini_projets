fn main() {
    let mut nombre: i64 = 600851475143;
    let racine: i64 = nombre.isqrt();

    let mut i: i64 = 2;
    let mut prime: Vec<i64> = Vec::new();

    while nombre > racine {
        let mut premier: bool = true;
        for item in prime.iter() {
            let division: f64 = i as f64 / *item as f64;
            if division == division.round() {
                premier = false;
            }
        }
        let mut reste: f64 = nombre as f64 / i as f64;
        while premier && reste == reste.round() {
            prime.push(i);
            nombre = reste as i64;
            reste = nombre as f64 / i as f64;
        }
        i += 1;
    }
    prime.push(nombre);
    println!("{:?}", prime);
}
