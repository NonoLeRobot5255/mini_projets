pub fn probleme1() -> u32 {
    let mut i: u32 = 1;
    let mut somme: u32 = 0;
    while i < 1000 {
        if i%3 ==0 || i % 5 ==0 {
            somme += i;
        }
        i += 1;
    }
    return somme;
}
