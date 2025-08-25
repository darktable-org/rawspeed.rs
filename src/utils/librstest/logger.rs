pub trait Logger {
    fn write(&mut self, msg: &str);
}

#[derive(Debug)]
#[non_exhaustive]
pub struct StdoutLogger;

impl StdoutLogger {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl Default for StdoutLogger {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Logger for StdoutLogger {
    #[expect(clippy::print_stdout)]
    #[inline]
    fn write(&mut self, msg: &str) {
        println!("{msg}");
    }
}
