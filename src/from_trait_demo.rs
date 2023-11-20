#[cfg(test)]
mod test {
    #[derive(Debug)]
    struct StringStruct {
        content: String,
    }

    impl From<String> for StringStruct {
        fn from(content: String) -> Self {
            StringStruct { content }
        }
    }

    impl From<StringStruct> for String {
        fn from(wrapper: StringStruct) -> Self {
            wrapper.content
        }
    }

    #[test]
    fn from_trait_demo() {
        let hello_string = String::from("hello");
        let hello_string_struct = StringStruct::from(hello_string);
        println!("hello_string_struct: {:?}", hello_string_struct);

        let hello_string = String::from(hello_string_struct);
        println!("hello_string: {:?}", hello_string);
    }
}