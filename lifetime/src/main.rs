

/// Return the longest string, or None if the strings have the same length.
/// Why do we need to specify lifetimes here?
/// The returned reference may be one of the references passed as parameters to the function.
/// In other words, the value referenced by the returned reference is correlated with the values
/// referenced by the function's parameters.
///
/// - If the returned value is the reference "string1", then the lifetime of the returned value is,
///   obviously, the lifetime of "string1".
/// - If the returned value is the reference "string2", then the lifetime of the returned value is,
///   obviously, the lifetime of "string2".
/// - If the returned value is None, then the lifetime of the returned value is not associated with
///   the lifetimes of any parameters.
///
/// But, at compile time, it is not possible to predict whether one string is longer than the other,
/// and, in this case, which string is the longest. All we can say is that the value referenced by
/// the returned reference is correlated with the values referenced by the function's parameters.
///
/// Please note that "string1" and "string2" are not mutable references. Thus, they reference
/// immutable variables (which values won't change in the future). This means that the value
/// referenced by the returned reference is not likely to change.
///
/// => The IMMUTABLE value referenced by the returned reference is correlated with the IMMUTABLE
/// values referenced by the function's parameters.

fn longest_by_str<'a>(string1: &'a str, string2: &'a str) -> Option<&'a str> {
    if string1.len() > string2.len() {
        return Some(string1);
    }
    if string2.len() > string1.len() {
        return Some(string2);
    }
    return None;
}

fn use_longest_by_str() {
    let str1: &str = "This is a longer string";
    let str2: &str = "This is a short string";
    let longest: Option<&str> = longest_by_str(str1, str2);
    println!("str1: {}\nstr2: {}", str1, str2);
    // The line below creates a new variable "str1".
    let str1: &str = "new str1";
    println!("str1: {}\nstr2: {}", str1, str2);
    if longest.is_some() {
        println!("The longest string is \"{}\"", longest.unwrap()); // => The longest string is "This is a longer string"
    }
}

/// As well as for the function "longest_by_str", the value referenced by the returned reference
/// is correlated to the values references by the function's parameters.
///
/// However, in contrast to the function "longest_by_str", in this case, all the references
/// involved are mutable.
///
/// => The MUTABLE value referenced by the returned reference is correlated with the MUTABLE
/// values referenced by the function's parameters.
///
/// This has consequences on the code that follows the call to the function. The lifetime of
/// the value referenced by the returned reference is limited to the lifetimes of the (mutable)
/// values passed as parameters to the function.

fn bigger_by_mut_u8<'a>(i1: &'a mut u8, i2: &'a mut u8) -> Option<&'a mut u8> {
    if i1 > i2 {
        return Some(i1);
    }
    if i2 > i1 {
        return Some(i2);
    }
    return None;
}

fn use_bigger_by_mut_u8() {
    let mut v1: u8 = 10;
    let mut v2: u8 = 5;
    let bigger: Option<&mut u8> = bigger_by_mut_u8(&mut v1, &mut v2);

    // As long as you don't change the values of "v1" or "v2" you can use "bigger".
    if bigger.is_some() { println!("The bigger is {}", *(bigger.unwrap())); }
    v1 = 15;

    // Below this line, you cannot use the variable "bigger" anymore since it is correlated
    // to (the values of) "v1" and "v2". If you change the value of "v1" (or "v2"), then you
    // *potentially* change the value of "bigger".

    // The code below will not compile:
    // if bigger.is_some() {
    //     println!("The bigger is {}", *(bigger.unwrap()))
    // }
}

/// Return the longest string, or None if the strings have the same length.
/// Why don't we need to specify lifetimes here?
/// The function's parameters' values are "moved" from the "caller context" to the "function context".
/// Therefore, these (parameters') values are not accessible from the caller context anymore.
/// It means that, once passed to the function, these values cannot change (or be deleted)
/// "outside of the function".
/// The returned value is the result of a "move".

fn longest_by_string(string1: String, string2: String) -> Option<String> {
    if string1.len() > string2.len() {
        return Some(string1);
    }
    if string2.len() > string1.len() {
        return Some(string2);
    }
    return None;
}

fn use_longest_by_string() {
    let mut v1: String = String::from("v1");
    let v2: String = String::from("v2");
    let result: Option<String> = longest_by_string(v1, v2);
    if result.is_some() {
        println!("The longest string is \"{}\"\n", result.clone().unwrap());
    }
    v1 = String::from("other v1");
    if result.is_some() {
        println!("The longest string is \"{}\"\n", result.unwrap());
    }
}

/// Return the longest string, or None if the strings have the same length.
/// Why don't we need to specify lifetimes here?
/// The function's parameters' values are "moved" from the "caller context" to the "function context".
/// Therefore, these (parameters') values are not accessible from the caller context anymore.
/// It means that:
/// (1) once passed to the function, these values cannot change (or be deleted) "outside of
///     the function".
/// (2) once the function execution terminates, the "moved" values (form string1 and string2)
///     don't "exist" anymore.
/// The returned value is the result of a "borrow".
/// But what if the (returned) value is borrowed from a value that doesn't exist anymore ?

// fn longest_by_string(string1: String, string2: String) -> Option<&String> {
//     if string1.len() > string2.len() {
//         return Some(&string1);
//     }
//     if string2.len() > string1.len() {
//         return Some(&string2);
//     }
//     return None;
// }

fn main() {
    use_longest_by_str();
    use_bigger_by_mut_u8();
    use_longest_by_string();
}
