fn sqr(x : u32, exponent: u32) -> Option<u32>{
    //x is the base
    x.checked_pow(exponent)
}
fn pattern_match_sqr_res(a: u32, b: u32){
    if let Some(res) = sqr(a,b) {
        println!("{} raised to power {}  == {}",a, b, res);
    }
    if let None= sqr(a,b){
        println!("This number -> {} raised to pow {} cannot be computed",a,b);
    }
}

fn main() {
    let first_base = 4;
    let second_base = u32::max_value();
    pattern_match_sqr_res(first_base,4);
    pattern_match_sqr_res(second_base,4);
}
