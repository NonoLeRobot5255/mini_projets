fn main() {
    let mut c: i32 = 1;
    let mut trouver: bool = false;
    loop {
        let mut b: i32 = 1;
        let c2: i32 = c * c;
        let mut b2: i32 = b * b;

        while b2 < c2 / 2 {
            b2 = b * b;
            let a2: i32 = c2 - b2;
            let a: f32 = (a2 as f32).sqrt();

            if a.round() == a {
                if a as i32 + b + c == 1000 {
                    println!("le resultat est {}", a as i32 * b * c);
                    trouver = true;
                    break;
                }
            }
            b += 1
        }
        c += 1;
        if trouver {
            break;
        }
    }
}
