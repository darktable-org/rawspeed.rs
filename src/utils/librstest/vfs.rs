pub trait VFS {
    fn exists(&self, path: &str) -> std::io::Result<bool>;

    fn remove_file(&mut self, path: &str) -> std::io::Result<()>;

    fn write(&mut self, path: &str, contents: &[u8]) -> std::io::Result<()>;

    fn read(&self, path: &str) -> std::io::Result<Vec<u8>>;
}

#[derive(Debug)]
#[non_exhaustive]
pub struct NativeFileSystem;

impl NativeFileSystem {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl Default for NativeFileSystem {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl VFS for NativeFileSystem {
    #[inline]
    fn exists(&self, path: &str) -> std::io::Result<bool> {
        std::fs::exists(path)
    }

    #[inline]
    fn remove_file(&mut self, path: &str) -> std::io::Result<()> {
        std::fs::remove_file(path)
    }

    #[inline]
    fn write(&mut self, path: &str, contents: &[u8]) -> std::io::Result<()> {
        std::fs::write(path, contents)
    }

    #[inline]
    fn read(&self, path: &str) -> std::io::Result<Vec<u8>> {
        std::fs::read(path)
    }
}
