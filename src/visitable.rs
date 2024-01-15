use std::fs::Metadata;
use std::path::Path;
use crate::cached_metadata::CachedMetadata;

pub trait Visitable {
    fn visit(&mut self, metadata: &mut CachedMetadata);

    fn recap(&mut self);
}