struct MyStruct {
    text: &'static str,
    number: u32,
}

impl MyStruct {
    fn new (text: &'static str, number: u32) -> MyStruct {
        MyStruct {
            text,
            number,
        }
    }
    fn get_number(&self)-> u32{
        self.number
    }
    fn inc_number (&mut self) {
        self.number +=1;
    }
    fn destructor (self) {
        println!("Destructing {}" , self.text);
    }
}

//Only used for type check
fn is_fn <A, R>(_x: fn(A) -> R) {}
fn is_Fn <A, R, F: Fn(A) -> R> (_x: &F) {}
fn is_FnMut <A, R, F: FnMut(A) -> R> (_x: &F) {} fn is_FnOnce <A, R, F: FnOnce(A) -> R> (_x: &F) {}

#[test]
// No Context and the fn (lowercase f) type
fn closure1_test() {
    println!(" -= different type closures : type 01 =-");

    let obj1 = MyStruct::new("Hello" , 15);
    let obj2 = MyStruct::new("More Text" , 10);

    let closure1 = |x:&MyStruct| -> u32 {
        x.get_number() + 3
    };
    assert_eq!(closure1(&obj1), 18);
    assert_eq!(closure1(&obj2), 13);

    is_fn(closure1);
    is_Fn(&closure1);
    is_FnMut(&closure1);
    is_FnOnce(&closure1);
}

#[test]
// Immutable context and the Fn (Capital F) trait
fn closure2_test() {
    let obj1 = MyStruct::new("Hello", 15);
    let obj2 = MyStruct::new("Mote Text", 10);

    let closure2 = |x:&MyStruct| -> u32 {
        x.get_number() + obj1.get_number()
    };

    assert_eq!(closure2(&obj2) , 25);

    // We can borrow obj1 again immutably
    assert_eq!(obj1.get_number(), 15);

    // But we can't borrow it mutably
    // obj1.inc_number();

    // Does not compile:
    // is_fn(closure2);
    // Compiles successfully:
    is_Fn(&closure2);
    is_FnMut(&closure2);
    is_FnOnce(&closure2);
}

#[test]
//Mutable context and the FnMut trait
fn closure3_test(){
    //⚠️ mut
    let mut obj1 = MyStruct::new("Hello", 15);
    let obj2 = MyStruct::new("More Text", 10);

    //⚠️ mut
    let mut closure3 = |x: &MyStruct| -> u32 {
        obj1.inc_number();
        x.get_number() + obj1.get_number()
    };

    assert_eq!(closure3(&obj2), 26);
    assert_eq!(closure3(&obj2), 27);
    assert_eq!(closure3(&obj2), 28);

    // We can't borrow obj1 mutably or immutably
    // assert_eq!(obj1.get_number(), 18); // ERROR
    // obj1.inc_number(); // ERROR

    // Does not compile:
    // is_fn(closure3);
    // is_Fn(&closure3);
    // Compiles successfully:
    is_FnMut(&closure3); is_FnOnce(&closure3);
}

#[test]
// Owned Context
fn closure4_test(){
    let obj1 = MyStruct::new("Hello", 15);
    let obj2 = MyStruct::new("More Text", 10);

    // obj1 is owned by the closure
    let closure4 = |x: &MyStruct| -> u32 {
        obj1.destructor();
        x.get_number()
    };

    // Does not compile:
    // is_fn(closure4);
    // is_Fn(&closure4);
    // is_FnMut(&closure4);
    // Compiles successfully:
    is_FnOnce(&closure4);

    assert_eq!(closure4(&obj2), 10);
    // We can't call closure4 twice...
    // assert_eq!(closure4(&obj2), 10);//ERROR

    // We can't borrow obj1 mutably or immutably
    // assert_eq!(obj1.get_number(), 15);//ERROR
    // obj1.inc_number();//ERROR
}

fn main() {
    println!(" -= different type closures =-");
}