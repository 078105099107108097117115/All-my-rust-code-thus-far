use std::rc::Rc;

#[derive(Debug)]
enum List{
    Cons(i32, Rc<List>),
    Nil,
}

//impl Drop for List{
//    fn drop(&mut self){
//        println!("Dropping......Count == {}", Rc::strong_count(&a));
//    }
//}

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(List::Cons(
            5,Rc::new(Cons(
                    10,Rc::new(
                        Nil
                        )
                    )
                )
            )
    );
    println!("a = {:?}",a);
    println!("The reference count after creating a = {}", Rc::strong_count(&a));
    {
        let b = Cons(3, Rc::clone(&a));
        println!("The reference count after creating b is {}",Rc::strong_count(&a));
        println!("b = {:?}",b);
        {
            let c = Cons(4, Rc::clone(&a));
            println!("c == {:?}",c);
            println!("The reference count after creating c is {}", Rc::strong_count(&a));
        }
        println!("Dropped c, Reference count after = {}", Rc::strong_count(&a));
    }
    println!("Dropped b .... Reference count after = {}", Rc::strong_count(&a));
    let _x = "Napoleon Bonaparte was militarily, a strategical and tactical genius".to_string();
    //let y = &mut x;
    //println!("y = {}",*y);
}
