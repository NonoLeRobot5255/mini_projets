fn main() {
    let chiffre = vec![0, 3, 3, 5, 4, 4, 3, 5, 5, 4, 3, 6, 6, 8, 8, 7, 7, 9, 8, 8];
    let dizaines = vec![0, 0, 6, 6, 5, 5, 5, 7, 6, 6];
    let mut compteur: i32 = 0;
    for i in 1..1000 {
        let nombre: Vec<u32> = i
            .to_string()
            .as_str()
            .chars()
            .rev()
            .map(|a| a.to_digit(10).unwrap())
            .collect();

        if i > 9 && i < 20 {
            compteur += chiffre[i as usize];
        } else {
            if i % 100 == 0 {
                compteur += 7;
                compteur += chiffre[i / 100 as usize];
                continue;
            }
            let mut j = 0;
            for chi in nombre.clone() {
                if j == 0 {
                    if i > 10 && nombre[1] == 1 {
                        let dizaine = 10 + chi;
                        compteur += chiffre[dizaine as usize];
                    } else {
                        compteur += chiffre[chi as usize];
                    }
                } else if j == 1 {
                    compteur += dizaines[chi as usize];
                } else {
                    compteur += chiffre[chi as usize];
                    compteur += 10;
                }
                j += 1;
            }
        }
    }
    compteur += 11;
    println!("le res est {compteur}");
}
