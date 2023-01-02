// We **CREATE** two sub-modules, with the keyword "mod".

// This tells Cargo to look for the files "src/japanese/farewells.rs" or "src/japanese/farewells/mod.rs".
// The "pub" keyword makes the sub module "japanese::farewell" public.
pub mod farewells;

// This tells Cargo to look for the files "src/japanese/greetings.rs" or "src/japanese/greetings/mod.rs".
// The "pub" keyword makes the sub module "japanese::greetings" public.
pub mod greetings;
