//#[cfg(test)]
//mod tests {
  //  #[test]
    //fn it_works() {
      //  assert_eq!(2 + 2, 4);
    //}
//}
pub mod line{
    use std::io::{self, Write};
    pub fn line_fxn() -> Result<(),Box<dyn std::error::Error>> {
        let current_line = line!();
        let stdout = io::stdout();
        let mut handle = io::BufWriter::new(stdout);
        writeln!(handle, "defined on line {}",current_line)?;
        Ok(())
    }
}
