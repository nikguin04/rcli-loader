
pub struct LoadingElement {
    max: usize,
    progress: usize,
}
impl LoadingElement {
    // New functions
    pub fn new(max: usize) -> LoadingElement {
        LoadingElement { max: (max), progress: (0) }
    }
    pub fn empty() -> LoadingElement {
        LoadingElement { max: (0), progress: (0) }
    }

    // Getters
    pub fn get_progress(&self) -> usize {
        return self.progress;
    }
    pub fn get_progress_decimal(&self) -> f64 {
        return self.progress as f64 / self.max as f64;
    }
    pub fn get_max(&self) -> usize {
        return self.max;
    }

    // Setters
    pub fn set_max(&mut self, max: usize) {
        self.max = max;
    }
    pub fn update(&mut self, addition: usize) {
        self.progress += addition;
    }
}
