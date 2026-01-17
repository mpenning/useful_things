use std::boxed::Box;
use std::rc::Rc;

trait Vehicle {
    // The Vehicle trait only implements one
    // function -> drive()
    fn drive(&self);
}

struct Truck;

// Implement the Vehicle trait for the Truck
// struct
impl Vehicle for Truck {
    fn drive(&self) {
        println!("Truck is driving");
    }
}

fn main() {
    // Box acts like a normal pointer, but
    // it stores the value in the heap instead of the stack
    //
    // Anytime you reference a trait, it must
    // be prefixed with dyn
    //
    // The problem with Vehicle trait is that we
    // can't know the size at complile-time.
    // Therefore we must store t on the
    // heap (i.e. use Box<T>)
    let t: Box<dyn Vehicle>;
    t = Box::new(Truck);
    t.drive();
}
