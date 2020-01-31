use std::ops::Drop;
use std::io::{self, Write};
//use std::io::prelude::*;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};


#[derive(Debug)]
struct Appellation{
    name : String,
    nicknames : Vec<String>
}



impl Drop for Appellation{
    fn drop(&mut self){
        print!("Dropping {}",self.name);
        if !self.nicknames.is_empty(){
            print!(" (AKA {})",self.nicknames.join(","));
        }
        println!("");
    }
}

#[allow(dead_code)]
struct RcBox<T : ?Sized> {
    ref_count : usize,
    value : T
}

fn display(boxed : &RcBox<Display>) {
    println!("For your enjoyment : {}", &boxed.value);
}

struct Selector<T> {
    elements : Vec<T>,
    current : usize,
}
impl<T> Deref for Selector<T>{
    type Target = T;
    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
}
impl<T> DerefMut for Selector<T> {
    //type Target = T;
    fn deref_mut(&mut self) -> &mut T {
        &mut self.elements[self.current]
    }
}

fn main() {
    let v : Vec<i32> = Vec::new();
    println!("{}",v.is_empty());
    print!("Before 1st assignment of Appellation"); 
    io::stdout().flush().unwrap();
    let mut a = Appellation{
        name : "Zeus".to_string(),
        nicknames : vec!["cloud collector".to_string(),
        "king of all greek gods".to_string(),
        String::from("Lightning god")
        ]
    };
    println!("The 1st assignment yields a = {:?}",a);
    println!("Before second assignment!!");
    a = Appellation{
        name : "Hera".to_string(),
        nicknames : vec![String::from("")]
    };
    println!("The second assignment of a yields :-> {:#?}",a);
    println!("at the end of the block");

    println!("");
    println!("And now for the Sized trait type");
    let boxed_lunch : RcBox<String> = RcBox {
        ref_count : 1,
        value : "lunch".to_string()
    };
    //let boxed_displayable : &RcBox<dyn Display> = &boxed_lunch;
    display(&boxed_lunch);

    println!("");
    println!("And now for the Deref and DerefMut trait examples");
    let mut s = Selector{
        elements : vec!['a','b','v','n','n','o'],
        current : 3
    };
    assert_eq!(*s, 'n');
    assert!(s.is_alphanumeric());
    *s = 'w';
    assert_eq!(s.elements, ['a','b','v','w','n','o']);
    println!("");
    show(&s);
    println!("");
    generic_show(&s as &char);
    println!("");

}

fn show(item : &char) {
    println!(" show(&s) ->  '(  {:#?}  )'",item);
}

fn generic_show<T : Display>(item : T) {
    println!(" output of generic_show  -> {}",item);
}
