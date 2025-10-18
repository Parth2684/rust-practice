fn main() {
    println!("Hello, world!");
    let ans;
    let str1 = String::from("Short");
    {
        let str2 = String::from("longer");
        ans = find_longest_str(&str1, &str2);
        println!("{}", ans);
    }
    // println!("{}", ans); throws error befcause lifetime of ans is over
    
}

fn find_longest_str<'a> (s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    }
    else {
        s2
    }
}