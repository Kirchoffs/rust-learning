#[cfg(test)]
mod test {
    use std::convert::TryInto;

    struct NonZeroPoint {
        x: i32,
        y: i32,
    }

    impl TryInto<String> for NonZeroPoint {
        type Error = String;
        fn try_into(self) -> Result<String, Self::Error> {
            if self.x != 0 && self.y != 0 {
                return Ok(format!("NonZeroPoint(x: {}, y: {})", self.x, self.y));
            }
            Err("invalid point".to_string())
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl Into<String> for Point {
        fn into(self) -> String {
            format!("Point(x: {}, y: {})", self.x, self.y)
        }
    }

    #[test]
    fn into_demo() {
        let non_zero_point = NonZeroPoint { x: 1, y: 2 };
        let non_zero_point_str: String = non_zero_point.try_into().unwrap();
        println!("non_zero_point_str: {}", non_zero_point_str);

        let point = Point { x: 0, y: 2 };
        let point_str: String = point.into();
        println!("point_str: {}", point_str);
    }
}
