use super::PostExprMark;
use crate::resyn::expr;

#[cfg(feature = "printing")]
impl quote::ToTokens for PostExprMark {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        use quote::TokenStreamExt;
        match self {
            PostExprMark::If(post_if) => {
                post_if.then_branch.to_tokens(tokens);
                expr::printing::maybe_wrap_else(tokens, &post_if.else_branch);
            }
            PostExprMark::While(post_while) => {
                post_while.body.brace_token.surround(tokens, |tokens| {
                    expr::printing::inner_attrs_to_tokens(&post_while.attrs, tokens);
                    tokens.append_all(&post_while.body.stmts);
                });
            }
            PostExprMark::ForLoop(post_for_loop) => {
                post_for_loop.body.brace_token.surround(tokens, |tokens| {
                    expr::printing::inner_attrs_to_tokens(&post_for_loop.attrs, tokens);
                    tokens.append_all(&post_for_loop.body.stmts);
                });
            }
            PostExprMark::Match(post_match) => {
                post_match.brace_token.surround(tokens, |tokens| {
                    expr::printing::inner_attrs_to_tokens(&post_match.attrs, tokens);
                    for (i, arm) in post_match.arms.iter().enumerate() {
                        arm.to_tokens(tokens);
                        // Ensure that we have a comma after a non-block arm, except
                        // for the last one.
                        let is_last = i == post_match.arms.len() - 1;
                        if !is_last && expr::requires_terminator(&arm.body) && arm.comma.is_none() {
                            <syn::Token![,]>::default().to_tokens(tokens);
                        }
                    }
                });
            }
        }
    }
}