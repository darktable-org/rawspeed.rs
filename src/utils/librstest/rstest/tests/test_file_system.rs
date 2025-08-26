use crate::vfs::VFS;

#[derive(Debug, PartialEq)]
pub struct VirtualFile {
    path: String,
    content: Vec<u8>,
}

impl VirtualFile {
    #[must_use]
    pub const fn new(path: String, content: Vec<u8>) -> Self {
        Self { path, content }
    }
}

#[derive(Debug, PartialEq)]
pub struct TestFileSystem {
    log: Vec<String>,
    files: Vec<VirtualFile>,
}

impl TestFileSystem {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            log: vec![],
            files: vec![],
        }
    }

    #[must_use]
    pub fn list_files(&self) -> Vec<&str> {
        let mut paths = vec![];
        for vfile in &self.files {
            paths.push(vfile.path.as_str());
        }
        paths
    }

    #[must_use]
    pub fn log(&self) -> &[String] {
        &self.log
    }

    pub fn clear_log(&mut self) {
        self.log.clear();
    }
}

impl VFS for TestFileSystem {
    #[inline]
    fn exists(&self, path: &str) -> std::io::Result<bool> {
        Ok(self.files.iter().any(|f| path == f.path))
    }

    #[inline]
    fn remove_file(&mut self, path: &str) -> std::io::Result<()> {
        self.log.push(format!("removing '{path}'"));
        if !self.exists(path)? {
            return Err(std::io::Error::other("No such file or directory"));
        }
        self.files.retain(|f| path != f.path);
        Ok(())
    }

    #[inline]
    fn write(&mut self, path: &str, contents: &[u8]) -> std::io::Result<()> {
        if self.exists(path)? {
            self.remove_file(path)?;
        }
        self.log.push(format!("writing '{path}'"));
        self.files
            .push(VirtualFile::new(path.to_owned(), contents.to_vec()));
        Ok(())
    }

    #[inline]
    fn read(&self, path: &str) -> std::io::Result<Vec<u8>> {
        let Some(file) = self.files.iter().find(|f| path == f.path) else {
            return Err(std::io::Error::other("No such file or directory"));
        };
        Ok(file.content.clone())
    }
}
