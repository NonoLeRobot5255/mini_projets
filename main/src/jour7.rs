pub fn probleme7() ->u64{
    let mut nb:u64=0;
    let mut premier = vec![2];
    let mut i =3;
    while nb<10001{
        let mut estpremier:bool = true;
        for &p in &premier{
            if i%p ==0 {
                estpremier=false;
                break;
            }
        }
        if estpremier{
            premier.push(i);
           nb=nb+1; 
        }
        i+=1;
    }
    return premier[10000];
}