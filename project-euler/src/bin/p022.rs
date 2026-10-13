use std::fs;

fn main() {
    let contenu = fs::read_to_string("listes_noms.txt").expect("impossible de lire le fichier");
    let mut contenu_propre: Vec<String> = contenu.split(',').map(|p| p.replace('"', "")).collect();
    contenu_propre.sort();

    let mut compteur: i32 = 1;
    let mut somme: i64 = 0;
    for nom in contenu_propre {
        let mut total = 0;
        for char in nom.chars() {
            total += char as i32 - 'A' as i32 + 1
        }

        somme += (total * compteur) as i64;
        compteur += 1;
    }
    println!("le résultat est {somme}");
}
