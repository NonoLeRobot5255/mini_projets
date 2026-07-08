fn palyndrome(nombre: i64) -> bool {
    let normal: i64 = nombre;
    let strin = normal.to_string();
    let rev_s = strin.chars().rev().collect::<String>();
    let rev: i64 = rev_s.parse().unwrap();

    rev == normal
}

fn main() {
    let mut nombre: i64 = 999 * 999;
    let mut trouve = false;
    loop {
        if palyndrome(nombre) {
            let mut i = 100;
            while i < 1000 {
                let division: f64 = nombre as f64 / i as f64;
                if division == division.round() && nombre / i < 1000 {
                    println!("le nombre cherche est {}", nombre);
                    trouve = true;
                    break;
                }
                i += 1;
            }
        }
        if trouve {
            break;
        }
        nombre -= 1;
    }
}
