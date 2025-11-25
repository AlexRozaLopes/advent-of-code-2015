pub struct Rectangle {
    length: i32,
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn new(length: i32, width: i32, height: i32) -> Self {
        Self {
            length,
            width,
            height,
        }
    }

    pub fn surface_area(&self) -> i32 {
        2 * self.length * self.width + 2 * self.width * self.height + 2 * self.height * self.length
    }

    pub fn shortest_area(&self) -> i32 {
        let mut arr = [self.length, self.width, self.height];
        arr.sort();
        arr[0] * arr[1]
    }
}
