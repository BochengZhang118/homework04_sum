fn sum_list(list:&[u32]) ->Option<u32>{
    let mut  sum =    list.iter();
    sum.try_fold(0u32,|acc,&sum| acc.checked_add(sum))
    
}

fn main() {
    let list = [934444778,938372737,942277343,941183785,938737835,9729735,493583735,68737295];
    let listt = [1,4];
    let s = sum_list(&list);
    let ss = sum_list(&listt);
    match s {
        Some(c)=>println!("sum is {}",c),
        None =>{
            println!("None")
        }
    }
    match ss {
        Some(c)=>println!("sum is {}",c),
        None =>{
            println!("None")
        }
    }
}
