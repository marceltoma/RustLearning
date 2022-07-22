fn main() {
    println!("Hello, world!");
    let s1: String = String::from("teste");
    let len = string_size(&s1);
    
    println!("size: {len}");
}

fn string_size(s: &String) -> usize {
    s.len()
}
