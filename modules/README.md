# Rust modules

* "`main.rs`" does not declare any modules with the keyword "`mod`", it only imports them
  with the keyword "`use`" ([source](https://users.rust-lang.org/t/main-rs-and-lib-rs-at-same-level/42499/2)).
* In projects with both a "`lib.rs`" and a "`main.rs`", Cargo effectively treats "`lib.rs`" as the root of your 
  crate, and "`main.rs`" as a separate binary that depends on your crate
  ([source](https://users.rust-lang.org/t/main-rs-and-lib-rs-at-same-level/42499/2)).
* "`mod module_name`" **CREATES** a (sub) module.
* "`use module_name`" **IMPORTS** a module.
* "`mod module_name`" tells Cargo to look for the files "`module_name.rs`"
  or "`module_name/mod.rs`". 
* "`pub mod module_name`" **CREATES** a publicly accessible (sub) module.
* "`mod module_name`" **CREATES** a private (sub) module.

```
│   Cargo.toml
│
├───src
│   │   lib.rs
│   │   main.rs
│   │
│   ├───english
│   │   │   farewells.rs
│   │   │   greetings.rs
│   │   │   mod.rs
│   │   │   slang.rs
│   │   │
│   │   └───slang
│   │           wtf.rs
│   │
│   └───japanese
│           farewells.rs
│           greetings.rs
│           mod.rs
```
