#[cfg(test)]
mod test {
    #[test]
    fn array_demo_hitotsu() {
        let array: [i32; 42] = [42; 42];
        println!("array: {:?}", array);
    }

    #[test]
    fn array_demo_futatsu() {
        let array: [i32; 5] = [1, 2, 3, 4, 5];
        let sliced_array = &array[2..];
        println!("sliced_array: {:?}", sliced_array);
    }
}
