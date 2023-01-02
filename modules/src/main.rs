// Notes:
// - "main.rs" does not declare any modules with the keyword "mod", it only imports them
//   with the keyword "use".
// - The line "use modules" is optional. In projects with both a "lib.rs" and a "main.rs",
//   Cargo effectively treats "lib.rs" as the root of your crate, and "main.rs" as a separate
//   binary that depends on your crate.

use modules; // This line is optional since, by convention, "main.rs" **IS** the entry point
             // for the crate. Thus, there is no need to import the module.

fn main() {
    modules::id();
    modules::english::tell_slang();
    modules::english::farewells::say();
    modules::english::greetings::say();
    modules::japanese::farewells::say();
    modules::japanese::greetings::say();
}
