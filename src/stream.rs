pub struct ElementStream<T> {
    elements: Vec<T>,
    pub index: usize
}

impl<T: Clone> ElementStream<T> {
    pub fn new(elements: Vec<T>) -> Self {
        Self {
            elements,
            index: 0
        }
    }

    // Peeks in to the next optional element (current index).
    pub fn peek(&self) -> Option<T> {
        self.elements.get(self.index).cloned()
    }

    // Grabs the next optional element (current index), advances the counter, and returns that element.
    pub fn consume(&mut self) -> Option<T> {
        let element = self.peek();

        if self.index + 1 > self.elements.len() {
            return None;
        }

        self.index += 1;
        element
    }
}
