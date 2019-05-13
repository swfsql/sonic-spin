use crate::resyn::expr::turboball::mark;
use crate::resyn::expr::turboball::ExprMark;
use syn::punctuated::Punctuated;

#[cfg(feature = "full")]
impl syn::parse::Parse for ExprMark {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mark = if input.peek(syn::Token![&]) {
            let and_token = input.parse()?;
            let mutability = input.parse()?;
            let mark = mark::Reference {
                and_token,
                mutability,
            };
            ExprMark::Reference(mark)
        } else if input.peek(syn::Token![box]) {
            let box_token = input.parse()?;
            let mark = mark::MarkBox { box_token };
            ExprMark::Box(mark)
        } else if input.peek(syn::Token![*])
            || input.peek(syn::Token![!])
            || input.peek(syn::Token![-])
        {
            let op = input.parse()?;
            let mark = mark::Unary { op };
            ExprMark::Unary(mark)
        } else if input.peek(syn::Token![let]) {
            let let_token = input.parse()?;
            let pats = {
                let mut pats = Punctuated::new();
                input.parse::<Option<syn::Token![|]>>()?;
                let value: syn::Pat = input.parse()?;
                pats.push_value(value);
                while input.peek(syn::Token![|])
                    && !input.peek(syn::Token![||])
                    && !input.peek(syn::Token![|=])
                {
                    let punct = input.parse()?;
                    pats.push_punct(punct);
                    let value: syn::Pat = input.parse()?;
                    pats.push_value(value);
                }
                pats
            };
            let eq_token = input.parse()?;
            let mark = mark::Let {
                let_token,
                pats,
                eq_token,
            };
            ExprMark::Let(mark)
        } else if input.peek(syn::Token![if]) {
            let if_token = input.parse()?;
            let mark = mark::If { if_token };
            ExprMark::If(mark)
        } else if input.peek(syn::Lifetime) {
            let label: syn::Label = input.parse()?;
            let label = Some(label);
            if input.peek(syn::Token![while]) {
                let while_token = input.parse()?;
                let mark = mark::While { label, while_token };
                ExprMark::While(mark)
            } else if input.peek(syn::Token![for]) {
                let for_token = input.parse()?;
                let pat: syn::Pat = input.parse()?;
                let pat = Box::new(pat);
                let in_token: syn::Token![in] = input.parse()?;
                let mark = mark::ForLoop {
                    label,
                    for_token,
                    pat,
                    in_token,
                };
                ExprMark::ForLoop(mark)
            } else if input.peek(syn::Token![loop]) {
                let loop_token = input.parse()?;
                let mark = mark::Loop { label, loop_token };
                ExprMark::Loop(mark)
            } else if input.is_empty() {
                let mark = mark::Block { label };
                ExprMark::Block(mark)
            } else {
                return Err(input.error("expected loop or block expression"));
            }
        } else if input.peek(syn::Token![while]) {
            let label = None;
            let while_token = input.parse()?;
            let mark = mark::While { label, while_token };
            ExprMark::While(mark)
        } else if input.peek(syn::Token![for]) {
            let label = None;
            let for_token = input.parse()?;
            let pat: syn::Pat = input.parse()?;
            let pat = Box::new(pat);
            let in_token: syn::Token![in] = input.parse()?;
            let mark = mark::ForLoop {
                label,
                for_token,
                pat,
                in_token,
            };
            ExprMark::ForLoop(mark)
        } else if input.peek(syn::Token![loop]) {
            let label = None;
            let loop_token = input.parse()?;
            let mark = mark::Loop { label, loop_token };
            ExprMark::Loop(mark)
        } else if input.peek(syn::Token![match]) {
            let match_token = input.parse()?;
            let mark = mark::Match { match_token };
            ExprMark::Match(mark)
        } else if input.peek(syn::Token![unsafe]) {
            let unsafe_token = input.parse()?;
            let mark = mark::Unsafe { unsafe_token };
            ExprMark::Unsafe(mark)
        } else if input.peek(syn::Token![break]) {
            let break_token = input.parse()?;
            let label = input.parse()?;
            let mark = mark::Break { break_token, label };
            ExprMark::Break(mark)
        } else if input.peek(syn::Token![return]) {
            let return_token = input.parse()?;
            let mark = mark::Return { return_token };
            ExprMark::Return(mark)
        } else if input.peek(syn::token::Group) {
            return Err(input.error("TODO Group Turboball"));
        } else if input.peek(syn::Token![async]) {
            let async_token = input.parse()?;
            let capture = input.parse()?;
            let mark = mark::Async {
                async_token,
                capture,
            };
            ExprMark::Async(mark)
        } else if input.peek(syn::Token![try]) {
            let try_token = input.parse()?;
            let mark = mark::TryBlock { try_token };
            ExprMark::TryBlock(mark)
        } else if input.peek(syn::Token![yield]) {
            let yield_token = input.parse()?;
            let mark = mark::Yield { yield_token };
            ExprMark::Yield(mark)
        } else {
            return Err(input.error("Unkown Turboball marker"));
        };
        Ok(mark)
    }
}
