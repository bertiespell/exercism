pub fn raindrops(n: u32) -> String {
    let mut result = String::from("");
    if n % 3 == 0 {
        result = result + &String::from("Pling");
    }
    if n % 5 == 0 {
        result = result + &String::from("Plang");
    }
    if n % 7 == 0 {
        result = result + &String::from("Plong");
    }
    if result.len() != 0 {
        return result;
    } else {
        return n.to_string();
    }
}
