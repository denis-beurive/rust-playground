use std::fmt;

/// Define a structure that implements the "Display" trait (so it can be directly represented
/// as text).
///
/// For every lifetime "'a", define MyString to contain a reference "value": &'a str.
/// So MyString is "generic" and can store a reference with any lifetime.
/// Ref: https://stackoverflow.com/questions/39355984/what-does-the-first-explicit-lifetime-specifier-on-an-impl-mean

struct MyString<'a> {
    name: String,
    value: &'a str
}

/// For every lifetime "'a", define methods for the type MyString<'a>.
/// Rel: https://stackoverflow.com/questions/39355984/what-does-the-first-explicit-lifetime-specifier-on-an-impl-mean

impl<'a> fmt::Display for MyString<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut str = "";
        fmt.write_str(self.name.as_str())?;
        fmt.write_str(":")?;
        fmt.write_str(self.value)?;
        Ok(())
    }
}

/// Return an instance of MyString that identified the longest string, or None if the strings have the same length.
///
/// The returned value may depend on one of the parameters' values passed as parameters to the function.
///
/// - If the longest string is the one referenced by "string1", then the returned value depends on
///   the value referenced by "string1". Thus, the returned value is "valid" as long as the value
///   referenced by "string1" does not change.
///   <=> The lifetime of the returned value depends on the lifetimes of the value referenced by
///   "string1".
/// - If the longest string is the one referenced by "string2", then the returned value depends on
///   the value referenced by "string2". Thus, the returned value is "valid" as long as the value
///   referenced by "string2" does not change.
///   <=> The lifetime of the returned value depends on the lifetimes of the value referenced by
///   "string2".
/// - If the returned value is None, then, obviously, it does not depend on the values referenced by
///   the references passed as parameters to the function.
///   <=> The lifetime of the returned value does not depend on the lifetimes of the values
///   referenced by the references passed as parameters to the function.
///
/// But, at compile time, it is not possible to predict whether one string is longer than the other,
/// and, in this case, which string is the longest. All we can say is that the returned value may
/// depend on one of the parameters' values passed as parameters to the function.
/// <=> All we can say is that the lifetime of the returned value may depend on the lifetimes of
/// the parameters' values passed as parameters to the function.
///
/// Please note that "string1" and "string2" are not mutable references. Thus, they reference
/// immutable variables (which values won't change in the future). This means that the value of
/// the field "value" (of the returned reference) is not likely to change.
/// <=> The IMMUTABLE returned value may depend on the IMMUTABLE values referenced by the function's
/// parameters.

fn longest_by_str<'a>(string1: &'a str, string2: &'a str) -> Option<MyString<'a>> {
    if string1.len() > string2.len() {
        return Some(MyString { name: String::from("string1"), value: string1 });
    }
    if string2.len() > string1.len() {
        return Some(MyString { name: String::from("string2"), value: string2 });
    }
    return None;
}

fn use_longest_by_str() {
    let str1: &str = "This is a longer string";
    let str2: &str = "This is a short string";
    let longest: Option<MyString> = longest_by_str(str1, str2);
    println!("str1: {}\nstr2: {}", str1, str2);
    // The line below creates a new variable "str1".
    let str1: &str = "new str1";
    println!("str1: {}\nstr2: {}", str1, str2);
    if longest.is_some() {
        println!("The longest string is \"{}\"", longest.unwrap());
    }
}

/// As well as for the function "longest_by_str", the value referenced by the returned reference
/// may depend on the values references by the function's parameters.
///
/// However, in contrast to the function "longest_by_str", in this case, all the references
/// involved are mutable.
///
/// => The MUTABLE value referenced by the returned reference may depend on the MUTABLE values
/// referenced by the function's parameters.
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
///
/// Please note:
/// - The function's parameters' values are "moved" from the "caller context" to the "function
///   context". Therefore, these (parameters') values are not accessible from the caller context
///   anymore. It means that, once passed to the function, these values cannot change "outside of
///   the function" (and neither can they change within the function because they are declared as
///   immutable).
/// - The returned value is the result of a "move".

fn longest_by_string(string1: String, string2: String) -> Option<String> {
    if string1.len() > string2.len() {
        return Some(string1); // move
    }
    if string2.len() > string1.len() {
        return Some(string2); // move
    }
    return None;
}

fn use_longest_by_string() {
    let mut v1: String = String::from("v1");
    let v2: String = String::from("v2");
    let result: Option<String> = longest_by_string(v1, v2); // v1 and v2 move
    // Below this line you cannot access the values of "v1" and "v2" since these values were
    // moved to the function's context.
    if result.is_some() {
        println!("The longest string is \"{}\"\n", result.clone().unwrap());
    }
    // The value of "v1" can change (since "v1" is mutable). However, the value (of "v1") that was
    // moved to the function is not affected by the following assignment.
    v1 = String::from("other v1");
    if result.is_some() {
        println!("The longest string is \"{}\"\n", result.unwrap());
    }
}

fn main() {
    use_longest_by_str();
    use_bigger_by_mut_u8();
    use_longest_by_string();
}
