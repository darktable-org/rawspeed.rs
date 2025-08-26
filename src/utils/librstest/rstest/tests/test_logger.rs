use crate::logger::Logger;

#[derive(Debug, PartialEq)]
pub struct TestLogger {
    log: Vec<String>,
}

impl TestLogger {
    #[must_use]
    pub const fn new() -> Self {
        Self { log: vec![] }
    }

    #[must_use]
    pub fn log(&self) -> &[String] {
        &self.log
    }
}

impl Logger for TestLogger {
    fn write(&mut self, msg: &str) {
        self.log.push(msg.to_owned());
    }
}
