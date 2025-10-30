#[allow(unused_imports)]
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
struct MyStruct {
    data: i32,
    sibling: Option<Rc<RefCell<MyStruct>>>,
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

    i1.borrow_mut().sibling = Some(i2);

    println!("The value of field1 is: {}", i1.data);
    println!("The value of field1 is: {}", i2.data);
}