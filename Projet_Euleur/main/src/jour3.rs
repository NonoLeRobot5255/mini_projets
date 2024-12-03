pub fn probleme3()->u64{
    let mut nb : u64 = 600851475143;
    let mut i  = 2;
    let mut premier= vec![2];
    while  i*i <= nb{
        let mut estpremier:bool = true;
        for &p in &premier{
            if i%p ==0 {
                estpremier=false;
                break;
            }
        }
        if estpremier{
            premier.push(i);
            while nb % i == 0{
                nb = nb/i;
            }
        }
        i = i+1;
    }
    return nb;
}