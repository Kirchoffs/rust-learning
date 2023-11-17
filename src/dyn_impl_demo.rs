trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f32,
}

struct Square {
    side: f32,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius: {}", self.radius);
    }
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a square with side: {}", self.side);
    }
}

fn draw_shape_dyn(shape: Box<dyn Drawable>) {
    shape.draw();
}

fn draw_shape_impl(shape: impl Drawable) {
    shape.draw();
}

#[cfg(test)]
mod test {
    use super::{Circle, Square, draw_shape_dyn, draw_shape_impl};

    #[test]
    fn dyn_impl_demo() {
        // Using dyn
        println!("Using dyn");
        let circle_pointer = Box::new(Circle { radius: 4.0 });
        let square_pointer = Box::new(Square { side: 2.0 });
        draw_shape_dyn(circle_pointer);
        draw_shape_dyn(square_pointer);

        // Using impl
        println!("Using impl");
        let circle = Circle { radius: 4.0 };
        let square = Square { side: 2.0 };
        draw_shape_impl(circle);
        draw_shape_impl(square);
    }
}
