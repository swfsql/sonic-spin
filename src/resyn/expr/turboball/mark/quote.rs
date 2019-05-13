use super::ExprMark;

#[cfg(feature = "printing")]
impl quote::ToTokens for ExprMark {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            ExprMark::Box(mark_box) => mark_box.box_token.to_tokens(tokens),
            // ExprMark::InPlace(mark::InPlace),
            ExprMark::Unary(mark_unary) => 
                mark_unary.op.to_tokens(tokens),
            ExprMark::Let(mark_let) => {
                mark_let.let_token.to_tokens(tokens);
                mark_let.pats.to_tokens(tokens);
                mark_let.eq_token.to_tokens(tokens);
            },
            ExprMark::If(mark_if) => 
                mark_if.if_token.to_tokens(tokens),
            ExprMark::While(mark_while) => {
                mark_while.label.to_tokens(tokens);
                mark_while.while_token.to_tokens(tokens);
            },
            ExprMark::ForLoop(mark_for_loop) => {
                mark_for_loop.label.to_tokens(tokens);
                mark_for_loop.for_token.to_tokens(tokens);
                mark_for_loop.pat.to_tokens(tokens);
                mark_for_loop.in_token.to_tokens(tokens);
            },
            ExprMark::Loop(mark_loop) => {
                mark_loop.label.to_tokens(tokens);
                mark_loop.loop_token.to_tokens(tokens);
            },
            ExprMark::Match(mark_match) => 
                mark_match.match_token.to_tokens(tokens),
            ExprMark::Unsafe(mark_unsafe) => 
                mark_unsafe.unsafe_token.to_tokens(tokens),
            ExprMark::Block(mark_block) => 
                mark_block.label.to_tokens(tokens),
            // ExprMark::Assign(mark::Assign),
            // ExprMark::AssignOp(mark::AssignOp),
            ExprMark::Reference(mark_reference) => {
                mark_reference.and_token.to_tokens(tokens);
                mark_reference.mutability.to_tokens(tokens);
            },
            ExprMark::Break(mark_break) => {
                mark_break.break_token.to_tokens(tokens);
                mark_break.label.to_tokens(tokens);
            },
            ExprMark::Return(mark_return) => 
                mark_return.return_token.to_tokens(tokens),
            // ExprMark::Macro(mark::Macro),
            // ExprMark::Paren(mark::Paren),
            // ExprMark::Group(mark::Group),
            ExprMark::Async(mark_async) => {
                mark_async.async_token.to_tokens(tokens);
                mark_async.capture.to_tokens(tokens);
            },
            ExprMark::TryBlock(mark_try_block) => 
                mark_try_block.try_token.to_tokens(tokens),
            ExprMark::Yield(mark_yield) => 
                mark_yield.yield_token.to_tokens(tokens),
        }
    }
}