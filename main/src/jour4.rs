pub fn probleme4() -> u64{
    let mut x1:u32 = 100;
    let mut x2:u32 = 100;
    let mut max:u32 = 0;
    while x1<1000 && x2<1000{
        let mut palyndrome:bool = true;
        let multiplication: u32 = x1 * x2;
        let test = multiplication.to_string();
        for i in 0..test.len()/2 {
            if test.as_bytes()[i] != test.as_bytes()[test.len() - 1 - i] {
                palyndrome = false;
                break;
            }
        }
    
        if palyndrome && multiplication>max{
            max = multiplication;
        }
        if x1 == 999 {
            x1 = 100;
            x2+=1;
        }
        else {
        x1=x1 + 1;
        }
    }
    return max as u64;
}

