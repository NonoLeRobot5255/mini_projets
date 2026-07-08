fn main() {
    let mut i: i64 = 2520;
    let diviseur: Vec<i32> = vec![11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
    loop {
        let mut trouve: bool = true;
        for item in diviseur.iter() {
            let division: f64 = i as f64 / *item as f64;
            if division != division.round() {
                trouve = false;
                break;
            }
        }
        if trouve {
            println!("nombre trouve : {i}");
            break;
        }
        i += 2;
    }
}
