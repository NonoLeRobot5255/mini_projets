fn main() {
    let mut premier: Vec<bool> = vec![true; 2000000];
    let taille: i64 = premier.len() as i64 - 1;

    let mut regarde: i64 = 2;

    while regarde <= taille / 2 {
        let mut i: i64 = 2;
        let mut index = i * regarde;

        while index <= taille {
            premier[index as usize] = false;
            i += 1;
            index = i * regarde;
        }
        if regarde == 2 {
            regarde += 1;
        } else {
            regarde += 2;
            while premier[regarde as usize] != true {
                regarde += 2
            }
        }
    }

    let mut index: i128 = 3;
    let mut somme: i128 = 2;
    while index < premier.len() as i128 - 1 {
        if premier[index as usize] == true {
            somme += index;
        }
        index += 2;
    }
    println!("la somme est {somme}");
}
