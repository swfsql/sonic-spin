mod quote;

use super::*;

#[derive(Clone)]
pub enum PostExprMark {
    If(post_mark::If),
    While(post_mark::While),
    ForLoop(post_mark::ForLoop),
    Match(post_mark::Match),
}

#[derive(Clone)]
pub struct If {
    pub then_branch: Block,
    pub else_branch: Option<(syn::Token![else], Box<Expr>)>,
}

#[derive(Clone)]
pub struct While {
    pub attrs: Vec<syn::Attribute>,
    pub body: Block,
}

#[derive(Clone)]
pub struct ForLoop {
    pub attrs: Vec<syn::Attribute>,
    pub body: Block,
}

#[derive(Clone)]
pub struct Match {
    pub attrs: Vec<syn::Attribute>,
    pub brace_token: syn::token::Brace,
    pub arms: Vec<Arm>,
}

#[cfg(feature = "full")]
impl syn::parse::Parse for If {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let then_branch = input.parse()?;
        let else_branch = {
            if input.peek(syn::Token![else]) {
                Some(input.call(parsing::else_block)?)
            } else {
                None
            }
        };
        Ok(If {then_branch, else_branch})
    }
}

#[cfg(feature = "full")]
impl syn::parse::Parse for While {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let brace_token = syn::braced!(content in input);
        let inner_attrs = content.call(syn::Attribute::parse_inner)?;
        let stmts = content.call(Block::parse_within)?;
        Ok(While {
            attrs: inner_attrs,
            body: Block {
                brace_token: brace_token,
                stmts: stmts,
            },
        })
    }
}

#[cfg(feature = "full")]
impl syn::parse::Parse for ForLoop {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let brace_token = syn::braced!(content in input);
        let inner_attrs = content.call(syn::Attribute::parse_inner)?;
        let stmts = content.call(Block::parse_within)?;
        Ok(ForLoop {
            attrs: inner_attrs,
            body: Block {
                brace_token: brace_token,
                stmts: stmts,
            },
        })
    }
}

#[cfg(feature = "full")]
impl syn::parse::Parse for Match {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let brace_token = syn::braced!(content in input);
        let inner_attrs = content.call(syn::Attribute::parse_inner)?;

        let mut arms = Vec::new();
        while !content.is_empty() {
            arms.push(content.call(Arm::parse)?);
        }

        Ok(Match {
            attrs: inner_attrs,
            brace_token: brace_token,
            arms: arms,
        })
    }
}