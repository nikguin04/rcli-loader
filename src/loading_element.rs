use std::sync::Arc;

pub struct LoadingElement {
    max: usize,
    progress: usize,
    name: Arc<Box<str>>, // TODO: make sure that Arc is the proper use for single write multiple read
}
impl LoadingElement {
    // New functions
    pub fn new(max: usize, name: Box<str>) -> LoadingElement {
        LoadingElement { max: (max), progress: (0), name: (Arc::new(name)) }
    }
    // pub fn empty() -> LoadingElement {
    //     LoadingElement { max: (0), progress: (0), name: (Rc::new(Box::from(""))) }
    // }

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
    pub fn get_name(&self) -> Arc<Box<str>> {
        return self.name.clone();
    }

    // Setters
    pub fn set_max(&mut self, max: usize) {
        self.max = max;
    }
    pub fn update(&mut self, addition: usize) {
        self.progress += addition;
    }
}
