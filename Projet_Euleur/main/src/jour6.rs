pub fn probleme6() ->u64{
    let mut i:u32 = 1;
    let mut avant:u64 = 0;
    let mut apres:u64 = 0;
    while i<=100{
        avant =  avant +(i*i) as u64;
        apres = apres +i as u64;
        i=i+1;
    } 
    return apres*apres - avant;
}