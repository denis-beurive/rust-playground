fn main() {
    let var1 = 10;
    println!("var1: \"{}\" ({:p})", var1, &var1);
    println!("Pause");
    let var2 = var1;
    println!("var2: \"{}\" ({:p})", var2, &var2);
    println!("The end");
}
