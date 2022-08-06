use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct SourceMeta {
    pub filepath: PathBuf,
    pub lines: (usize, usize),
}

