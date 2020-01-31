use std::net::Ipv4Addr;

fn ping<A>(address : A) -> std::io::Result<bool>
where A : Into<Ipv4Addr>
{
    let ipv4_addr = address.into();

}
fn main() {
    
    //println!("Hello, world!");
}
