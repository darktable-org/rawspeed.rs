use super::LoopUnrollConf;
use super::UnrollMethod;

pub fn perform_loop_unroll(c: LoopUnrollConf) -> proc_macro2::TokenStream {
    match c.unroll_method {
        UnrollMethod::Runtime => unroll_runtime::transform(&c),
        UnrollMethod::WithRemainder => unroll_with_remainder::transform(c),
    }
}

mod utils {
    pub mod loop_break_labeller;
}

mod unroll_runtime;
mod unroll_with_remainder;
