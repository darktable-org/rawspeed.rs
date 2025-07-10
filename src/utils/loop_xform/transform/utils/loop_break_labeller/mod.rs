use syn::visit_mut::VisitMut;

pub struct LabelUnlabelledBreaks {
    loop_label: syn::Lifetime,
}

impl LabelUnlabelledBreaks {
    #[allow(clippy::single_call_fn)]
    pub fn visit(i: &mut syn::ExprForLoop) {
        let mut this = Self {
            loop_label: i.label.as_ref().unwrap().name.clone(),
        };
        for stmt in &mut i.body.stmts {
            this.visit_stmt_mut(stmt);
        }
    }
}

#[allow(clippy::missing_trait_methods)]
impl VisitMut for LabelUnlabelledBreaks {
    fn visit_expr_loop_mut(&mut self, _i: &mut syn::ExprLoop) {}
    fn visit_expr_while_mut(&mut self, _i: &mut syn::ExprWhile) {}
    fn visit_expr_for_loop_mut(&mut self, _i: &mut syn::ExprForLoop) {}

    fn visit_expr_break_mut(&mut self, i: &mut syn::ExprBreak) {
        if i.label.is_none() {
            i.label = Some(self.loop_label.clone());
        }
    }
}

#[cfg(test)]
#[allow(clippy::large_stack_frames)]
mod tests;
