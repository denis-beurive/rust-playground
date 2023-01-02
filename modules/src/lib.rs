// Note:
// - When you use the "mod" keyword, you are creating a module, not importing a module.
// - You must not declare the module "packages" here since, by convention, the file "lib.rs"
//   implements this module.

// We **CREATE** the module called "japanese".
//
// This tells Cargo to look for the files "src/japanese.rs" or "src/japanese/mod.rs".
// The "pub" keyword makes the sub module "japanese" public.
pub mod japanese;

// We **CREATE** the module called "english".
//
// This tells Cargo to look for the files "src/english.rs" or "src/english/mod.rs".
// The "pub" keyword makes the sub module "english" public.
pub mod english;

// The "pub" keyword makes the function "id()" public.
pub fn id() {
    println!("This is 'lib.rs'")
}
