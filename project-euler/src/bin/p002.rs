fn main() {
    let mut a: i32 = 1;
    let mut b: i32 = 2;

    let mut res: i64 = 2;
    loop {
        let c: i32 = a + b;
        if c > 4000000 {
            break;
        }
        if c % 2 == 0 {
            res += c as i64;
        }
        a = b;
        b = c;
    }
    println!("nombre trouvé = {}", res);
}
