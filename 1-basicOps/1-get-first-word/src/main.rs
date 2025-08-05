fn main() {
    let greeting = String::from("nigga this is rust");
    println!("{}", greeting);
    let first_word = get_first_word(greeting);
    println!("{}", first_word)
}

fn get_first_word (sentence: String) -> String {
    let mut ans = String::from("");
    for char in sentence.chars() {
        if char == ' ' {
            break;
        }
        ans.push_str(char.to_string().as_str());
    }
    return ans
}