#[cfg(test)]
mod test {
    struct Foo<T>(T);
    struct FooWrapper(Foo<i32>);
    struct Bar<T: ?Sized>(T);
    struct BarWrapper(Bar<[i32]>);
    struct BoxBarWrapper(Box<Bar<[i32]>>);


    #[test]
    fn sized_demo_hitotsu() {
        let foo = Foo(42);
        let foo_wrapper = FooWrapper(foo);

        let bar = Bar([1, 2, 3]);
        let box_bar_wrapper = BoxBarWrapper(Box::new(bar));
    }

    #[test]
    fn sized_demo_futatsu() {
        // Not work to directly instantiate an array.
        // let arr: [i32] = [1, 2, 3];
        let arr_pointer: Box<[i32]> = Box::new([42]);
    }
}