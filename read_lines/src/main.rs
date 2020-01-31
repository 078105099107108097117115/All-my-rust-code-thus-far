use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;


static LOREM_IPSUM : &str = 
" Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
";


fn main() {
    //File hosts must exist in current path before this produces output
    
    println!("{}",LOREM_IPSUM);        
    let path = Path::new("out/lorem_ipsum.txt");
    let display = path.display();
    
    //Open a file in write-only mode, returns io::REsult<File>
    
    let mut file = match File::create(&path) {
        Err(err) => panic!("couldn't write {} : {}", display, err.description()),
        Ok(file) => file,
    };

    //Write the string to file, returns io::Result<()>
    
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(err) => panic!("couldn't write to {}: {}", display, err.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }
    
    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {} :{}",
                           display,
                           why.description()),
        Ok(file) =>file,
    };

    //Read the file contents into a string, returns an io::Result<usize>
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(err) => panic!("couldn't read {} : {}", display, err.description()),
        Ok(_) => println!("{} contains:\n{}", display, s),
    }
    
    if let Ok(lines) = read_lines("./hosts") {
        
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}


fn read_lines<P>(filename : P) -> io::Result<io::Lines<io::BufReader<File>>>
where P : AsRef<Path>, {
    let file = File::open(filename)?;
    
    //The method lines() returns an iterator over the lines of a file
    
    Ok(io::BufReader::new(file).lines())
}


