
// s takes ownership of the value which is passed to the function.
fn print_it1(s: String) {
    println!("s: \"{}\" (@(s): {:p})", s, &s);
    // The variable s disappears here.
    // Therefore, the memory allocated for the value (the string) is freed.
}

// s does **NOT** borrow the value which is passed to the function.
fn print_it2(s: &String) {
    println!("value(s): \"{}\" (s: {:p})", s, s);
    // The variable s disappears here.
    // But it is a reference (to the value). It does not owns the value.
    // Therefore, the memory allocated for the value (the string) is not freed.
}

// s takes ownership of the value which is passed to the function.
// But, it returns this value.
fn print_it3(s: String) -> String {
    println!("s: \"{}\" (@(s): {:p})", s, &s);
    return s;
}

// s borrows the value which is passed to the function.
fn print_it4(s: &mut String) {
    println!("value(s): \"{}\" (s: {:p})", s, s);
    // We change the value borrowed by the function's parameter (s).
    *s = String::from("new value");
    // The variable s disappears here.
    // But it is a reference (to the value). It does not owns the value. It has only borrowed it.
    // Therefore, the memory allocated for the value (the string) is not freed.
}


fn borrowing() {
    // let x: <type> = ...;
    // let r: &<type> = &x;
    //    => reference that can be used to read (only) the referenced value.
    //       r does **NOT** borrow the ownership of the value (owned by the variable x).
    //
    // let mut x: <type> = ...;
    // let r: &mut <type> = &mut x;
    //    => reference that can be used to read and write the referenced value.
    //       r borrows the ownership of the value (previously owned by the variable x).
    //       Thus, you cannot use x to access the value (since it does not own it).


    let x = 1;
    // The variables r1 and r2 are references to x.
    // But they can only be used to read the value owned by x (&x).
    // WARNING: you cannot use theses references to change the value owned by x.
    let r1: &i32 = &x;
    let r2: &i32 = &x;
    println!("value of *r1: {}", *r1);
    println!("value of *r2: {}", *r2);
    println!("value of x: {}", x);

    // Please note that the variable x is mutable. This means that we can change its value.
    let mut x: i32 = 1;
    // Please note that the variable rw is a reference to a mutable value.
    // The variable rw borrows the value of x. Thus x does not own the value anymore.
    // And thus, you cannot use x to access the value!
    let rw: &mut i32 = &mut x;
    println!("value of *rw: {}", *rw);
    *rw = 10;
    println!("value of *rw: {}", *rw);
    *rw = 20;
    println!("value of *rw: {}", *rw);
    // Do not try to use the variable x (since it does not own "its" value, which has been borrowed).
}

fn main() {

    // Keep in mind:
    // (1) Resources can only have one owner.
    // (2) A reference of a variable is a pointer that leads to that variable.
    //     It is *NOT* a pointer that leads to the variable's value.

    println!("TEST 1");
    // The value "this is a string" is the resource.
    // It can only have one owner, which is a variable x.
    let x = String::from("this is a string"); // the variable x is the owner of the value "this is a string".
    { let x = String::from("hello from another scope"); println!("x: {}", x); }
    println!("x: \"{}\" (@(x): {:p})", x, &x);
    // The value (the resource) is moved to another owner (the input parameter of the function).
    print_it1(x);
    // From this point, the variable x does not own the value "this is a string" anymore.
    // The variable x does not own any value anymore. It does not exist anymore.

    println!("TEST 2");
    // The value "this is a string" is the resource.
    // It can only have one owner, which is a variable x.
    let x = String::from("this is a string");
    println!("x: \"{}\" (@(x): {:p})", x, &x);
    // Here, we pass the reference of the variable x to the function.
    // Please note that a reference is a scalar. It is passed through the stack.
    print_it2(&x);
    println!("x: \"{}\" (@(x): {:p})", x, &x);
    // Since the value's owner (that is, the variable x) did not change, the variable x still
    // owns the value. Thus, the variable x still exists.

    println!("TEST 3");
    // The value "this is a string" is the resource.
    // It can only have one owner, which is a variable x.
    // Please note that the variable x is now mutable.
    let mut x: String = String::from("this is a string");
    println!("x: \"{}\" (@(x): {:p})", x, &x);
    // We transfer the ownership of the value "this is a string" from the variable x to the
    // function's parameter.
    // Because x is mutable, we can assign it a new value.
    x = print_it3(x); // watch out for the return value!
    println!("x: \"{}\" (@(x): {:p})", x, &x);
    // Please note: the value of &x does not change (this means that it is the same variable).

    println!("TEST 4");
    // The value "this is a string" is the resource.
    // It can only have one owner, which is a variable x.
    // Please note that the variable x is mutable.
    let mut x: String = String::from("this is a string");
    println!("x: \"{}\" (@(x): {:p})", x, &x);
    // The function's parameter borrows the value of x.
    print_it4(&mut x);
    println!("x: \"{}\" (@(x): {:p})", x, &x);
    // Please note: the value of &x does not change (this means that it is the same variable).

    println!("TEST 5");
    borrowing();
}
