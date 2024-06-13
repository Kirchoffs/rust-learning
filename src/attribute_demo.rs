#[cfg(test)]
mod test {
    use customized_attribute::print_message;

    #[print_message]
    fn print_with_attribute_macro() {
        println!("This is a print_with_attribute_macro message");
    }

    fn print_without_attribute_macro() {
        println!("This is a print_without_attribute_macro message");
    }
    
    #[test]
    fn test_attribute_demo() {
        print_with_attribute_macro();
        print_without_attribute_macro();
    }
}
