use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryType {
    File,
    Directory,
    Symlink,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryEntry {
    pub name: String,
    pub path: PathBuf,
    pub entry_type: EntryType,
}

pub fn create_directory(path: &Path) -> io::Result<()> {
    fs::create_dir(path)
}

pub fn create_directory_all(path: &Path) -> io::Result<()> {
    fs::create_dir_all(path)
}

pub fn create_file(path: &Path) -> io::Result<()> {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?
        .write_all(b"")
}

pub fn read_directory(path: &Path) -> io::Result<Vec<DirectoryEntry>> {
    fs::read_dir(path)?
        .map(|entry| {
            let entry = entry?;
            let entry_type = classify_entry_type(entry.file_type()?);

            Ok(DirectoryEntry {
                name: entry.file_name().to_string_lossy().into_owned(),
                path: entry.path(),
                entry_type,
            })
        })
        .collect()
}

pub fn read_text_file(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn write_text_file(path: &Path, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

pub fn rename(source: &Path, destination: &Path) -> io::Result<()> {
    fs::rename(source, destination)
}

pub fn remove_directory_all(path: &Path) -> io::Result<()> {
    fs::remove_dir_all(path)
}

pub fn move_to_trash(path: &Path) -> io::Result<()> {
    trash::delete(path).map_err(io::Error::other)
}

pub fn entry_type(path: &Path) -> io::Result<EntryType> {
    let file_type = fs::symlink_metadata(path)?.file_type();
    Ok(classify_entry_type(file_type))
}

pub fn path_exists(path: &Path) -> io::Result<bool> {
    path.try_exists()
}

pub fn canonicalize(path: &Path) -> io::Result<PathBuf> {
    path.canonicalize()
}

fn classify_entry_type(file_type: fs::FileType) -> EntryType {
    if file_type.is_symlink() {
        EntryType::Symlink
    } else if file_type.is_dir() {
        EntryType::Directory
    } else if file_type.is_file() {
        EntryType::File
    } else {
        EntryType::Other
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn reads_directory_entries_and_text_files() {
        let temp = TempDir::new().unwrap();
        let folder = temp.path().join("folder");
        let file = temp.path().join("diagram.mmd");
        create_directory(&folder).unwrap();
        write_text_file(&file, "flowchart LR").unwrap();

        let mut entries = read_directory(temp.path()).unwrap();
        entries.sort_by(|left, right| left.name.cmp(&right.name));

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].name, "diagram.mmd");
        assert_eq!(entries[0].entry_type, EntryType::File);
        assert_eq!(entries[1].name, "folder");
        assert_eq!(entries[1].entry_type, EntryType::Directory);
        assert_eq!(read_text_file(&file).unwrap(), "flowchart LR");
    }

    #[test]
    fn supports_file_and_directory_crud() {
        let temp = TempDir::new().unwrap();
        let nested = temp.path().join("one/two");
        let source = nested.join("source.mmd");
        let destination = nested.join("destination.mmd");

        create_directory_all(&nested).unwrap();
        create_file(&source).unwrap();
        assert_eq!(entry_type(&source).unwrap(), EntryType::File);
        assert!(path_exists(&source).unwrap());
        assert!(create_file(&source).is_err());

        write_text_file(&source, "sequenceDiagram").unwrap();
        rename(&source, &destination).unwrap();
        assert!(!path_exists(&source).unwrap());
        assert_eq!(read_text_file(&destination).unwrap(), "sequenceDiagram");
        assert!(canonicalize(&destination).unwrap().is_absolute());

        fs::remove_file(&destination).unwrap();
        remove_directory_all(&temp.path().join("one")).unwrap();
        assert!(!path_exists(&nested).unwrap());
    }

    #[cfg(unix)]
    #[test]
    fn reports_symbolic_links_without_following_them() {
        use std::os::unix::fs::symlink;

        let temp = TempDir::new().unwrap();
        let target = temp.path().join("target.mmd");
        write_text_file(&target, "flowchart LR").unwrap();
        symlink(&target, temp.path().join("link.mmd")).unwrap();

        let link = read_directory(temp.path())
            .unwrap()
            .into_iter()
            .find(|entry| entry.name == "link.mmd")
            .unwrap();

        assert_eq!(link.entry_type, EntryType::Symlink);
        assert_eq!(
            entry_type(&temp.path().join("link.mmd")).unwrap(),
            EntryType::Symlink
        );
    }
}
