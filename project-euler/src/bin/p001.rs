fn main() {
    let mut i = 0;
    let mut res = 0;
    loop {
        i += 1;
        if i == 1000 {
            break;
        }
        if i % 3 == 0 || i % 5 == 0 {
            res += i;
        }
    }
    println!("{}", res)
}
