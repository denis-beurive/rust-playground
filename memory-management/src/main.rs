fn main() {

    fn rerun(n: &mut i32) {
        let var = *n * 2;
        println!("n: {}, var: {} (&var: {:p})", *n, var, &var);
        *n = *n+1;
    }

    let mut n = 0;
    while n < 4 { rerun(&mut n); }
}
