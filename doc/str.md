# What is "str"

## UTF-8: one byte long character 

Let's consider the following code:

```
  1	fn main() {
  2	    let str1: &str = &"abc";
  3	    println!("str1: {}", str1);
  4	}
```

Build the binary:

```bash
$ RUSTFLAGS=-g cargo build
Compiling variables v0.1.0 (/home/denis/Documents/github/rust-playground/variables)
Finished dev [unoptimized + debuginfo] target(s) in 0.47s
```

Then fire up GDB and explore the binary:

```bash
$ gdb target/debug/variables
...
(gdb) b 3
Breakpoint 1 at 0x795c: file src/main.rs, line 3.
(gdb) r
Starting program: /home/denis/Documents/github/rust-playground/variables/target/debug/variables
...
Breakpoint 1, variables::main () at src/main.rs:3
3	    println!("str1: {}", str1);
(gdb) explore str1
The value of 'str1' is a struct/class of type '&str' with the following fields:

data_ptr = <Enter 0 to explore this field of type '*mut u8'>
length = 3 .. (Value of type 'usize')

Enter the field number of choice: 0
'str1.data_ptr' is a pointer to a value of type 'u8'
Continue exploring it as a pointer to a single value [y/n]: n
Continue exploring it as a pointer to an array [y/n]: n

Returning to parent value...

The value of 'str1' is a struct/class of type '&str' with the following fields:

data_ptr = <Enter 0 to explore this field of type '*mut u8'>
length = 3 .. (Value of type 'usize')

Enter the field number of choice:
(gdb) p str1.data_ptr
$1 = (*mut u8) 0x555555591000
(gdb) x/3c 0x555555591000
0x555555591000:	97 'a'	98 'b'	99 'c'
(gdb) quit
```

This experience tells us that a variable of type "str" is made of 2 fields:

* The field `data_ptr` that contains a pointer to the first character of the slice.
  GDB: "_'str1.data_ptr' is a pointer to a value of type 'u8'_".
* The field `length` that contains the number of characters in the slice.

## UTF-8: three bytes long character

Let's consider the following code:

```
  1	fn main() {
  2	    let str1: &str = &"abc";
  3	    println!("str1: {}", str1);
  4	    let str1: &str = &"ᛨᛞᛝ";
  5	    println!("str1: {}", str1);
  6	}
```

Then fire up GDB and explore the binary:

```
$ gdb target/debug/variables
...
(gdb) b 5
Breakpoint 1 at 0x79ce: file src/main.rs, line 5.
(gdb) r
Starting program: /home/denis/Documents/github/rust-playground/variables/target/debug/variables
...
str1: abc

Breakpoint 1, variables::main () at src/main.rs:5
5	    println!("str1: {}", str1);
(gdb) p str1.data_ptr
$1 = (*mut u8) 0x55555559100a
(gdb) p str1.length
$2 = 9
```

We can see that the length of the slice is 9 bytes (which represents 3 UTF-8 characters).

