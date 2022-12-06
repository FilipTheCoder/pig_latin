

pub fn pig_latin(word: &mut String) {
    if word.is_empty() {
        println!("No input given");
        return ()
    }
    let first = word.chars().nth(0).unwrap();
    word.remove(0);
    word.push_str(&format!("-{first}ay"))

}

pub fn word(w: &str) -> String {
    for i in w.chars() {
        if i.eq(&' ') {
            return String::from("default")
        }
    }
    return w.to_string()
    
}