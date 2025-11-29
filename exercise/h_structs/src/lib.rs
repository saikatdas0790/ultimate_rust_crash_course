pub struct Polygon {
    name: String,
    sides: u32,
    visible: bool,
}

impl Polygon {
    pub fn new(name: String) -> Self {
        Self {
            name,
            sides: 3,
            visible: true,
        }
    }

    pub fn shape(&self) -> String {
        if self.sides == 3 {
            "triangle".to_string()
        } else if self.sides == 4 {
            "square".to_string()
        } else if self.sides == 5 {
            "pentagon".to_string()
        } else {
            "polygon".to_string()
        }
    }

    pub fn increment_sides(&mut self) {
        self.sides += 1;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn sides(&self) -> u32 {
        self.sides
    }

    pub fn visible(&self) -> bool {
        self.visible
    }
}
