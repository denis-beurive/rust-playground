// We **CREATE** two sub-modules, with the keyword "mod".

// This tells Cargo to look for the files "src/english/farewells.rs" or "src/english/farewells/mod.rs".
// The "pub" keyword makes the sub module "english::farewell" public.
pub mod farewells;

// This tells Cargo to look for the files "src/english/greetings.rs" or "src/english/greetings/mod.rs".
// The "pub" keyword makes the sub module "english::greetings" public.
pub mod greetings;

// This tells Cargo to look for the files "src/english/slang.rs" or "src/english/slang/mod.rs".
// Please note that the keyword "mod" is not preceded by the keyword "pub".
// Therefore, the sub-module "slang" will not be accessible from the outside of the "english"
// module.
mod slang;

pub fn tell_slang() {
    slang::tell_slang();
}
