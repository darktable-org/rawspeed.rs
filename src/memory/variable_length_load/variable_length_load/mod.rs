pub trait VariableLengthLoad {
    fn variable_length_load(&mut self, src: &[u8], src_pos: usize);
}

impl VariableLengthLoad for [u8] {
    fn variable_length_load(&mut self, src: &[u8], src_pos: usize) {
        self.fill(0);

        let src_pos = std::cmp::min(src_pos, src.len());
        let src = &src[src_pos..];
        let copy_size = std::cmp::min(self.len(), src.len());
        self[..copy_size].copy_from_slice(&src[..copy_size]);
    }
}

#[cfg(test)]
#[allow(clippy::large_stack_frames)]
mod test;
