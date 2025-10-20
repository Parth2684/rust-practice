struct MyString<'a> {
    text: &'a str
}


fn main() {
    let str1;
    {
        let _str3 = String::from("Hi theree");
        // str1 = &str3;
    }
    str1 = String::from("hi there"); //correct
    let str2 = MyString{
        text: &str1
    };
    println!("{}", str2.text);
}

fn example <'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}
