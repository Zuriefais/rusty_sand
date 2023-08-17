pub mod lib {
    use std::collections::HashMap;

    pub struct Vec2i {
        pub x: i32,
        pub y: i32,
    }

    pub struct World {
        pub cells: HashMap<Vec2i, char>,
    }
}
