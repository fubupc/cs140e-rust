use traits;
use vfat::{Cluster, Dir, File, Metadata};

// TODO: You may need to change this definition.
#[derive(Debug)]
pub enum Entry {
    File(File),
    Dir(Dir),
}

// TODO: Implement any useful helper methods on `Entry`.

impl traits::Entry for Entry {
    type File = File;

    type Dir = Dir;

    type Metadata = Metadata;

    fn name(&self) -> &str {
        match self {
            Entry::File(_) => todo!(),
            Entry::Dir(dir) => dir.long_name.as_ref().unwrap_or(&dir.short_name),
        }
    }

    fn metadata(&self) -> &Self::Metadata {
        match self {
            Entry::File(_) => todo!(),
            Entry::Dir(dir) => &dir.metadata,
        }
    }

    fn as_file(&self) -> Option<&File> {
        match self {
            Entry::File(_) => todo!(),
            Entry::Dir(_) => None,
        }
    }

    fn as_dir(&self) -> Option<&Dir> {
        match self {
            Entry::File(_) => todo!(),
            Entry::Dir(dir) => Some(dir),
        }
    }

    fn into_file(self) -> Option<File> {
        match self {
            Entry::File(_) => todo!(),
            Entry::Dir(_) => None,
        }
    }

    fn into_dir(self) -> Option<Dir> {
        match self {
            Entry::File(_) => todo!(),
            Entry::Dir(dir) => Some(dir),
        }
    }
}
