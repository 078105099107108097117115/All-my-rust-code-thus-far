
fn main(){
    //tests::it_works();
    let deadbeef = vec![0xde,0xad,0xbe,0xef];
    let deadbeef_iter = (&deadbeef).into_iter();
    for val in deadbeef_iter {
        println!("{}",val);
    }

}
