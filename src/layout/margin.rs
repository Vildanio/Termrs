pub struct Margin {
    pub top: u16,
    pub left: u16,
    pub right: u16,
    pub bottom: u16,
}

impl Margin {
    pub fn new(top: u16, left: u16, right: u16, bottom: u16) -> Self {
        Self {
            top,
            left,
            right,
            bottom,
        }
    }
}
