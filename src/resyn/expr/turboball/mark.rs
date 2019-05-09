mod parse;
mod quote;

use super::*;

#[derive(Clone)]
pub enum ExprMark {
    Box(mark::MarkBox),
    // InPlace(mark::InPlace),
    Unary(mark::Unary),
    Let(mark::Let),
    If(mark::If),
    While(mark::While),
    ForLoop(mark::ForLoop),
    Loop(mark::Loop),
    Match(mark::Match),
    Unsafe(mark::Unsafe),
    // Block(mark::Block),
    // Assign(mark::Assign),
    // AssignOp(mark::AssignOp),
    Reference(mark::Reference),
    Break(mark::Break),
    Return(mark::Return),
    // Macro(mark::Macro),
    // Paren(mark::Paren),
    // Group(mark::Group),
    Async(mark::Async),
    TryBlock(mark::TryBlock),
    Yield(mark::Yield),
}

#[derive(Clone)]
pub struct MarkBox {
    pub box_token: syn::Token![box],
}

// TODO
// #[derive(Clone)]
// pub struct InPlace {
//     pub place: Box<Expr>,
//     pub arrow_token: syn::Token![<-],
// }

#[derive(Clone)]
pub struct Unary {
    pub op: syn::UnOp
}

#[derive(Clone)]
pub struct Let {
    pub let_token: syn::Token![let],
    pub pats: Punctuated<syn::Pat, syn::Token![|]>,
    pub eq_token: syn::Token![=], // maybe remove
}

#[derive(Clone)]
pub struct If {
    pub if_token: syn::Token![if],
}

#[derive(Clone)]
pub struct While {
    pub label: Option<syn::Label>,
    pub while_token: syn::Token![while],
}

#[derive(Clone)]
pub struct ForLoop {
    pub label: Option<syn::Label>,
    pub for_token: syn::Token![for],
    pub pat: Box<syn::Pat>,
    pub in_token: syn::Token![in],
}

#[derive(Clone)]
pub struct Loop {
    pub label: Option<syn::Label>,
    pub loop_token: syn::Token![loop]
}

#[derive(Clone)]
pub struct Match {
    pub match_token: syn::Token![match],
}

#[derive(Clone)]
pub struct Unsafe {
    pub unsafe_token: syn::Token![unsafe]
}

// #[derive(Clone)]
// pub struct Block {
//     pub label: Option<syn::Label>
// }

// #[derive(Clone)]
// pub struct Assign {
//     pub left: Box<Expr>,
//     pub eq_token: syn::Token![=], // maybe remove
// }

// #[derive(Clone)]
// pub struct AssignOp {
//     pub left: Box<Expr>,
//     pub op: syn::BinOp,
// }

#[derive(Clone)]
pub struct Reference {
    pub and_token: syn::Token![&],
    pub mutability: Option<syn::Token![mut]>,
}

#[derive(Clone)]
pub struct Break {
    pub break_token: syn::Token![break],
    pub label: Option<syn::Lifetime>,
}

#[derive(Clone)]
pub struct Return {
    pub return_token: syn::Token![return],
}

// #[derive(Clone)]
// pub struct Paren {
//     pub paren_token: syn::token::Paren,
// }

// #[derive(Clone)]
// pub struct Group {
//     pub group_token: syn::token::Group,
// }

#[derive(Clone)]
pub struct Async {
    pub async_token: syn::Token![async],
    pub capture: Option<syn::Token![move]>,
}

#[derive(Clone)]
pub struct TryBlock {
    pub try_token: syn::Token![try],
}

#[derive(Clone)]
pub struct Yield {
    pub yield_token: syn::Token![yield],
}

// TODO: Macro
// #[derive(Clone)]
// pub struct Macro {
//     pub mac: crate::resyn::Macro,
// }