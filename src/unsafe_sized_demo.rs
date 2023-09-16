#[cfg(test)]
mod test {
    struct Bar<T: ?Sized>(T);
    #[repr(transparent)]
    struct BarUse(Bar<[i32]>);

    #[test]
    fn unsafe_sized_demo() {
        let bar: Box<Bar<[i32]>> = Box::new(Bar([1, 2, 3]));
        let bar_use: Box<BarUse> = unsafe { Box::from_raw(Box::into_raw(bar) as *mut BarUse) };
    }
}