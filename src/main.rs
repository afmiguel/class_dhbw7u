#[allow(unused_imports)]
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[allow(dead_code)]
struct MyStruct {
    data: i32,
    sibling: Option<Weak<RefCell<MyStruct>>>,
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping MyStruct with data: {}", self.data);
    }
}

fn main() {
    let i1 = Rc::new(RefCell::new(MyStruct {
        data: 10,
        sibling: None,
    }));
    let i2 = Rc::new(RefCell::new(MyStruct {
        data: 20,
        sibling: None,
    }));

    i1.borrow_mut().sibling = Some(Rc::downgrade(&i2));
    i2.borrow_mut().sibling = Some(Rc::downgrade(&i1));

    println!("The value of field1 is: {}", i1.borrow().data);
    println!("The value of field1 is: {}", i2.borrow().data);

    match &i1.borrow().sibling{
        Some(w) => {},
        None => println!("i1 não tem irmão!") 
    }
}