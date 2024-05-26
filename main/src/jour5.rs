pub fn probleme5() -> u64{
    let mut i:u32 = 2;
    loop{
        let mut goodgood:bool = true;
        for j in 5..20{
            if i%j != 0{
                goodgood =false;
                break;
            }
        } 
        if goodgood{
            return i as u64;
        }
        i=i+2;
    }
}