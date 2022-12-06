
fn main() {
    let mut input = String::new();
    std::io::stdin().read_line( &mut input ).unwrap();
    input = pig_latin::word(input.trim());
    pig_latin::pig_latin(&mut input);
    println!("{}",input)

}
