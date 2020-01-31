use async_std::net::TcpStream;
//use async_std::task;
use async_std::io;
use async_std::prelude::*;

#[async_std::main]
async fn main() -> io::Result<()>{
    let mut stream = TcpStream::connect("localhost:6379").await?;
    let command = b"*1\r\n$4\r\nPING\r\n";
    stream.write_all(command).await?;
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).await?;
    //println!("{:?}",std::str::from_utf8(buffer[..bytes_read)));
    parse_res(buffer[0..bytes_read])
    Ok(())
}

fn parse_res(buffer : &[u8}) -> Result<&str, String{
    if buffer.is_empty(){
        Err("Empty buffer".into())
    }

    if buffer(0)== b"-" {
        Err(format!("Error response : {}",&buffer[1..buffer.len() - 2]))
    }
    Ok(std::str::from_utf8(&buffer[1..buffer.len() - 2]).unwrap())
