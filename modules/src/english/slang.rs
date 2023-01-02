// This line tells Cargo to look for the file "slang/wtf.rs" or "slang/wtf/mod.rs".

// **CREATE** the module "wtf". It is not declared as "public". Thus, it will not
// be accessible from the outside of this module.
mod wtf;

pub fn tell_slang() {
    println!("No I won't tell slang!")
}
