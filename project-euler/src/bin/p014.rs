use std::cmp::max;

fn main() {
    let mut res: Vec<i32> = vec![0; 1000001];
    res[1] = 1;

    let mut index = 2;
    let mut maximum: i32 = 0;
    let mut index_max = 0;

    loop {
        if index > 1000000 {
            break;
        }
        let mut compteur = 1;
        let mut nombre = index;
        while nombre != 1 {
            if nombre % 2 == 0 {
                nombre = nombre / 2;
            } else {
                nombre = nombre * 3 + 1;
            }
            if nombre < index {
                compteur += res[nombre];
                res[index] = compteur;
                if compteur > maximum {
                    maximum = max(maximum, compteur);
                    index_max = index;
                }
                break;
            }
            compteur += 1;
        }
        index += 1;
    }

    let mut somme: i32 = 0;
    for nombre in res.iter() {
        somme += nombre;
    }
    println!("maximum = {maximum} en position {index_max}");
    println!("la moyenne d'étape est {}", somme / 1000000);
}
