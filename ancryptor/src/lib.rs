use base64::{
    Engine as _,
    engine::general_purpose::STANDARD as base64Engine
};

pub fn encode(to: &str) -> String {
    base64Engine.encode(String::from(to))
}

pub fn decode(from: &str) -> String {
    let base64_bytes = base64Engine.decode(
        String::from(from)
    ).unwrap_or(vec![]);

    match String::from_utf8(base64_bytes) {
        Ok(result) => result,
        Err(_) => "".to_owned()
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


enum BoxColor{
    Red,
    Blue,
    Black
}

struct Box {
    dimentions: f64,
    weight: f64,
    color: BoxColor
}

impl Box{
    fn create_new_box() -> Self {
        Self { dimentions: 4.5, weight: 5.0, color: BoxColor::Black }
    }
}

impl BoxColor{
    fn print(&self) {
        match self {
            BoxColor::Black => println!("Black"),
            BoxColor::Red => println!("Red"),
            BoxColor::Blue => println!("Blue"),
        }
    }
}

fn main(){

    let boxChar = Box::create_new_box();
    println!("Box dimension is {:?}- weight is {:?}- color is {:?}", boxChar.dimentions, boxChar.weight, boxChar.color.print())
}