use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq)]
pub struct SourceMeta {
    pub filepath: PathBuf,
    pub lines: (usize, usize),
}

