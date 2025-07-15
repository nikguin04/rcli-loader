
pub struct LoadingElement {
    max: usize,
    progress: usize,
}
impl LoadingElement {
    pub fn new(max: usize) -> LoadingElement {
        LoadingElement { max: (max), progress: (0) }
    }
    pub fn get_progress(&self) -> usize {
        return self.progress;
    }
    pub fn get_progress_decimal(&self) -> f64 {
        return self.progress as f64 / self.max as f64;
    }
    pub fn get_max(&self) -> usize {
        return self.max;
    }
    pub fn update(&mut self, addition: usize) {
        self.progress += addition
    }
}
