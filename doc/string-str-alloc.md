# String and str allocation

In this [Rust documentation](https://doc.rust-lang.org/rust-by-example/std/str.html), it is written that `String` is allocated on the heap.

Let's find out...

Let's consider the basic Rust source file below:

```rust
// rustc -o string_vs_str -C opt-level=0 -C debuginfo=2 main.rs

fn main() {
    let my_string: String = String::from("abc");
    let my_str: &str = "def";
    println!("{}{}", my_string, my_str);
    let my_string_str: &str = my_string.as_str();
    println!("my_string_str: {}", my_string_str);
}
```

Let's execute RUST-GDB on the executable, upon the following commands:

```gdb
# rust-gdb --batch --command=test.gdb --args ./string_vs_str

set width 0
set height 0
set verbose off

### Set 2 breakpoints
b main.rs:6
b main.rs:8

### Start the process
r

### Display the memory mapping into the file "map.txt"
set logging redirect on
set logging file map.txt
set logging overwrite on
set logging enabled on
info proc map
set logging enabled off

### Get information about "my_string"
echo ==== my_string ====\n
print my_string
echo type:\n
ptype my_string
echo &my_string:\n
print &my_string
echo my_string.vec.buf.ptr.pointer.pointer:\n
print my_string.vec.buf.ptr.pointer.pointer
c

### Get information about "my_str"
echo ==== my_str ====\n
print my_str
echo type:\n
ptype my_str
echo &my_str:\n
print &my_str
echo my_str.length:\n
print my_str.length
echo &my_str.length:\n
print &my_str.length
echo my_str.data_ptr:\n
print my_str.data_ptr
```

The result is given below:

```
Breakpoint 1 at 0x8ea7: file main.rs, line 6.
Breakpoint 2 at 0x8fa9: file main.rs, line 8.
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/x86_64-linux-gnu/libthread_db.so.1".

Breakpoint 1, main::main () at main.rs:6
6	    println!("{}{}", my_string, my_str);
==== my_string ====
$1 = "abc"
type:
type = struct alloc::string::String {
  vec: alloc::vec::Vec<u8, alloc::alloc::Global>,
}
&my_string:
$2 = (*mut alloc::string::String) 0x7fffffffd960
my_string.vec.buf.ptr.pointer.pointer:
$3 = (*mut u8) 0x5555555a5ba0
abcdef

Breakpoint 2, main::main () at main.rs:8
8	    println!("my_string_str: {}", my_string_str);
==== my_str ====
$4 = "def"
type:
type = struct &str {
  data_ptr: *mut u8,
  length: usize,
}
&my_str:
$5 = (*mut &str) 0x7fffffffd978
my_str.length:
$6 = 3
&my_str.length:
$7 = (*mut usize) 0x7fffffffd980
my_str.data_ptr:
$8 = (*mut u8) 0x55555559304f
```

And the content of the (logged) file "`map.txt`" is:

```
process 7725
Mapped address spaces:

          Start Addr           End Addr       Size     Offset  Perms  objfile
      0x555555554000     0x55555555a000     0x6000        0x0  r--p   /home/denis/Documents/github/tester/src/string_vs_str
      0x55555555a000     0x555555593000    0x39000     0x6000  r-xp   /home/denis/Documents/github/tester/src/string_vs_str
      0x555555593000     0x5555555a1000     0xe000    0x3f000  r--p   /home/denis/Documents/github/tester/src/string_vs_str
      0x5555555a1000     0x5555555a4000     0x3000    0x4c000  r--p   /home/denis/Documents/github/tester/src/string_vs_str
      0x5555555a4000     0x5555555a5000     0x1000    0x4f000  rw-p   /home/denis/Documents/github/tester/src/string_vs_str
      0x5555555a5000     0x5555555c6000    0x21000        0x0  rw-p   [heap]
      0x7ffff7d5c000     0x7ffff7d5f000     0x3000        0x0  rw-p   
      0x7ffff7d5f000     0x7ffff7d87000    0x28000        0x0  r--p   /usr/lib/x86_64-linux-gnu/libc.so.6
      0x7ffff7d87000     0x7ffff7f1c000   0x195000    0x28000  r-xp   /usr/lib/x86_64-linux-gnu/libc.so.6
      0x7ffff7f1c000     0x7ffff7f74000    0x58000   0x1bd000  r--p   /usr/lib/x86_64-linux-gnu/libc.so.6
      0x7ffff7f74000     0x7ffff7f78000     0x4000   0x214000  r--p   /usr/lib/x86_64-linux-gnu/libc.so.6
      0x7ffff7f78000     0x7ffff7f7a000     0x2000   0x218000  rw-p   /usr/lib/x86_64-linux-gnu/libc.so.6
      0x7ffff7f7a000     0x7ffff7f87000     0xd000        0x0  rw-p   
      0x7ffff7f87000     0x7ffff7f8a000     0x3000        0x0  r--p   /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
      0x7ffff7f8a000     0x7ffff7fa1000    0x17000     0x3000  r-xp   /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
      0x7ffff7fa1000     0x7ffff7fa5000     0x4000    0x1a000  r--p   /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
      0x7ffff7fa5000     0x7ffff7fa6000     0x1000    0x1d000  r--p   /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
      0x7ffff7fa6000     0x7ffff7fa7000     0x1000    0x1e000  rw-p   /usr/lib/x86_64-linux-gnu/libgcc_s.so.1
      0x7ffff7fb8000     0x7ffff7fb9000     0x1000        0x0  ---p   
      0x7ffff7fb9000     0x7ffff7fbd000     0x4000        0x0  rw-p   
      0x7ffff7fbd000     0x7ffff7fc1000     0x4000        0x0  r--p   [vvar]
      0x7ffff7fc1000     0x7ffff7fc3000     0x2000        0x0  r-xp   [vdso]
      0x7ffff7fc3000     0x7ffff7fc5000     0x2000        0x0  r--p   /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
      0x7ffff7fc5000     0x7ffff7fef000    0x2a000     0x2000  r-xp   /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
      0x7ffff7fef000     0x7ffff7ffa000     0xb000    0x2c000  r--p   /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
      0x7ffff7ffb000     0x7ffff7ffd000     0x2000    0x37000  r--p   /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
      0x7ffff7ffd000     0x7ffff7fff000     0x2000    0x39000  rw-p   /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2
      0x7ffffffde000     0x7ffffffff000    0x21000        0x0  rw-p   [stack]
  0xffffffffff600000 0xffffffffff601000     0x1000        0x0  --xp   [vsyscall]
```

Let's find the locations of the following addresses:

* `0x7fffffffd960`: `&my_string`
* `0x5555555a5ba0`: `my_string.vec.buf.ptr.pointer.pointer`
* `0x7fffffffd978`: `&my_str`
* `0x7fffffffd980`: `&my_str.length`
* `0x55555559304f`: `my_str.data_ptr`

```bash
$ perl mapping.pl -- 0x7fffffffd960 0x5555555a5ba0 0x7fffffffd978 0x55555559304f 0x7fffffffd980
-----------------------------------------------------------------------------------------------------------------
/home/denis/Documents/github/tester/src/string_vs_str: 0x555555554000              -> 0x5555555A4FFF            
                                                       0x55555559304F
                                                 heap: 0x5555555A5000              -> 0x5555555C5FFF            
                                                       0x5555555A5BA0
                                            unnamed-1: 0x7FFFF7D5C000              -> 0x7FFFF7D5EFFF            
                  /usr/lib/x86_64-linux-gnu/libc.so.6: 0x7FFFF7D5F000              -> 0x7FFFF7F79FFF            
                                            unnamed-2: 0x7FFFF7F7A000              -> 0x7FFFF7F86FFF            
              /usr/lib/x86_64-linux-gnu/libgcc_s.so.1: 0x7FFFF7F87000              -> 0x7FFFF7FA6FFF            
                                            unnamed-3: 0x7FFFF7FB8000              -> 0x7FFFF7FB8FFF            
                                            unnamed-4: 0x7FFFF7FB9000              -> 0x7FFFF7FBCFFF            
                                                 vvar: 0x7FFFF7FBD000              -> 0x7FFFF7FC0FFF            
                                                 vdso: 0x7FFFF7FC1000              -> 0x7FFFF7FC2FFF            
       /usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2: 0x7FFFF7FC3000              -> 0x7FFFF7FFEFFF            
                                                stack: 0x7FFFFFFDE000              -> 0x7FFFFFFFEFFF            
                                                       0x7FFFFFFFD960
                                                       0x7FFFFFFFD978
                                                       0x7FFFFFFFD980
                                             vsyscall: 0xFFFFFFFFFF600000          -> 0xFFFFFFFFFF600FFF        
-----------------------------------------------------------------------------------------------------------------
```

> Script [mapping.pl](https://github.com/denis-beurive/gdb-notes/blob/master/tools/mapping.pl)

Conclusion:

| data                                     | location   | note                                                      |
|------------------------------------------|------------|------------------------------------------------------------
| `&my_string`                             | `stack`    | the variable `my_string` is allocated on the stack        |
| `my_string.vec.buf.ptr.pointer.pointer`  | `heap`     | the data `my_string` points at allocated on the heap      |
| `&my_str`                                | `stack`    | the variable `my_str` is allocated on the stack           |
| `&my_str.length`                         | `stack`    | the length of `my_str` is allocated on the stack          |
| `my_str.data_ptr`                        | `.rodata`  | the data `my_str` points at is allocated on the `.rodata` |


The `String` and `&str` variables themselves are allocated on the `stack` (the length, data pointer and capacity for String). But the data they point at is allocated on the `heap` for `String`, and at `.rodata` for your `&str` (thanks to [this response to my answer](https://stackoverflow.com/questions/75124797/is-string-always-allocated-on-the-heap)).
