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

    #[test]
    fn print_array_demo() {
        fn print_generic_reference_array<T, const N: usize>(array: &[T; N]) where T: std::fmt::Debug {
            println!("Array: {:?}", array);
        }

        fn print_reference_array_i32(array: &[i32; 5]) {
            println!("Array: {:?}", array);
        }

        fn print_array_i32(array: [i32; 5]) {
            println!("Array: {:?}", array);
        }

        fn print_generic_slice<T>(slice: &[T]) where T: std::fmt::Debug {
            println!("Slice: {:?}", slice);
        }

        fn print_slice_i32(slice: &[i32]) {
            println!("Slice: {:?}", slice);
        }

        let array: [i32; 5] = [1, 2, 3, 4, 5];
        print_generic_reference_array(&array);
        print_reference_array_i32(&array);
        print_array_i32(array);

        let slice = &array[2..];
        print_generic_slice(slice);
        print_slice_i32(slice);
    }
}
