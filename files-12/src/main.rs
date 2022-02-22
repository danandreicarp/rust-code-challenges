use std::fs::{File, OpenOptions};
use std::io::ErrorKind;
use std::path;

trait FileMetadata {
    fn exists(&self) -> bool;

    fn is_writeable(&self) -> bool;

    fn is_readable(&self) -> bool;
}

impl FileMetadata for path::Path {
    fn is_readable(&self) -> bool {
        // match OpenOptions::new().read(true).open(self) {
        //     Ok(_) => true,
        //     Err(e) => match e.kind() {
        //         ErrorKind::PermissionDenied => {
        //             println!("PermissionDenied! {}", e);
        //             false
        //         }
        //         _ => {
        //             println!("Other Error: {}", e);
        //             true
        //         }
        //     },
        // }

        File::open(&self).is_ok()
    }

    fn is_writeable(&self) -> bool {
        // let metadata = self.metadata().unwrap();
        // let permissions = metadata.permissions();

        // !permissions.readonly()

        // if let Ok(metadata) = self.metadata() {
        //     !metadata.permissions().readonly()
        // } else {
        //     false
        // }

        self.metadata()
            .map(|metadata| !metadata.permissions().readonly())
            .unwrap_or(false)
    }

    fn exists(&self) -> bool {
        // let metadata = self.metadata().unwrap();
        // metadata.is_file()

        self.exists()
    }
}

fn main() {
    //
}

#[test]
fn exists() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    assert!(f.path().exists());

    fs::remove_file(f.path()).unwrap();
}

#[test]
fn writeable() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    assert!(f.path().is_writeable());

    fs::remove_file(f.path()).unwrap();
}

#[test]
fn read_only() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    let mut perms = fs::metadata(f.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(f.path(), perms).unwrap();
    assert_eq!(f.path().is_writeable(), false);

    perms = fs::metadata(f.path()).unwrap().permissions();
    perms.set_readonly(false);
    fs::set_permissions(f.path(), perms).unwrap();
    fs::remove_file(f.path()).unwrap();
}

#[test]
fn readable() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    let mut perms = fs::metadata(f.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(f.path(), perms).unwrap();
    assert!(f.path().is_readable());

    perms = fs::metadata(f.path()).unwrap().permissions();
    perms.set_readonly(false);
    fs::set_permissions(f.path(), perms).unwrap();
    fs::remove_file(f.path()).unwrap();
}
