# Loops

Let's execute the code given below:

```rust
fn main() {
    let mut n = 0;

    // We repeat a block (of code) execution.
    println!("-- Test 1 --");
    while n < 4 {
        let var = n * 2;
        println!("n: {}, var: {} (&var: {:p})", n, var, &var);
        n += 1;
    }

    // It is equivalent to (using a named function).
    println!("-- Test 2 --");
    fn run(n: i32) -> i32 {
        let var = n * 2;
        println!("n: {}, var: {} (&var: {:p})", n, var, &var);
        n+1
    }
    n = 0;
    while n < 4 { n = run(n); }

    // It is equivalent to (using a closure).
    println!("-- Test 3 --");
    n = 0;
    while n < 4 { n = (|x: i32| {
        let var = x * 2;
        println!("n: {}, var: {} (&var: {:p})", n, var, &var);
        x+1
    })(n); }

    // It is equivalent to (using a closure).
    // By default, closures capture their environment by borrowing.
    println!("-- Test 4 --");
    n = 0;
    while n < 4 { n = (|| {
        let var = n * 2;
        println!("n: {}, var: {} (&var: {:p})", n, var, &var);
        n+1
    })(); }

    // It is equivalent to (using a closure).
    // By default, closures capture their environment by borrowing.
    println!("-- Test 5 --");
    n = 0;
    while n < 4 { (|| {
        let var = n * 2;
        println!("n: {}, var: {} (&var: {:p})", n, var, &var);
        n = n+1
    })(); }

    println!("-- Test 6 --");
    n = 0;
    (|| {
        let var = n * 2;
        println!("n: {}, var: {} (&var: {:p})", n, var, &var);
        n = n+1
    })();
    (|| {
        let var = n * 2;
        println!("n: {}, var: {} (&var: {:p})", n, var, &var);
        n = n+1
    })();
    (|| {
        let var = n * 2;
        println!("n: {}, var: {} (&var: {:p})", n, var, &var);
        n = n+1
    })();
    (|| {
        let var = n * 2;
        println!("n: {}, var: {} (&var: {:p})", n, var, &var);
        n = n+1
    })();

    // Also similar to:
    println!("-- Test 7 --");
    fn rerun(n: &mut i32) {
        let var = *n * 2;
        println!("n: {}, var: {} (&var: {:p})", *n, var, &var);
        *n = *n+1;
    }
    n = 0;
    while n < 4 { rerun(&mut n); }
}
```

Result:

```
-- Test 1 --
n: 0, var: 0 (&var: 0x7ffcfe15e704)
n: 1, var: 2 (&var: 0x7ffcfe15e704)
n: 2, var: 4 (&var: 0x7ffcfe15e704)
n: 3, var: 6 (&var: 0x7ffcfe15e704)
-- Test 2 --
n: 0, var: 0 (&var: 0x7ffcfe15e5dc)
n: 1, var: 2 (&var: 0x7ffcfe15e5dc)
n: 2, var: 4 (&var: 0x7ffcfe15e5dc)
n: 3, var: 6 (&var: 0x7ffcfe15e5dc)
-- Test 3 --
n: 0, var: 0 (&var: 0x7ffcfe15e5cc)
n: 1, var: 2 (&var: 0x7ffcfe15e5cc)
n: 2, var: 4 (&var: 0x7ffcfe15e5cc)
n: 3, var: 6 (&var: 0x7ffcfe15e5cc)
-- Test 4 --
n: 0, var: 0 (&var: 0x7ffcfe15e5d4)
n: 1, var: 2 (&var: 0x7ffcfe15e5d4)
n: 2, var: 4 (&var: 0x7ffcfe15e5d4)
n: 3, var: 6 (&var: 0x7ffcfe15e5d4)
-- Test 5 --
n: 0, var: 0 (&var: 0x7ffcfe15e5d4)
n: 1, var: 2 (&var: 0x7ffcfe15e5d4)
n: 2, var: 4 (&var: 0x7ffcfe15e5d4)
n: 3, var: 6 (&var: 0x7ffcfe15e5d4)
-- Test 6 --
n: 0, var: 0 (&var: 0x7ffcfe15e5d4)
n: 1, var: 2 (&var: 0x7ffcfe15e5d4)
n: 2, var: 4 (&var: 0x7ffcfe15e5d4)
n: 3, var: 6 (&var: 0x7ffcfe15e5d4)
-- Test 7 --
n: 0, var: 0 (&var: 0x7ffcfe15e5d4)
n: 1, var: 2 (&var: 0x7ffcfe15e5d4)
n: 2, var: 4 (&var: 0x7ffcfe15e5d4)
n: 3, var: 6 (&var: 0x7ffcfe15e5d4)
```

As you can see, the reference to the variable `var` (`&var`) remains constant. 
The block of code bound to the "while" keyword is executed like a function (in its own context).

