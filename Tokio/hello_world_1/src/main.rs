use std::{thread, time};
use tokio::io;
use tokio::net::TcpStream;
use tokio::prelude::*;

fn main() {
    let address = "127.0.0.1:6142".parse().unwrap();
    let sec = time::Duration::from_millis(6000);
    let now = time::Instant::now();
    let client = TcpStream::connect(&address)
        .and_then(move |stream|{
            thread::sleep(sec);
            println!("The stream has being created!! :-) ");
            thread::sleep(sec);
            let output_to_stream = format!("hello there! I am a stream coming to you and time elapsed = {:?}\n",now.elapsed());
            io::write_all(stream, output_to_stream)
                .then(move |result| {
                    thread::sleep(sec);
                    println!("Success! Successfully wrote to stream; success = {:?} after {:?} seconds",
                             result.is_ok(),
                             now.elapsed());
                    Ok(())
                })
        })
    .map_err(|err| {
        //We are going to log all our errors to stdout
        println!("connection error! ==> {:?}",err);
    });

    println!("About to create the stream and write to it..... ;-) ");
    tokio::run(client);
    println!("Stream has been created and written to .... |:-) after {:?} seconds",now.elapsed());

}
