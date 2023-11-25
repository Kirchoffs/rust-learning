#[cfg(test)]
mod test {
    struct Point {
        x: i32,
        y: i32,
    }

    impl AsRef<Point> for Point {
        fn as_ref(&self) -> &Point {
            self
        }
    }

    #[test]
    fn desctruturing_demo() {
        let point_x = Point { x: 1, y: 2 };

        let Point { x: point_x, y: point_y } = point_x.as_ref();
        assert_eq!(point_x, &1);
        assert_eq!(point_y, &2);
    }
}
