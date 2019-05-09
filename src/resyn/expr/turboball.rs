use crate::resyn::expr::{Block, Expr, Arm, parsing, ExprTurboball};
use syn::punctuated::Punctuated;

pub mod mark;
pub mod post_mark;

pub use mark::ExprMark;
pub use post_mark::PostExprMark;
use syn::parse::{Result, ParseBuffer};

pub fn parse_turboball(input: &ParseBuffer, e: Expr) -> Result<Expr> {
    let colon2_token: syn::Token![::] = input.parse()?;
    let content;
    let paren_token = syn::parenthesized!(content in input);
    let expr_mark: ExprMark = content.parse()?;

    let post_mark = match expr_mark {
        ExprMark::If(_) => {
            let mark: post_mark::If = input.parse()?;
            Some(PostExprMark::If(mark))
        },
        ExprMark::While(_) => {
            let mark: post_mark::While = input.parse()?;
            Some(PostExprMark::While(mark))
        },
        ExprMark::ForLoop(_) => {
            let mark: post_mark::ForLoop = input.parse()?;
            Some(PostExprMark::ForLoop(mark))
        },
        ExprMark::Match(_) => {
            let mark: post_mark::Match = input.parse()?;
            Some(PostExprMark::Match(mark))
        },
        _ => None
    };

    Ok(Expr::Turboball(ExprTurboball {
        attrs: Vec::new(),
        expr: Box::new(e),
        colon2_token,
        paren_token,
        expr_mark,
        post_mark,
    }))
}
