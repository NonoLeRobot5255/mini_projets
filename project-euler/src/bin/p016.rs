fn main() {
    let mut res: Vec<i16> = vec![0; 1001];
    res[1] = 1;

    for _i in 1..=1000 {
        for nombre in res.iter_mut() {
            *nombre = *nombre * 2;
        }
        for i in 0..=res.len() - 1 {
            while res[i] > 9 {
                res[i] -= 10;
                res[i + 1] += 1;
            }
        }
    }
    let mut somme: i64 = 0;
    for nombre in res.iter() {
        somme += *nombre as i64;
    }
    println!("le resultat est {somme}");
}
