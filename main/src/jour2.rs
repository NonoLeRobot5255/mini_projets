pub fn probleme2()-> u64 {
    let mut x1: u32 = 0;
    let mut x2: u32 = 1;
    let mut x3: u32 = x1 + x2;
    let mut somme : u64 = 0;
    while x3 < 4000000 {
        if x3%2==0 {
            somme = somme + x3 as u64;
        }
        x1 = x2;
        x2 = x3;
        x3 = x2 + x1;
    }
    return somme;
}