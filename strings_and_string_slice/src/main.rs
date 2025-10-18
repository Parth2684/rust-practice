fn main() {
// strings and &str
    // string
     let mut name = String::from("Parth");
     println!("{}", name);
     let lang = "Rust".to_string();
     println!("{}", lang);

     name.insert_str(5, " Bhosle");
     println!("{}", name);

     let new_name = name.replace("Parth", "Dev");
     println!("{}", new_name);

    //  &str string slice
    let str1 = "Hello";
    let str2 = str1.to_string();
    let str3 = &str2;
    println!("{}", str1);
    println!("{}", str2);
    println!("{}", str3);

    let rust = "\x52\x75\x73\x74";
    println!("{}", rust);
}
