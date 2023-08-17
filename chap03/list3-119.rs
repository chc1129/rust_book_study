let a = Box::new(10);        // type inference
let a = Box::<i32>::new(20); // explicit type
let a = 30;             // immutable object
let b = Box::new(a);    // move object from stack memory to heap memory
let c = *b;             // dereference
