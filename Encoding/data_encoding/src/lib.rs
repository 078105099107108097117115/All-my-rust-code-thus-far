
#[cfg(test)]
mod tests {
    use data_encoding::HEXUPPER;
    #[test]
    fn it_works() {
        let deadbeef = vec![0xde,0xad,0xbe,0xef];
        let deadbeef_iter = (&deadbeef).into_iter();
        for val in deadbeef_iter {
            println!("{}",val);
        }
        assert_eq!(HEXUPPER.decode(b"DEADBEEF").unwrap(),deadbeef);
        assert_eq!(HEXUPPER.encode(&deadbeef),"DEADBEEF");
        //assert_eq!(2 + 2, 4);
    }
}
