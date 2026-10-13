fn main() {
    let mut annee: i32 = 1901;
    let mois: Vec<i16> = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut nombredimanche: i16 = 0;
    let mut jour = 2;

    while annee <= 2000 {
        for n_jour in &mois {
            if jour == 0 {
                nombredimanche += 1;
            }
            if *n_jour == 28 && annee % 4 == 0 && annee % 100 != 0 {
                jour += 1;
            }
            if annee == 2000 && *n_jour == 28 {
                jour += 1;
            }
            jour += n_jour;
            jour = jour % 7;
        }
        annee += 1;
    }
    println!(
        "le nombre de dimanche sur un premier du mois est {}",
        nombredimanche
    );
}
