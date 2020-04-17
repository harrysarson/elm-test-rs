use std::mem;
use std::path::Path;
use std::path::PathBuf;
use tempdir::TempDir;

pub enum WorkDir<P> {
    Tempory(TempDir),
    Provided(P),
}

impl<P: AsRef<Path>> AsRef<Path> for WorkDir<P> {
    #[inline]
    fn as_ref(&self) -> &Path {
        match self {
            Self::Tempory(td) => td.as_ref(),
            Self::Provided(p) => p.as_ref(),
        }
    }
}

impl<P: From<PathBuf>> WorkDir<P> {
    pub fn persist(&mut self) {
        if let Self::Tempory(_) = self {
            match mem::replace(self, Self::Provided(PathBuf::new().into())) {
                Self::Tempory(td) => *self = Self::Provided(td.into_path().into()),
                _ => {
                    panic!("impossible state");
                }
            }
        }
    }
}
