use std::ffi::OsStr;
use std::path::Path;
use std::iter::FromIterator;


fn main() {
    let mut v = vec!["antimony","arsenic","aluminium","copper","tin","selenium","gallium"];
    let mut iterator = (&v).into_iter();
    while let Some(elem) = iterator.next(){
        println!("{}",elem);
    }
    println!();
    let path = Path::new("/usr/local/bin/python3");
    let mut iterator = (&path).into_iter();
    while let Some(item) = iterator.next(){
        println!("{:?}",item);
        //The iterator's item is of type &std::ffi::OsStr
    }
    println!("{:?}",&path);
    println!("{:?}",&path);
    let drained_vec = Vec::from_iter(v.drain(1..2));
    println!("drained_vec == {:?}",drained_vec);
    let daily_temps = vec![23,24,100,99,90,0,8,10,23,2,45,1,2,9];
    println!("daily_temps === {:?}",daily_temps);
    println!("len of daily_temps == {}",daily_temps.len());
    let changes = daily_temps
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect::<Vec<_>>();
    println!("changes == {:?}",changes);
    println!("len of changes == {}",changes.len());
    let three_sum = daily_temps
        .window(2)
        .map(|w| fold(0, |sum
}
