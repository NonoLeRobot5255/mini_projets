fn main() {
    let mut carre: i64 = 0;
    let mut somme: i64 = 0;
    for n in 1..=100 {
        somme += n;
        carre += n * n;
    }
    println!("Résultat = {}", somme * somme - carre);
}
