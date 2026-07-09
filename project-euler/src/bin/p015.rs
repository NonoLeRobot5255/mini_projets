fn fini(deplacement: Vec<char>) -> bool {
    let check: i32 = deplacement.len() as i32 / 2;
    for i in 1..=check {
        if deplacement[(deplacement.len() as i32 - i) as usize] != 'D' {
            return false;
        }
    }
    true
}

fn main() {
    let mut mouvement: Vec<char> = vec![
        'D', 'D', 'D', 'D', 'D', 'D', 'D', 'D', 'D', 'D', 'B', 'B', 'B', 'B', 'B', 'B', 'B', 'B',
        'B', 'B',
    ];

    let mut compteur = 1;
    let mut fin: bool = fini(mouvement.clone());
    let taille: i32 = mouvement.clone().len() as i32 - 1;
    let mut _bonne_place = 0;
    while !fin {
        println!("{:?}", mouvement);
        let mut nd = 0;
        while mouvement[(taille - nd) as usize] == 'D' {
            nd += 1;
        }

        let mut ed: i32 = 1;
        while mouvement[(taille - nd - ed) as usize] != 'D' {
            ed += 1;
        }
        let edd = taille - ed - nd;

        mouvement[(edd + 1) as usize] = 'D';
        mouvement[edd as usize] = 'B';
        println!("edd = {edd}, nd = {nd}");

        if nd != 0 && edd != taille - 1 - nd {
            let mut i = 0;
            while i < nd {
                if mouvement[(edd + 2 + i) as usize] == 'D' {
                    i += 1;
                    continue;
                }
                mouvement[(edd + 2 + i) as usize] = 'D';
                mouvement[(taille - i) as usize] = 'B';
                i += 1;
            }
        }

        compteur += 1;
        println!("compteur = {compteur}");
        fin = fini(mouvement.clone());
    }
    println!("{:?}", mouvement);
    println!("le nombre de chemin est {compteur}");
}
