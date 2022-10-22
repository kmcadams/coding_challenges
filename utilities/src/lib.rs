//implement generic funtion to print any string type in Rust
pub fn print_any<'a, T: AsRef<str>>(item: &T){
    println!("{}", item.as_ref());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_input_as_string(){
    let input = String::from("This string should be printable");
    print_any(&input);
    }

    #[test]
    fn ut_input_as_str(){
    let input = "This string should be printable";
    print_any(&input);
    
    }
}
