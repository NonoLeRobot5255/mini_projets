fn main() {
    let mut res = 0;
    let mut i: i128 = 0;
    while i <= 200000 {
        let carre: i128 = i * i;
        if carre % 2 != 0 {
            res += carre;
        }
        i += 1
    }
    println!("{}", res);
}
