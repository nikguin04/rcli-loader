
pub struct LoadingElement {
    max: i32,
    progress: i32,
}
impl LoadingElement {
    pub fn new(max: i32) -> LoadingElement {
        LoadingElement { max: (max), progress: (0) }
    }
    pub fn get_progress(&self) -> i32 {
        return self.progress;
    }
    pub fn get_max(&self) -> i32 {
        return self.max;
    }
    pub fn update(&mut self, addition: i32) {
        self.progress += addition
    }
}
