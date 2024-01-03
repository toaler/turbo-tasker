use std::path::Path;

pub trait Visitable {
    fn visit(&mut self, path: &Path);
}