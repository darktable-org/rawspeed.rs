use core::cell::RefCell;
use std::rc::Rc;

struct LoggingIter {
    log: Rc<RefCell<Vec<String>>>,
    pos: usize,
    end: usize,
}

impl LoggingIter {
    fn new(
        log: Rc<RefCell<Vec<String>>>,
        core::ops::Range { start, end }: core::ops::Range<usize>,
    ) -> Self {
        log.borrow_mut().push(format!("Iter created, at {start}"));

        Self {
            log,
            pos: start,
            end,
        }
    }
}

#[allow(clippy::missing_trait_methods)]
impl Iterator for LoggingIter {
    type Item = IterVal;

    fn next(&mut self) -> Option<Self::Item> {
        self.log
            .borrow_mut()
            .push(format!("Iter next() called at pos = {}", self.pos));

        if self.pos >= self.end {
            self.log.borrow_mut().push(format!(
                "Iter next() called at pos = {}, returning None",
                self.pos
            ));
            return None;
        }

        let current = self.pos;
        let next = self.pos + 1;
        self.log.borrow_mut().push(format!(
            "Iter next() called at pos = {}, returning {}, next is {}",
            self.pos, current, next
        ));
        self.pos = next;
        Some(IterVal::new(Rc::clone(&self.log), current))
    }
}

impl Drop for LoggingIter {
    fn drop(&mut self) {
        self.log
            .borrow_mut()
            .push(format!("Iter dropped, was at {}", self.pos));
    }
}

struct IterVal {
    log: Rc<RefCell<Vec<String>>>,
    val: usize,
}

impl IterVal {
    fn new(log: Rc<RefCell<Vec<String>>>, val: usize) -> Self {
        log.borrow_mut().push(format!("IterVal({val}) created"));
        Self { log, val }
    }
}

impl core::ops::Deref for IterVal {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        self.log
            .borrow_mut()
            .push(format!("IterVal({}) deref", self.val));
        &self.val
    }
}

impl Drop for IterVal {
    fn drop(&mut self) {
        self.log
            .borrow_mut()
            .push(format!("IterVal({}) dropped", self.val));
    }
}

fn gen_native_output(r: core::ops::Range<usize>) -> Vec<String> {
    let mut vec = vec![];
    vec.push("Before macro".to_owned());
    vec.push(format!("Iter created, at {}", r.start).to_owned());
    for i in r.clone() {
        vec.push(format!("Iter next() called at pos = {i}"));
        vec.push(format!(
            "Iter next() called at pos = {i}, returning {i}, next is {}",
            i + 1
        ));
        vec.push(format!("IterVal({i}) created"));
        vec.push(format!("IterVal({i}) deref"));
        vec.push(format!("Loop body at i = {i}"));
        vec.push(format!("IterVal({i}) dropped"));
    }
    vec.push(format!("Iter next() called at pos = {}", r.end));
    vec.push(format!(
        "Iter next() called at pos = {}, returning None",
        r.end,
    ));
    vec.push(format!("Iter dropped, was at {}", r.end).to_owned());
    vec.push("After loop".to_owned());
    vec.push("After macro".to_owned());
    vec
}

#[cfg(test)]
#[allow(clippy::large_stack_frames)]
mod naive;

#[cfg(test)]
#[allow(clippy::large_stack_frames)]
mod unroll_runtime;

#[cfg(test)]
#[allow(clippy::large_stack_frames)]
mod unroll_with_remainder;
