// changes https://github.com/dtolnay/syn/blob/master/src/expr.rs

use syn::Ident;
use proc_macro2::{Span, TokenStream};
use syn::punctuated::Punctuated;
#[cfg(feature = "extra-traits")]
use std::hash::{Hash, Hasher};
#[cfg(all(feature = "parsing", feature = "full"))]
use std::mem;
#[cfg(feature = "extra-traits")]

use crate::resyn;

use syn::{ast_enum_of_structs, ast_enum, ast_struct, maybe_ast_struct, generate_to_tokens, to_tokens_call};

pub mod turboball;

ast_enum_of_structs! {
    /// A Rust expression.
    ///
    /// *This type is available if Syn is built with the `"derive"` or `"full"`
    /// feature.*
    ///
    /// # Syntax tree enums
    ///
    /// This type is a syntax tree enum. In Syn this and other syntax tree enums
    /// are designed to be traversed using the following rebinding idiom.
    ///
    /// ```edition2018
    /// # use syn::Expr;
    /// #
    /// # fn example(expr: Expr) {
    /// # const IGNORE: &str = stringify! {
    /// let expr: Expr = /* ... */;
    /// # };
    /// match expr {
    ///     Expr::MethodCall(expr) => {
    ///         /* ... */
    ///     }
    ///     Expr::Cast(expr) => {
    ///         /* ... */
    ///     }
    ///     Expr::If(expr) => {
    ///         /* ... */
    ///     }
    ///
    ///     /* ... */
    ///     # _ => {}
    /// # }
    /// # }
    /// ```
    ///
    /// We begin with a variable `expr` of type `Expr` that has no fields
    /// (because it is an enum), and by matching on it and rebinding a variable
    /// with the same name `expr` we effectively imbue our variable with all of
    /// the data fields provided by the variant that it turned out to be. So for
    /// example above if we ended up in the `MethodCall` case then we get to use
    /// `expr.receiver`, `expr.args` etc; if we ended up in the `If` case we get
    /// to use `expr.cond`, `expr.then_branch`, `expr.else_branch`.
    ///
    /// This approach avoids repeating the variant names twice on every line.
    ///
    /// ```edition2018
    /// # use syn::{Expr, ExprMethodCall};
    /// #
    /// # fn example(expr: Expr) {
    /// // Repetitive; recommend not doing this.
    /// match expr {
    ///     Expr::MethodCall(ExprMethodCall { method, args, .. }) => {
    /// # }
    /// # _ => {}
    /// # }
    /// # }
    /// ```
    ///
    /// In general, the name to which a syntax tree enum variant is bound should
    /// be a suitable name for the complete syntax tree enum type.
    ///
    /// ```edition2018
    /// # use syn::{Expr, ExprField};
    /// #
    /// # fn example(discriminant: ExprField) {
    /// // Binding is called `base` which is the name I would use if I were
    /// // assigning `*discriminant.base` without an `if let`.
    /// if let Expr::Tuple(base) = *discriminant.base {
    /// # }
    /// # }
    /// ```
    ///
    /// A sign that you may not be choosing the right variable names is if you
    /// see names getting repeated in your code, like accessing
    /// `receiver.receiver` or `pat.pat` or `cond.cond`.
    pub enum Expr {
        /// A box expression: `box f`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Box(ExprBox #full {
            pub attrs: Vec<syn::Attribute>,
            pub box_token: syn::Token![box],
            pub expr: Box<Expr>,
        }),

        
        /// A placement expression: `place <- value`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub InPlace(ExprInPlace #full {
            pub attrs: Vec<syn::Attribute>,
            pub place: Box<Expr>,
            pub arrow_token: syn::Token![<-],
            pub value: Box<Expr>,
        }),

        /// A slice literal expression: `[a, b, c, d]`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Array(ExprArray #full {
            pub attrs: Vec<syn::Attribute>,
            pub bracket_token: syn::token::Bracket,
            pub elems: Punctuated<Expr, syn::Token![,]>,
        }),

        /// A function call expression: `invoke(a, b)`.
        ///
        /// *This type is available if Syn is built with the `"derive"` or
        /// `"full"` feature.*
        pub Call(ExprCall {
            pub attrs: Vec<syn::Attribute>,
            pub func: Box<Expr>,
            pub paren_token: syn::token::Paren,
            pub args: Punctuated<Expr, syn::Token![,]>,
        }),

        /// A method call expression: `x.foo::<T>(a, b)`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub MethodCall(ExprMethodCall #full {
            pub attrs: Vec<syn::Attribute>,
            pub receiver: Box<Expr>,
            pub dot_token: syn::Token![.],
            pub method: Ident,
            pub turbofish: Option<MethodTurbofish>,
            pub paren_token: syn::token::Paren,
            pub args: Punctuated<Expr, syn::Token![,]>,
        }),

        /// A tuple expression: `(a, b, c, d)`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Tuple(ExprTuple #full {
            pub attrs: Vec<syn::Attribute>,
            pub paren_token: syn::token::Paren,
            pub elems: Punctuated<Expr, syn::Token![,]>,
        }),

        /// A binary operation: `a + b`, `a * b`.
        ///
        /// *This type is available if Syn is built with the `"derive"` or
        /// `"full"` feature.*
        pub Binary(ExprBinary {
            pub attrs: Vec<syn::Attribute>,
            pub left: Box<Expr>,
            pub op: syn::BinOp,
            pub right: Box<Expr>,
        }),

        /// A unary operation: `!x`, `*x`.
        ///
        /// *This type is available if Syn is built with the `"derive"` or
        /// `"full"` feature.*
        pub Unary(ExprUnary {
            pub attrs: Vec<syn::Attribute>,
            pub op: syn::UnOp,
            pub expr: Box<Expr>,
        }),

        /// A literal in place of an expression: `1`, `"foo"`.
        ///
        /// *This type is available if Syn is built with the `"derive"` or
        /// `"full"` feature.*
        pub Lit(ExprLit {
            pub attrs: Vec<syn::Attribute>,
            pub lit: syn::Lit,
        }),

        /// A cast expression: `foo as f64`.
        ///
        /// *This type is available if Syn is built with the `"derive"` or
        /// `"full"` feature.*
        pub Cast(ExprCast {
            pub attrs: Vec<syn::Attribute>,
            pub expr: Box<Expr>,
            pub as_token: syn::Token![as],
            pub ty: Box<syn::Type>,
        }),

        /// A type ascription expression: `foo: f64`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Type(ExprType #full {
            pub attrs: Vec<syn::Attribute>,
            pub expr: Box<Expr>,
            pub colon_token: syn::Token![:],
            pub ty: Box<syn::Type>,
        }),

        /// A `let` guard: `let Some(x) = opt`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Let(ExprLet #full {
            pub attrs: Vec<syn::Attribute>,
            pub let_token: syn::Token![let],
            pub pats: Punctuated<syn::Pat, syn::Token![|]>,
            pub eq_token: syn::Token![=],
            pub expr: Box<Expr>,
        }),

        /// An `if` expression with an optional `else` block: `if expr { ... }
        /// else { ... }`.
        ///
        /// The `else` branch expression may only be an `If` or `Block`
        /// expression, not any of the other types of expression.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub If(ExprIf #full {
            pub attrs: Vec<syn::Attribute>,
            pub if_token: syn::Token![if],
            pub cond: Box<Expr>,
            pub then_branch: Block,
            pub else_branch: Option<(syn::Token![else], Box<Expr>)>,
        }),

        /// A while loop: `while expr { ... }`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub While(ExprWhile #full {
            pub attrs: Vec<syn::Attribute>,
            pub label: Option<syn::Label>,
            pub while_token: syn::Token![while],
            pub cond: Box<Expr>,
            pub body: Block,
        }),

        /// A for loop: `for pat in expr { ... }`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub ForLoop(ExprForLoop #full {
            pub attrs: Vec<syn::Attribute>,
            pub label: Option<syn::Label>,
            pub for_token: syn::Token![for],
            pub pat: Box<syn::Pat>,
            pub in_token: syn::Token![in],
            pub expr: Box<Expr>,
            pub body: Block,
        }),

        /// Conditionless loop: `loop { ... }`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Loop(ExprLoop #full {
            pub attrs: Vec<syn::Attribute>,
            pub label: Option<syn::Label>,
            pub loop_token: syn::Token![loop],
            pub body: Block,
        }),

        /// A `match` expression: `match n { Some(n) => {}, None => {} }`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Match(ExprMatch #full {
            pub attrs: Vec<syn::Attribute>,
            pub match_token: syn::Token![match],
            pub expr: Box<Expr>,
            pub brace_token: syn::token::Brace,
            pub arms: Vec<Arm>,
        }),

        /// A closure expression: `|a, b| a + b`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Closure(ExprClosure #full {
            pub attrs: Vec<syn::Attribute>,
            pub asyncness: Option<syn::Token![async]>,
            pub movability: Option<syn::Token![static]>,
            pub capture: Option<syn::Token![move]>,
            pub or1_token: syn::Token![|],
            pub inputs: Punctuated<syn::FnArg, syn::Token![,]>,
            pub or2_token: syn::Token![|],
            pub output: syn::ReturnType,
            pub body: Box<Expr>,
        }),

        /// An unsafe block: `unsafe { ... }`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Unsafe(ExprUnsafe #full {
            pub attrs: Vec<syn::Attribute>,
            pub unsafe_token: syn::Token![unsafe],
            pub block: Block,
        }),

        /// A blocked scope: `{ ... }`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Block(ExprBlock #full {
            pub attrs: Vec<syn::Attribute>,
            pub label: Option<syn::Label>,
            pub block: Block,
        }),

        /// An assignment expression: `a = compute()`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Assign(ExprAssign #full {
            pub attrs: Vec<syn::Attribute>,
            pub left: Box<Expr>,
            pub eq_token: syn::Token![=],
            pub right: Box<Expr>,
        }),

        /// A compound assignment expression: `counter += 1`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub AssignOp(ExprAssignOp #full {
            pub attrs: Vec<syn::Attribute>,
            pub left: Box<Expr>,
            pub op: syn::BinOp,
            pub right: Box<Expr>,
        }),

        /// Access of a named struct field (`obj.k`) or unnamed tuple struct
        /// field (`obj.0`).
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Field(ExprField {
            pub attrs: Vec<syn::Attribute>,
            pub base: Box<Expr>,
            pub dot_token: syn::Token![.],
            pub member: Member,
        }),

        /// A square bracketed indexing expression: `vector[2]`.
        ///
        /// *This type is available if Syn is built with the `"derive"` or
        /// `"full"` feature.*
        pub Index(ExprIndex {
            pub attrs: Vec<syn::Attribute>,
            pub expr: Box<Expr>,
            pub bracket_token: syn::token::Bracket,
            pub index: Box<Expr>,
        }),

        /// A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Range(ExprRange #full {
            pub attrs: Vec<syn::Attribute>,
            pub from: Option<Box<Expr>>,
            pub limits: syn::RangeLimits,
            pub to: Option<Box<Expr>>,
        }),

        /// A path like `std::mem::replace` possibly containing generic
        /// parameters and a qualified self-type.
        ///
        /// A plain identifier like `x` is a path of length 1.
        ///
        /// *This type is available if Syn is built with the `"derive"` or
        /// `"full"` feature.*
        pub Path(ExprPath {
            pub attrs: Vec<syn::Attribute>,
            pub qself: Option<syn::QSelf>,
            pub path: syn::Path,
        }),

        /// A referencing operation: `&a` or `&mut a`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Reference(ExprReference #full {
            pub attrs: Vec<syn::Attribute>,
            pub and_token: syn::Token![&],
            pub mutability: Option<syn::Token![mut]>,
            pub expr: Box<Expr>,
        }),

        /// A `break`, with an optional label to break and an optional
        /// expression.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Break(ExprBreak #full {
            pub attrs: Vec<syn::Attribute>,
            pub break_token: syn::Token![break],
            pub label: Option<syn::Lifetime>,
            pub expr: Option<Box<Expr>>,
        }),

        /// A `continue`, with an optional label.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Continue(ExprContinue #full {
            pub attrs: Vec<syn::Attribute>,
            pub continue_token: syn::Token![continue],
            pub label: Option<syn::Lifetime>,
        }),

        /// A `return`, with an optional value to be returned.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Return(ExprReturn #full {
            pub attrs: Vec<syn::Attribute>,
            pub return_token: syn::Token![return],
            pub expr: Option<Box<Expr>>,
        }),

        /// A macro invocation expression: `format!("{}", q)`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Macro(ExprMacro #full {
            pub attrs: Vec<syn::Attribute>,
            pub mac: crate::resyn::Macro,
        }),

        /// A struct literal expression: `Point { x: 1, y: 1 }`.
        ///
        /// The `rest` provides the value of the remaining fields as in `S { a:
        /// 1, b: 1, ..rest }`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Struct(ExprStruct #full {
            pub attrs: Vec<syn::Attribute>,
            pub path: syn::Path,
            pub brace_token: syn::token::Brace,
            pub fields: Punctuated<FieldValue, syn::Token![,]>,
            pub dot2_token: Option<syn::Token![..]>,
            pub rest: Option<Box<Expr>>,
        }),

        /// An array literal constructed from one repeated element: `[0u8; N]`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Repeat(ExprRepeat #full {
            pub attrs: Vec<syn::Attribute>,
            pub bracket_token: syn::token::Bracket,
            pub expr: Box<Expr>,
            pub semi_token: syn::Token![;],
            pub len: Box<Expr>,
        }),

        /// A parenthesized expression: `(a + b)`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Paren(ExprParen {
            pub attrs: Vec<syn::Attribute>,
            pub paren_token: syn::token::Paren,
            pub expr: Box<Expr>,
        }),

        /// An expression contained within invisible delimiters.
        ///
        /// This variant is important for faithfully representing the precedence
        /// of expressions and is related to `None`-delimited spans in a
        /// `TokenStream`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Group(ExprGroup #full {
            pub attrs: Vec<syn::Attribute>,
            pub group_token: syn::token::Group,
            pub expr: Box<Expr>,
        }),

        /// A try-expression: `expr?`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Try(ExprTry #full {
            pub attrs: Vec<syn::Attribute>,
            pub expr: Box<Expr>,
            pub question_token: syn::Token![?],
        }),

        /// A turboball expression: `expr::(..)`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Turboball(ExprTurboball #full {
            pub attrs: Vec<syn::Attribute>,
            pub expr: Box<Expr>,
            pub colon2_token: syn::Token![::],
            pub paren_token: syn::token::Paren,
            pub expr_mark: turboball::ExprMark,
            pub post_mark: Option<turboball::PostExprMark>,
        }),

        /// An async block: `async { ... }`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Async(ExprAsync #full {
            pub attrs: Vec<syn::Attribute>,
            pub async_token: syn::Token![async],
            pub capture: Option<syn::Token![move]>,
            pub block: Block,
        }),

        /// A try block: `try { ... }`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub TryBlock(ExprTryBlock #full {
            pub attrs: Vec<syn::Attribute>,
            pub try_token: syn::Token![try],
            pub block: Block,
        }),

        /// A yield expression: `yield expr`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Yield(ExprYield #full {
            pub attrs: Vec<syn::Attribute>,
            pub yield_token: syn::Token![yield],
            pub expr: Option<Box<Expr>>,
        }),

        /// Tokens in expression position not interpreted by Syn.
        ///
        /// *This type is available if Syn is built with the `"derive"` or
        /// `"full"` feature.*
        pub Verbatim(ExprVerbatim #manual_extra_traits {
            pub tts: TokenStream,
        }),
    }
}

#[cfg(feature = "extra-traits")]
impl Eq for ExprVerbatim {}

#[cfg(feature = "extra-traits")]
impl PartialEq for ExprVerbatim {
    fn eq(&self, other: &Self) -> bool {
        TokenStreamHelper(&self.tts) == TokenStreamHelper(&other.tts)
    }
}

#[cfg(feature = "extra-traits")]
impl Hash for ExprVerbatim {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        TokenStreamHelper(&self.tts).hash(state);
    }
}

impl Expr {
    #[cfg(all(feature = "parsing", feature = "full"))]
    fn replace_attrs(&mut self, new: Vec<syn::Attribute>) -> Vec<syn::Attribute> {
        match *self {
            Expr::Box(ExprBox { ref mut attrs, .. })
            | Expr::InPlace(ExprInPlace { ref mut attrs, .. })
            | Expr::Array(ExprArray { ref mut attrs, .. })
            | Expr::Call(ExprCall { ref mut attrs, .. })
            | Expr::MethodCall(ExprMethodCall { ref mut attrs, .. })
            | Expr::Tuple(ExprTuple { ref mut attrs, .. })
            | Expr::Binary(ExprBinary { ref mut attrs, .. })
            | Expr::Unary(ExprUnary { ref mut attrs, .. })
            | Expr::Lit(ExprLit { ref mut attrs, .. })
            | Expr::Cast(ExprCast { ref mut attrs, .. })
            | Expr::Type(ExprType { ref mut attrs, .. })
            | Expr::Let(ExprLet { ref mut attrs, .. })
            | Expr::If(ExprIf { ref mut attrs, .. })
            | Expr::While(ExprWhile { ref mut attrs, .. })
            | Expr::ForLoop(ExprForLoop { ref mut attrs, .. })
            | Expr::Loop(ExprLoop { ref mut attrs, .. })
            | Expr::Match(ExprMatch { ref mut attrs, .. })
            | Expr::Closure(ExprClosure { ref mut attrs, .. })
            | Expr::Unsafe(ExprUnsafe { ref mut attrs, .. })
            | Expr::Block(ExprBlock { ref mut attrs, .. })
            | Expr::Assign(ExprAssign { ref mut attrs, .. })
            | Expr::AssignOp(ExprAssignOp { ref mut attrs, .. })
            | Expr::Field(ExprField { ref mut attrs, .. })
            | Expr::Index(ExprIndex { ref mut attrs, .. })
            | Expr::Range(ExprRange { ref mut attrs, .. })
            | Expr::Path(ExprPath { ref mut attrs, .. })
            | Expr::Reference(ExprReference { ref mut attrs, .. })
            | Expr::Break(ExprBreak { ref mut attrs, .. })
            | Expr::Continue(ExprContinue { ref mut attrs, .. })
            | Expr::Return(ExprReturn { ref mut attrs, .. })
            | Expr::Macro(ExprMacro { ref mut attrs, .. })
            | Expr::Struct(ExprStruct { ref mut attrs, .. })
            | Expr::Repeat(ExprRepeat { ref mut attrs, .. })
            | Expr::Paren(ExprParen { ref mut attrs, .. })
            | Expr::Group(ExprGroup { ref mut attrs, .. })
            | Expr::Try(ExprTry { ref mut attrs, .. })
            | Expr::Async(ExprAsync { ref mut attrs, .. })
            | Expr::TryBlock(ExprTryBlock { ref mut attrs, .. })
            | Expr::Turboball(ExprTurboball { ref mut attrs, .. })
            | Expr::Yield(ExprYield { ref mut attrs, .. }) => mem::replace(attrs, new),
            Expr::Verbatim(_) => Vec::new(),
        }
    }
}

ast_enum! {
    /// A struct or tuple struct field accessed in a struct literal or field
    /// expression.
    ///
    /// *This type is available if Syn is built with the `"derive"` or `"full"`
    /// feature.*
    pub enum Member {
        /// A named field like `self.x`.
        Named(syn::Ident),
        /// An unnamed field like `self.0`.
        Unnamed(syn::Index),
    }
}

ast_struct! {
    /// The index of an unnamed tuple struct field.
    ///
    /// *This type is available if Syn is built with the `"derive"` or `"full"`
    /// feature.*
    pub struct Index #manual_extra_traits {
        pub index: u32,
        pub span: Span,
    }
}

impl From<usize> for Index {
    fn from(index: usize) -> Index {
        assert!(index < u32::max_value() as usize);
        Index {
            index: index as u32,
            span: Span::call_site(),
        }
    }
}

#[cfg(feature = "extra-traits")]
impl Eq for Index {}

#[cfg(feature = "extra-traits")]
impl PartialEq for Index {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

#[cfg(feature = "extra-traits")]
impl Hash for Index {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

#[cfg(feature = "full")]
ast_struct! {
    /// The `::<>` explicit type parameters passed to a method call:
    /// `parse::<u64>()`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct MethodTurbofish {
        pub colon2_token: syn::Token![::],
        pub lt_token: syn::Token![<],
        pub args: Punctuated<GenericMethodArgument, syn::Token![,]>,
        pub gt_token: syn::Token![>],
    }
}

#[cfg(feature = "full")]
ast_enum! {
    /// An individual generic argument to a method, like `T`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub enum GenericMethodArgument {
        /// A type argument.
        Type(syn::Type),
        /// A const expression. Must be inside of a block.
        ///
        /// NOTE: Identity expressions are represented as Type arguments, as
        /// they are indistinguishable syntactically.
        Const(Expr),
    }
}

#[cfg(feature = "full")]
ast_struct! {
    /// A field-value pair in a struct literal.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct FieldValue {
        /// syn::Attributes tagged on the field.
        pub attrs: Vec<syn::Attribute>,

        /// Name or index of the field.
        pub member: Member,

        /// The colon in `Struct { x: x }`. If written in shorthand like
        /// `Struct { x }`, there is no colon.
        pub colon_token: Option<syn::Token![:]>,

        /// Value of the field.
        pub expr: Expr,
    }
}

#[cfg(feature = "full")]
ast_struct! {
    /// A lifetime labeling a `for`, `while`, or `loop`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct Label {
        pub name: syn::Lifetime,
        pub colon_token: syn::Token![:],
    }
}

#[cfg(feature = "full")]
ast_struct! {
    /// A braced block containing Rust statements.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct Block {
        pub brace_token: syn::token::Brace,
        /// Statements in a block
        pub stmts: Vec<Stmt>,
    }
}

#[cfg(feature = "full")]
ast_enum! {
    /// A statement, usually ending in a semicolon.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub enum Stmt {
        /// A local (let) binding.
        Local(Local),

        /// An item definition.
        Item(syn::Item),

        /// Expr without trailing semicolon.
        Expr(Expr),

        /// Expression with trailing semicolon.
        Semi(Expr, syn::Token![;]),
    }
}

#[cfg(feature = "full")]
ast_struct! {
    /// A local `let` binding: `let x: u64 = s.parse()?`.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct Local {
        pub attrs: Vec<syn::Attribute>,
        pub let_token: syn::Token![let],
        pub pats: Punctuated<Pat, syn::Token![|]>,
        pub ty: Option<(syn::Token![:], Box<syn::Type>)>,
        pub init: Option<(syn::Token![=], Box<Expr>)>,
        pub semi_token: syn::Token![;],
    }
}

#[cfg(feature = "full")]
ast_enum_of_structs! {
    /// A pattern in a local binding, function signature, match expression, or
    /// various other places.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    ///
    /// # Syntax tree enum
    ///
    /// This type is a [syntax tree enum].
    ///
    /// [syntax tree enum]: enum.Expr.html#syntax-tree-enums
    pub enum Pat {
        /// A pattern that matches any value: `_`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Wild(PatWild {
            pub underscore_token: syn::Token![_],
        }),

        /// A pattern that binds a new variable: `ref mut binding @ SUBPATTERN`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Ident(PatIdent {
            pub by_ref: Option<syn::Token![ref]>,
            pub mutability: Option<syn::Token![mut]>,
            pub ident: Ident,
            pub subpat: Option<(syn::Token![@], Box<Pat>)>,
        }),

        /// A struct or struct variant pattern: `Variant { x, y, .. }`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Struct(PatStruct {
            pub path: syn::Path,
            pub brace_token: syn::token::Brace,
            pub fields: Punctuated<FieldPat, syn::Token![,]>,
            pub dot2_token: Option<syn::Token![..]>,
        }),

        /// A tuple struct or tuple variant pattern: `Variant(x, y, .., z)`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub TupleStruct(PatTupleStruct {
            pub path: syn::Path,
            pub pat: PatTuple,
        }),

        /// A path pattern like `Color::Red`, optionally qualified with a
        /// self-type.
        ///
        /// Unqualified path patterns can legally refer to variants, structs,
        /// constants or associated constants. Qualified path patterns like
        /// `<A>::B::C` and `<A as Trait>::B::C` can only legally refer to
        /// associated constants.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Path(PatPath {
            pub qself: Option<syn::QSelf>,
            pub path: syn::Path,
        }),

        /// A tuple pattern: `(a, b)`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Tuple(PatTuple {
            pub paren_token: syn::token::Paren,
            pub front: Punctuated<syn::Pat, syn::Token![,]>,
            pub dot2_token: Option<syn::Token![..]>,
            pub comma_token: Option<syn::Token![,]>,
            pub back: Punctuated<syn::Pat, syn::Token![,]>,
        }),

        /// A box pattern: `box v`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Box(PatBox {
            pub box_token: syn::Token![box],
            pub pat: Box<syn::Pat>,
        }),

        /// A reference pattern: `&mut (first, second)`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Ref(PatRef {
            pub and_token: syn::Token![&],
            pub mutability: Option<syn::Token![mut]>,
            pub pat: Box<syn::Pat>,
        }),

        /// A literal pattern: `0`.
        ///
        /// This holds an `Expr` rather than a `Lit` because negative numbers
        /// are represented as an `Expr::Unary`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Lit(PatLit {
            pub expr: Box<Expr>,
        }),

        /// A range pattern: `1..=2`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Range(PatRange {
            pub lo: Box<Expr>,
            pub limits: syn::RangeLimits,
            pub hi: Box<Expr>,
        }),

        /// A dynamically sized slice pattern: `[a, b, i.., y, z]`.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Slice(PatSlice {
            pub bracket_token: syn::token::Bracket,
            pub front: syn::punctuated::Punctuated<Pat, syn::Token![,]>,
            pub middle: Option<Box<Pat>>,
            pub dot2_token: Option<syn::Token![..]>,
            pub comma_token: Option<syn::Token![,]>,
            pub back: syn::punctuated::Punctuated<Pat, syn::Token![,]>,
        }),

        /// A macro in expression position.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Macro(PatMacro {
            pub mac: syn::Macro,
        }),

        /// Tokens in pattern position not interpreted by Syn.
        ///
        /// *This type is available if Syn is built with the `"full"` feature.*
        pub Verbatim(PatVerbatim #manual_extra_traits {
            pub tts: TokenStream,
        }),
    }
}

#[cfg(all(feature = "full", feature = "extra-traits"))]
impl Eq for PatVerbatim {}

#[cfg(all(feature = "full", feature = "extra-traits"))]
impl PartialEq for PatVerbatim {
    fn eq(&self, other: &Self) -> bool {
        TokenStreamHelper(&self.tts) == TokenStreamHelper(&other.tts)
    }
}

#[cfg(all(feature = "full", feature = "extra-traits"))]
impl Hash for PatVerbatim {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        TokenStreamHelper(&self.tts).hash(state);
    }
}

#[cfg(feature = "full")]
ast_struct! {
    /// One arm of a `match` expression: `0...10 => { return true; }`.
    ///
    /// As in:
    ///
    /// ```edition2018
    /// # fn f() -> bool {
    /// #     let n = 0;
    /// match n {
    ///     0...10 => {
    ///         return true;
    ///     }
    ///     // ...
    ///     # _ => {}
    /// }
    /// #   false
    /// # }
    /// ```
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct Arm {
        pub attrs: Vec<syn::Attribute>,
        pub leading_vert: Option<syn::Token![|]>,
        pub pats: Punctuated<Pat, syn::Token![|]>,
        pub guard: Option<(syn::Token![if], Box<Expr>)>,
        pub fat_arrow_token: syn::Token![=>],
        pub body: Box<Expr>,
        pub comma: Option<syn::Token![,]>,
    }
}

#[cfg(feature = "full")]
ast_enum! {
    /// Limit types of a range, inclusive or exclusive.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    #[cfg_attr(feature = "clone-impls", derive(Copy))]
    pub enum RangeLimits {
        /// Inclusive at the beginning, exclusive at the end.
        HalfOpen(syn::Token![..]),
        /// Inclusive at the beginning and end.
        Closed(syn::Token![..=]),
    }
}

#[cfg(feature = "full")]
ast_struct! {
    /// A single field in a struct pattern.
    ///
    /// Patterns like the fields of Foo `{ x, ref y, ref mut z }` are treated
    /// the same as `x: x, y: ref y, z: ref mut z` but there is no colon token.
    ///
    /// *This type is available if Syn is built with the `"full"` feature.*
    pub struct FieldPat {
        pub attrs: Vec<syn::Attribute>,
        pub member: Member,
        pub colon_token: Option<syn::Token![:]>,
        pub pat: Box<syn::Pat>,
    }
}

#[cfg(any(feature = "parsing", feature = "printing"))]
#[cfg(feature = "full")]
fn requires_terminator(expr: &Expr) -> bool {
    // see https://github.com/rust-lang/rust/blob/eb8f2586e/src/libsyntax/parse/classify.rs#L17-L37
    match *expr {
        Expr::Unsafe(..)
        | Expr::Block(..)
        | Expr::If(..)
        | Expr::Match(..)
        | Expr::While(..)
        | Expr::Loop(..)
        | Expr::ForLoop(..)
        | Expr::Async(..)
        | Expr::TryBlock(..) => false,
        _ => true,
    }
}

#[cfg(feature = "parsing")]
pub mod parsing {
    use super::*;

    #[cfg(feature = "full")]
    use syn::ext::IdentExt;
    use syn::parse::{Parse, ParseStream, Result};
    // use path;

    // When we're parsing expressions which occur before blocks, like in an if
    // statement's condition, we cannot parse a struct literal.
    //
    // Struct literals are ambiguous in certain positions
    // https://github.com/rust-lang/rfcs/pull/92
    #[derive(Copy, Clone)]
    pub struct AllowStruct(bool);

    #[derive(Copy, Clone, PartialEq, PartialOrd)]
    enum Precedence {
        Any,
        Assign,
        Placement,
        Range,
        Or,
        And,
        Compare,
        BitOr,
        BitXor,
        BitAnd,
        Shift,
        Arithmetic,
        Term,
        Cast,
    }

    impl Precedence {
        fn of(op: &syn::BinOp) -> Self {
            match *op {
                syn::BinOp::Add(_) | syn::BinOp::Sub(_) => Precedence::Arithmetic,
                syn::BinOp::Mul(_) | syn::BinOp::Div(_) | syn::BinOp::Rem(_) => Precedence::Term,
                syn::BinOp::And(_) => Precedence::And,
                syn::BinOp::Or(_) => Precedence::Or,
                syn::BinOp::BitXor(_) => Precedence::BitXor,
                syn::BinOp::BitAnd(_) => Precedence::BitAnd,
                syn::BinOp::BitOr(_) => Precedence::BitOr,
                syn::BinOp::Shl(_) | syn::BinOp::Shr(_) => Precedence::Shift,
                syn::BinOp::Eq(_)
                | syn::BinOp::Lt(_)
                | syn::BinOp::Le(_)
                | syn::BinOp::Ne(_)
                | syn::BinOp::Ge(_)
                | syn::BinOp::Gt(_) => Precedence::Compare,
                syn::BinOp::AddEq(_)
                | syn::BinOp::SubEq(_)
                | syn::BinOp::MulEq(_)
                | syn::BinOp::DivEq(_)
                | syn::BinOp::RemEq(_)
                | syn::BinOp::BitXorEq(_)
                | syn::BinOp::BitAndEq(_)
                | syn::BinOp::BitOrEq(_)
                | syn::BinOp::ShlEq(_)
                | syn::BinOp::ShrEq(_) => Precedence::Assign,
            }
        }
    }

    impl Parse for Expr {
        fn parse(input: ParseStream) -> Result<Self> {
            ambiguous_expr(input, AllowStruct(true))
        }
    }

    #[cfg(feature = "full")]
    fn expr_no_struct(input: ParseStream) -> Result<Expr> {
        ambiguous_expr(input, AllowStruct(false))
    }

    #[cfg(feature = "full")]
    fn parse_expr(
        input: ParseStream,
        mut lhs: Expr,
        allow_struct: AllowStruct,
        base: Precedence,
    ) -> Result<Expr> {
        loop {
            if input
                .fork()
                .parse::<syn::BinOp>()
                .ok()
                .map_or(false, |op| Precedence::of(&op) >= base)
            {
                let op: syn::BinOp = input.parse()?;
                let precedence = Precedence::of(&op);
                let mut rhs = unary_expr(input, allow_struct)?;
                loop {
                    let next = peek_precedence(input);
                    if next > precedence || next == precedence && precedence == Precedence::Assign {
                        rhs = parse_expr(input, rhs, allow_struct, next)?;
                    } else {
                        break;
                    }
                }
                lhs = if precedence == Precedence::Assign {
                    Expr::AssignOp(ExprAssignOp {
                        attrs: Vec::new(),
                        left: Box::new(lhs),
                        op: op,
                        right: Box::new(rhs),
                    })
                } else {
                    Expr::Binary(ExprBinary {
                        attrs: Vec::new(),
                        left: Box::new(lhs),
                        op: op,
                        right: Box::new(rhs),
                    })
                };
            } else if Precedence::Assign >= base
                && input.peek(syn::Token![=])
                && !input.peek(syn::Token![==])
                && !input.peek(syn::Token![=>])
            {
                let eq_token: syn::Token![=] = input.parse()?;
                let mut rhs = unary_expr(input, allow_struct)?;
                loop {
                    let next = peek_precedence(input);
                    if next >= Precedence::Assign {
                        rhs = parse_expr(input, rhs, allow_struct, next)?;
                    } else {
                        break;
                    }
                }
                lhs = Expr::Assign(ExprAssign {
                    attrs: Vec::new(),
                    left: Box::new(lhs),
                    eq_token: eq_token,
                    right: Box::new(rhs),
                });
            } else if Precedence::Placement >= base && input.peek(syn::Token![<-]) {
                let arrow_token: syn::Token![<-] = input.parse()?;
                let mut rhs = unary_expr(input, allow_struct)?;
                loop {
                    let next = peek_precedence(input);
                    if next > Precedence::Placement {
                        rhs = parse_expr(input, rhs, allow_struct, next)?;
                    } else {
                        break;
                    }
                }
                lhs = Expr::InPlace(ExprInPlace {
                    attrs: Vec::new(),
                    place: Box::new(lhs),
                    arrow_token: arrow_token,
                    value: Box::new(rhs),
                });
            } else if Precedence::Range >= base && input.peek(syn::Token![..]) {
                let limits: syn::RangeLimits = input.parse()?;
                let rhs = if input.is_empty()
                    || input.peek(syn::Token![,])
                    || input.peek(syn::Token![;])
                    || !allow_struct.0 && input.peek(syn::token::Brace)
                {
                    None
                } else {
                    let mut rhs = unary_expr(input, allow_struct)?;
                    loop {
                        let next = peek_precedence(input);
                        if next > Precedence::Range {
                            rhs = parse_expr(input, rhs, allow_struct, next)?;
                        } else {
                            break;
                        }
                    }
                    Some(rhs)
                };
                lhs = Expr::Range(ExprRange {
                    attrs: Vec::new(),
                    from: Some(Box::new(lhs)),
                    limits: limits,
                    to: rhs.map(Box::new),
                });
            } else if Precedence::Cast >= base && input.peek(syn::Token![as]) {
                let as_token: syn::Token![as] = input.parse()?;
                let ty = input.call(syn::Type::without_plus)?;
                lhs = Expr::Cast(ExprCast {
                    attrs: Vec::new(),
                    expr: Box::new(lhs),
                    as_token: as_token,
                    ty: Box::new(ty),
                });
            } else if Precedence::Cast >= base && input.peek(syn::Token![:]) && !input.peek(syn::Token![::]) {
                let colon_token: syn::Token![:] = input.parse()?;
                let ty = input.call(syn::Type::without_plus)?;
                lhs = Expr::Type(ExprType {
                    attrs: Vec::new(),
                    expr: Box::new(lhs),
                    colon_token: colon_token,
                    ty: Box::new(ty),
                });
            } else {
                break;
            }
        }
        Ok(lhs)
    }

    #[cfg(not(feature = "full"))]
    fn parse_expr(
        input: ParseStream,
        mut lhs: Expr,
        allow_struct: AllowStruct,
        base: Precedence,
    ) -> Result<Expr> {
        loop {
            if input
                .fork()
                .parse::<syn::BinOp>()
                .ok()
                .map_or(false, |op| Precedence::of(&op) >= base)
            {
                let op: syn::BinOp = input.parse()?;
                let precedence = Precedence::of(&op);
                let mut rhs = unary_expr(input, allow_struct)?;
                loop {
                    let next = peek_precedence(input);
                    if next > precedence || next == precedence && precedence == Precedence::Assign {
                        rhs = parse_expr(input, rhs, allow_struct, next)?;
                    } else {
                        break;
                    }
                }
                lhs = Expr::Binary(ExprBinary {
                    attrs: Vec::new(),
                    left: Box::new(lhs),
                    op: op,
                    right: Box::new(rhs),
                });
            } else if Precedence::Cast >= base && input.peek(syn::Token![as]) {
                let as_token: syn::Token![as] = input.parse()?;
                let ty = input.call(Type::without_plus)?;
                lhs = Expr::Cast(ExprCast {
                    attrs: Vec::new(),
                    expr: Box::new(lhs),
                    as_token: as_token,
                    ty: Box::new(ty),
                });
            } else {
                break;
            }
        }
        Ok(lhs)
    }

    fn peek_precedence(input: ParseStream) -> Precedence {
        if let Ok(op) = input.fork().parse() {
            Precedence::of(&op)
        } else if input.peek(syn::Token![=]) && !input.peek(syn::Token![=>]) {
            Precedence::Assign
        } else if input.peek(syn::Token![<-]) {
            Precedence::Placement
        } else if input.peek(syn::Token![..]) {
            Precedence::Range
        } else if input.peek(syn::Token![as]) || input.peek(syn::Token![:]) && !input.peek(syn::Token![::]) {
            Precedence::Cast
        } else {
            Precedence::Any
        }
    }

    // Parse an arbitrary expression.
    fn ambiguous_expr(input: ParseStream, allow_struct: AllowStruct) -> Result<Expr> {
        let lhs = unary_expr(input, allow_struct)?;
        parse_expr(input, lhs, allow_struct, Precedence::Any)
    }

    // <UnOp> <trailer>
    // & <trailer>
    // &mut <trailer>
    // box <trailer>
    #[cfg(feature = "full")]
    fn unary_expr(input: ParseStream, allow_struct: AllowStruct) -> Result<Expr> {
        let ahead = input.fork();
        ahead.call(syn::Attribute::parse_outer)?;
        if ahead.peek(syn::Token![&])
            || ahead.peek(syn::Token![box])
            || ahead.peek(syn::Token![*])
            || ahead.peek(syn::Token![!])
            || ahead.peek(syn::Token![-])
        {
            let attrs = input.call(syn::Attribute::parse_outer)?;
            if input.peek(syn::Token![&]) {
                Ok(Expr::Reference(ExprReference {
                    attrs: attrs,
                    and_token: input.parse()?,
                    mutability: input.parse()?,
                    expr: Box::new(unary_expr(input, allow_struct)?),
                }))
            } else if input.peek(syn::Token![box]) {
                Ok(Expr::Box(ExprBox {
                    attrs: attrs,
                    box_token: input.parse()?,
                    expr: Box::new(unary_expr(input, allow_struct)?),
                }))
            } else {
                Ok(Expr::Unary(ExprUnary {
                    attrs: attrs,
                    op: input.parse()?,
                    expr: Box::new(unary_expr(input, allow_struct)?),
                }))
            }
        } else {
            trailer_expr(input, allow_struct)
        }
    }

    #[cfg(not(feature = "full"))]
    fn unary_expr(input: ParseStream, allow_struct: AllowStruct) -> Result<Expr> {
        let ahead = input.fork();
        ahead.call(syn::Attribute::parse_outer)?;
        if ahead.peek(syn::Token![*]) || ahead.peek(syn::Token![!]) || ahead.peek(syn::Token![-]) {
            Ok(Expr::Unary(ExprUnary {
                attrs: input.call(syn::Attribute::parse_outer)?,
                op: input.parse()?,
                expr: Box::new(unary_expr(input, allow_struct)?),
            }))
        } else {
            trailer_expr(input, allow_struct)
        }
    }

    // <atom> (..<args>) ...
    // <atom> . <ident> (..<args>) ...
    // <atom> . <ident> ...
    // <atom> . <lit> ...
    // <atom> [ <expr> ] ...
    // <atom> ? ...
    #[cfg(feature = "full")]
    fn trailer_expr(input: ParseStream, allow_struct: AllowStruct) -> Result<Expr> {
        if input.peek(syn::token::Group) {
            return input.call(expr_group).map(Expr::Group);
        }

        let outer_attrs = input.call(syn::Attribute::parse_outer)?;

        let atom = atom_expr(input, allow_struct)?;
        let mut e = trailer_helper(input, atom)?;

        let inner_attrs = e.replace_attrs(Vec::new());
        let attrs = syn::private::attrs(outer_attrs, inner_attrs);
        e.replace_attrs(attrs);
        Ok(e)
    }

    #[cfg(feature = "full")]
    fn trailer_helper(input: ParseStream, mut e: Expr) -> Result<Expr> {
        loop {
            if input.peek(syn::token::Paren) {
                let content;
                e = Expr::Call(ExprCall {
                    attrs: Vec::new(),
                    func: Box::new(e),
                    paren_token: syn::parenthesized!(content in input),
                    args: content.parse_terminated(Expr::parse)?,
                });
            } else if input.peek(syn::Token![.]) && !input.peek(syn::Token![..]) {
                let dot_token: syn::Token![.] = input.parse()?;
                let member: Member = input.parse()?;
                let turbofish = if member.is_named() && input.peek(syn::Token![::]) {
                    Some(MethodTurbofish {
                        colon2_token: input.parse()?,
                        lt_token: input.parse()?,
                        args: {
                            let mut args = Punctuated::new();
                            loop {
                                if input.peek(syn::Token![>]) {
                                    break;
                                }
                                let value = input.call(generic_method_argument)?;
                                args.push_value(value);
                                if input.peek(syn::Token![>]) {
                                    break;
                                }
                                let punct = input.parse()?;
                                args.push_punct(punct);
                            }
                            args
                        },
                        gt_token: input.parse()?,
                    })
                } else {
                    None
                };

                if turbofish.is_some() || input.peek(syn::token::Paren) {
                    if let Member::Named(method) = member {
                        let content;
                        e = Expr::MethodCall(ExprMethodCall {
                            attrs: Vec::new(),
                            receiver: Box::new(e),
                            dot_token: dot_token,
                            method: method,
                            turbofish: turbofish,
                            paren_token: syn::parenthesized!(content in input),
                            args: content.parse_terminated(Expr::parse)?,
                        });
                        continue;
                    }
                }

                e = Expr::Field(ExprField {
                    attrs: Vec::new(),
                    base: Box::new(e),
                    dot_token: dot_token,
                    member: member,
                });
            } else if input.peek(syn::token::Bracket) {
                let content;
                e = Expr::Index(ExprIndex {
                    attrs: Vec::new(),
                    expr: Box::new(e),
                    bracket_token: syn::bracketed!(content in input),
                    index: content.parse()?,
                });
            } else if input.peek(syn::Token![?]) {
                e = Expr::Try(ExprTry {
                    attrs: Vec::new(),
                    expr: Box::new(e),
                    question_token: input.parse()?,
                });
            } else if input.peek(syn::Token![::]) {
                e = turboball::parse_turboball(input, e)?;
            } else {
                break;
            }
        }
        Ok(e)
    }

    #[cfg(not(feature = "full"))]
    fn trailer_expr(input: ParseStream, allow_struct: AllowStruct) -> Result<Expr> {
        let mut e = atom_expr(input, allow_struct)?;

        loop {
            if input.peek(syn::token::Paren) {
                let content;
                e = Expr::Call(ExprCall {
                    attrs: Vec::new(),
                    func: Box::new(e),
                    paren_token: parenthesized!(content in input),
                    args: content.parse_terminated(Expr::parse)?,
                });
            } else if input.peek(syn::Token![.]) {
                e = Expr::Field(ExprField {
                    attrs: Vec::new(),
                    base: Box::new(e),
                    dot_token: input.parse()?,
                    member: input.parse()?,
                });
            } else if input.peek(syn::token::Bracket) {
                let content;
                e = Expr::Index(ExprIndex {
                    attrs: Vec::new(),
                    expr: Box::new(e),
                    bracket_token: bracketed!(content in input),
                    index: content.parse()?,
                });
            } else {
                break;
            }
        }

        Ok(e)
    }

    // Parse all atomic expressions which don't have to worry about precedence
    // interactions, as they are fully contained.
    #[cfg(feature = "full")]
    fn atom_expr(input: ParseStream, allow_struct: AllowStruct) -> Result<Expr> {
        if input.peek(syn::token::Group) {
            input.call(expr_group).map(Expr::Group)
        } else if input.peek(syn::Lit) {
            input.parse().map(Expr::Lit)
        } else if input.peek(syn::Token![async])
            && (input.peek2(syn::token::Brace) || input.peek2(syn::Token![move]) && input.peek3(syn::token::Brace))
        {
            input.call(expr_async).map(Expr::Async)
        } else if input.peek(syn::Token![try]) && input.peek2(syn::token::Brace) {
            input.call(expr_try_block).map(Expr::TryBlock)
        } else if input.peek(syn::Token![|])
            || input.peek(syn::Token![async]) && (input.peek2(syn::Token![|]) || input.peek2(syn::Token![move]))
            || input.peek(syn::Token![static])
            || input.peek(syn::Token![move])
        {
            expr_closure(input, allow_struct).map(Expr::Closure)
        } else if input.peek(Ident)
            || (
                input.peek(syn::Token![::])
                && !input.peek3(syn::token::Paren)
               )
            || input.peek(syn::Token![<])
            || input.peek(syn::Token![self])
            || input.peek(syn::Token![Self])
            || input.peek(syn::Token![super])
            || input.peek(syn::Token![extern])
            || input.peek(syn::Token![crate])
        {
            path_or_macro_or_struct(input, allow_struct)
        } else if input.peek(syn::token::Paren) {
            paren_or_tuple(input)
        } else if input.peek(syn::Token![break]) {
            expr_break(input, allow_struct).map(Expr::Break)
        } else if input.peek(syn::Token![continue]) {
            input.call(expr_continue).map(Expr::Continue)
        } else if input.peek(syn::Token![return]) {
            expr_ret(input, allow_struct).map(Expr::Return)
        } else if input.peek(syn::token::Bracket) {
            array_or_repeat(input)
        } else if input.peek(syn::Token![let]) {
            input.call(expr_let).map(Expr::Let)
        } else if input.peek(syn::Token![if]) {
            input.parse().map(Expr::If)
        } else if input.peek(syn::Token![while]) {
            input.parse().map(Expr::While)
        } else if input.peek(syn::Token![for]) {
            input.parse().map(Expr::ForLoop)
        } else if input.peek(syn::Token![loop]) {
            input.parse().map(Expr::Loop)
        } else if input.peek(syn::Token![match]) {
            input.parse().map(Expr::Match)
        } else if input.peek(syn::Token![yield]) {
            input.call(expr_yield).map(Expr::Yield)
        } else if input.peek(syn::Token![unsafe]) {
            input.call(expr_unsafe).map(Expr::Unsafe)
        } else if input.peek(syn::token::Brace) {
            input.call(expr_block).map(Expr::Block)
        } else if input.peek(syn::Token![..]) {
            expr_range(input, allow_struct).map(Expr::Range)
        } else if input.peek(syn::Lifetime) {
            let the_label: syn::Label = input.parse()?;
            let mut expr = if input.peek(syn::Token![while]) {
                Expr::While(input.parse()?)
            } else if input.peek(syn::Token![for]) {
                Expr::ForLoop(input.parse()?)
            } else if input.peek(syn::Token![loop]) {
                Expr::Loop(input.parse()?)
            } else if input.peek(syn::token::Brace) {
                Expr::Block(input.call(expr_block)?)
            } else {
                return Err(input.error("expected loop or block expression"));
            };
            match expr {
                Expr::While(ExprWhile { ref mut label, .. })
                | Expr::ForLoop(ExprForLoop { ref mut label, .. })
                | Expr::Loop(ExprLoop { ref mut label, .. })
                | Expr::Block(ExprBlock { ref mut label, .. }) => *label = Some(the_label),
                _ => unreachable!(),
            }
            Ok(expr)
        } else {
            Err(input.error("expected expression"))
        }
    }

    #[cfg(not(feature = "full"))]
    fn atom_expr(input: ParseStream, _allow_struct: AllowStruct) -> Result<Expr> {
        if input.peek(Lit) {
            input.parse().map(Expr::Lit)
        } else if input.peek(syn::token::Paren) {
            input.call(expr_paren).map(Expr::Paren)
        } else if input.peek(Ident)
            || input.peek(syn::Token![::])
            || input.peek(syn::Token![<])
            || input.peek(syn::Token![self])
            || input.peek(syn::Token![Self])
            || input.peek(syn::Token![super])
            || input.peek(syn::Token![extern])
            || input.peek(syn::Token![crate])
        {
            input.parse().map(Expr::Path)
        } else {
            Err(input.error("unsupported expression; enable syn's features=[\"full\"]"))
        }
    }

    #[cfg(feature = "full")]
    fn path_or_macro_or_struct(input: ParseStream, allow_struct: AllowStruct) -> Result<Expr> {
        let expr: ExprPath = input.parse()?;
        if expr.qself.is_some() {
            return Ok(Expr::Path(expr));
        }

        if input.peek(syn::Token![!]) && !input.peek(syn::Token![!=]) {
            let mut contains_arguments = false;
            for segment in &expr.path.segments {
                match segment.arguments {
                    syn::PathArguments::None => {}
                    syn::PathArguments::AngleBracketed(_) | syn::PathArguments::Parenthesized(_) => {
                        contains_arguments = true;
                    }
                }
            }

            if !contains_arguments {
                let bang_token: syn::Token![!] = input.parse()?;
                let (delimiter, tts) = syn::mac::parse_delimiter(input)?;
                return Ok(Expr::Macro(ExprMacro {
                    attrs: Vec::new(),
                    mac: crate::resyn::Macro {
                        path: expr.path,
                        bang_token: bang_token,
                        delimiter: delimiter,
                        tts: tts,
                    },
                }));
            }
        }

        if allow_struct.0 && input.peek(syn::token::Brace) {
            let outer_attrs = Vec::new();
            expr_struct_helper(input, outer_attrs, expr.path).map(Expr::Struct)
        } else {
            Ok(Expr::Path(expr))
        }
    }

    #[cfg(feature = "full")]
    fn paren_or_tuple(input: ParseStream) -> Result<Expr> {
        let content;
        let paren_token = syn::parenthesized!(content in input);
        let inner_attrs = content.call(syn::Attribute::parse_inner)?;
        if content.is_empty() {
            return Ok(Expr::Tuple(ExprTuple {
                attrs: inner_attrs,
                paren_token: paren_token,
                elems: Punctuated::new(),
            }));
        }

        let first: Expr = content.parse()?;
        if content.is_empty() {
            return Ok(Expr::Paren(ExprParen {
                attrs: inner_attrs,
                paren_token: paren_token,
                expr: Box::new(first),
            }));
        }

        let mut elems = Punctuated::new();
        elems.push_value(first);
        while !content.is_empty() {
            let punct = content.parse()?;
            elems.push_punct(punct);
            if content.is_empty() {
                break;
            }
            let value = content.parse()?;
            elems.push_value(value);
        }
        Ok(Expr::Tuple(ExprTuple {
            attrs: inner_attrs,
            paren_token: paren_token,
            elems: elems,
        }))
    }

    #[cfg(feature = "full")]
    fn array_or_repeat(input: ParseStream) -> Result<Expr> {
        let content;
        let bracket_token = syn::bracketed!(content in input);
        let inner_attrs = content.call(syn::Attribute::parse_inner)?;
        if content.is_empty() {
            return Ok(Expr::Array(ExprArray {
                attrs: inner_attrs,
                bracket_token: bracket_token,
                elems: Punctuated::new(),
            }));
        }

        let first: Expr = content.parse()?;
        if content.is_empty() || content.peek(syn::Token![,]) {
            let mut elems = Punctuated::new();
            elems.push_value(first);
            while !content.is_empty() {
                let punct = content.parse()?;
                elems.push_punct(punct);
                if content.is_empty() {
                    break;
                }
                let value = content.parse()?;
                elems.push_value(value);
            }
            Ok(Expr::Array(ExprArray {
                attrs: inner_attrs,
                bracket_token: bracket_token,
                elems: elems,
            }))
        } else if content.peek(syn::Token![;]) {
            let semi_token: syn::Token![;] = content.parse()?;
            let len: Expr = content.parse()?;
            Ok(Expr::Repeat(ExprRepeat {
                attrs: inner_attrs,
                bracket_token: bracket_token,
                expr: Box::new(first),
                semi_token: semi_token,
                len: Box::new(len),
            }))
        } else {
            Err(content.error("expected `,` or `;`"))
        }
    }

    #[cfg(feature = "full")]
    fn expr_early(input: ParseStream) -> Result<Expr> {
        let mut attrs = input.call(syn::Attribute::parse_outer)?;
        let mut expr = if input.peek(syn::Token![if]) {
            Expr::If(input.parse()?)
        } else if input.peek(syn::Token![while]) {
            Expr::While(input.parse()?)
        } else if input.peek(syn::Token![for]) {
            Expr::ForLoop(input.parse()?)
        } else if input.peek(syn::Token![loop]) {
            Expr::Loop(input.parse()?)
        } else if input.peek(syn::Token![match]) {
            Expr::Match(input.parse()?)
        } else if input.peek(syn::Token![try]) && input.peek2(syn::token::Brace) {
            Expr::TryBlock(input.call(expr_try_block)?)
        } else if input.peek(syn::Token![unsafe]) {
            Expr::Unsafe(input.call(expr_unsafe)?)
        } else if input.peek(syn::token::Brace) {
            Expr::Block(input.call(expr_block)?)
        } else {
            let allow_struct = AllowStruct(true);
            let mut expr = unary_expr(input, allow_struct)?;

            attrs.extend(expr.replace_attrs(Vec::new()));
            expr.replace_attrs(attrs);

            return parse_expr(input, expr, allow_struct, Precedence::Any);
        };

        if input.peek(syn::Token![.])
        || input.peek(syn::Token![?])
        || (
            input.peek(syn::Token![::])
            && input.peek3(syn::token::Paren)
           ) {
            expr = trailer_helper(input, expr)?;

            attrs.extend(expr.replace_attrs(Vec::new()));
            expr.replace_attrs(attrs);

            let allow_struct = AllowStruct(true);
            return parse_expr(input, expr, allow_struct, Precedence::Any);
        }

        attrs.extend(expr.replace_attrs(Vec::new()));
        expr.replace_attrs(attrs);
        Ok(expr)
    }

    impl Parse for ExprLit {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(ExprLit {
                attrs: Vec::new(),
                lit: input.parse()?,
            })
        }
    }

    #[cfg(feature = "full")]
    fn expr_group(input: ParseStream) -> Result<ExprGroup> {
        let group = syn::private::parse_group(input)?;
        Ok(ExprGroup {
            attrs: Vec::new(),
            group_token: group.token,
            expr: group.content.parse()?,
        })
    }

    #[cfg(not(feature = "full"))]
    fn expr_paren(input: ParseStream) -> Result<ExprParen> {
        let content;
        Ok(ExprParen {
            attrs: Vec::new(),
            paren_token: parenthesized!(content in input),
            expr: content.parse()?,
        })
    }

    #[cfg(feature = "full")]
    fn generic_method_argument(input: ParseStream) -> Result<GenericMethodArgument> {
        // TODO parse const generics as well
        input.parse().map(GenericMethodArgument::Type)
    }

    #[cfg(feature = "full")]
    fn expr_let(input: ParseStream) -> Result<ExprLet> {
        Ok(ExprLet {
            attrs: Vec::new(),
            let_token: input.parse()?,
            pats: {
                let mut pats = Punctuated::new();
                input.parse::<Option<syn::Token![|]>>()?;
                let value: syn::Pat = input.parse()?;
                pats.push_value(value);
                while input.peek(syn::Token![|]) && !input.peek(syn::Token![||]) && !input.peek(syn::Token![|=]) {
                    let punct = input.parse()?;
                    pats.push_punct(punct);
                    let value: syn::Pat = input.parse()?;
                    pats.push_value(value);
                }
                pats
            },
            eq_token: input.parse()?,
            expr: Box::new(input.call(expr_no_struct)?),
        })
    }

    #[cfg(feature = "full")]
    impl Parse for ExprIf {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(ExprIf {
                attrs: Vec::new(),
                if_token: input.parse()?,
                cond: Box::new(input.call(expr_no_struct)?),
                then_branch: input.parse()?,
                else_branch: {
                    if input.peek(syn::Token![else]) {
                        Some(input.call(else_block)?)
                    } else {
                        None
                    }
                },
            })
        }
    }

    #[cfg(feature = "full")]
    pub fn else_block(input: ParseStream) -> Result<(syn::Token![else], Box<Expr>)> {
        let else_token: syn::Token![else] = input.parse()?;

        let lookahead = input.lookahead1();
        let else_branch = if input.peek(syn::Token![if]) {
            input.parse().map(Expr::If)?
        } else if input.peek(syn::token::Brace) {
            Expr::Block(ExprBlock {
                attrs: Vec::new(),
                label: None,
                block: input.parse()?,
            })
        } else {
            return Err(lookahead.error());
        };

        Ok((else_token, Box::new(else_branch)))
    }

    #[cfg(feature = "full")]
    impl Parse for ExprForLoop {
        fn parse(input: ParseStream) -> Result<Self> {
            let label: Option<syn::Label> = input.parse()?;
            let for_token: syn::Token![for] = input.parse()?;
            let pat: syn::Pat = input.parse()?;
            let in_token: syn::Token![in] = input.parse()?;
            let expr: Expr = input.call(expr_no_struct)?;

            let content;
            let brace_token = syn::braced!(content in input);
            let inner_attrs = content.call(syn::Attribute::parse_inner)?;
            let stmts = content.call(Block::parse_within)?;

            Ok(ExprForLoop {
                attrs: inner_attrs,
                label: label,
                for_token: for_token,
                pat: Box::new(pat),
                in_token: in_token,
                expr: Box::new(expr),
                body: Block {
                    brace_token: brace_token,
                    stmts: stmts,
                },
            })
        }
    }

    #[cfg(feature = "full")]
    impl Parse for ExprLoop {
        fn parse(input: ParseStream) -> Result<Self> {
            let label: Option<syn::Label> = input.parse()?;
            let loop_token: syn::Token![loop] = input.parse()?;

            let content;
            let brace_token = syn::braced!(content in input);
            let inner_attrs = content.call(syn::Attribute::parse_inner)?;
            let stmts = content.call(Block::parse_within)?;

            Ok(ExprLoop {
                attrs: inner_attrs,
                label: label,
                loop_token: loop_token,
                body: Block {
                    brace_token: brace_token,
                    stmts: stmts,
                },
            })
        }
    }

    #[cfg(feature = "full")]
    impl Parse for ExprMatch {
        fn parse(input: ParseStream) -> Result<Self> {
            let match_token: syn::Token![match] = input.parse()?;
            let expr = expr_no_struct(input)?;

            let content;
            let brace_token = syn::braced!(content in input);
            let inner_attrs = content.call(syn::Attribute::parse_inner)?;

            let mut arms = Vec::new();
            while !content.is_empty() {
                arms.push(content.call(Arm::parse)?);
            }

            Ok(ExprMatch {
                attrs: inner_attrs,
                match_token: match_token,
                expr: Box::new(expr),
                brace_token: brace_token,
                arms: arms,
            })
        }
    }

    macro_rules! impl_by_parsing_expr {
        (
            $(
                $expr_type:ty, $variant:ident, $msg:expr,
            )*
        ) => {
            $(
                #[cfg(all(feature = "full", feature = "printing"))]
                impl Parse for $expr_type {
                    fn parse(input: ParseStream) -> Result<Self> {
                        let mut expr: Expr = input.parse()?;
                        loop {
                            match expr {
                                Expr::$variant(inner) => return Ok(inner),
                                Expr::Group(next) => expr = *next.expr,
                                _ => return Err(syn::Error::new_spanned(expr, $msg)),
                            }
                        }
                    }
                }
            )*
        };
    }

    impl_by_parsing_expr! {
        ExprBox, Box, "expected box expression",
        ExprInPlace, InPlace, "expected placement expression",
        ExprArray, Array, "expected slice literal expression",
        ExprCall, Call, "expected function call expression",
        ExprMethodCall, MethodCall, "expected method call expression",
        ExprTuple, Tuple, "expected tuple expression",
        ExprBinary, Binary, "expected binary operation",
        ExprUnary, Unary, "expected unary operation",
        ExprCast, Cast, "expected cast expression",
        ExprType, Type, "expected type ascription expression",
        ExprLet, Let, "expected let guard",
        ExprClosure, Closure, "expected closure expression",
        ExprUnsafe, Unsafe, "expected unsafe block",
        ExprBlock, Block, "expected blocked scope",
        ExprAssign, Assign, "expected assignment expression",
        ExprAssignOp, AssignOp, "expected compound assignment expression",
        ExprField, Field, "expected struct field access",
        ExprIndex, Index, "expected indexing expression",
        ExprRange, Range, "expected range expression",
        ExprReference, Reference, "expected referencing operation",
        ExprBreak, Break, "expected break expression",
        ExprContinue, Continue, "expected continue expression",
        ExprReturn, Return, "expected return expression",
        ExprMacro, Macro, "expected macro invocation expression",
        ExprStruct, Struct, "expected struct literal expression",
        ExprRepeat, Repeat, "expected array literal constructed from one repeated element",
        ExprParen, Paren, "expected parenthesized expression",
        ExprTry, Try, "expected try expression",
        ExprAsync, Async, "expected async block",
        ExprTryBlock, TryBlock, "expected try block",
        ExprYield, Yield, "expected yield expression",
    }

    #[cfg(feature = "full")]
    fn expr_try_block(input: ParseStream) -> Result<ExprTryBlock> {
        Ok(ExprTryBlock {
            attrs: Vec::new(),
            try_token: input.parse()?,
            block: input.parse()?,
        })
    }

    #[cfg(feature = "full")]
    fn expr_yield(input: ParseStream) -> Result<ExprYield> {
        Ok(ExprYield {
            attrs: Vec::new(),
            yield_token: input.parse()?,
            expr: {
                if !input.is_empty() && !input.peek(syn::Token![,]) && !input.peek(syn::Token![;]) {
                    Some(input.parse()?)
                } else {
                    None
                }
            },
        })
    }

    #[cfg(feature = "full")]
    fn expr_closure(input: ParseStream, allow_struct: AllowStruct) -> Result<ExprClosure> {
        let asyncness: Option<syn::Token![async]> = input.parse()?;
        let movability: Option<syn::Token![static]> = if asyncness.is_none() {
            input.parse()?
        } else {
            None
        };
        let capture: Option<syn::Token![move]> = input.parse()?;
        let or1_token: syn::Token![|] = input.parse()?;

        let mut inputs = Punctuated::new();
        loop {
            if input.peek(syn::Token![|]) {
                break;
            }
            let value = fn_arg(input)?;
            inputs.push_value(value);
            if input.peek(syn::Token![|]) {
                break;
            }
            let punct: syn::Token![,] = input.parse()?;
            inputs.push_punct(punct);
        }

        let or2_token: syn::Token![|] = input.parse()?;

        let (output, body) = if input.peek(syn::Token![->]) {
            let arrow_token: syn::Token![->] = input.parse()?;
            let ty: syn::Type = input.parse()?;
            let body: Block = input.parse()?;
            let output = syn::ReturnType::Type(arrow_token, Box::new(ty));
            let block = Expr::Block(ExprBlock {
                attrs: Vec::new(),
                label: None,
                block: body,
            });
            (output, block)
        } else {
            let body = ambiguous_expr(input, allow_struct)?;
            (syn::ReturnType::Default, body)
        };

        Ok(ExprClosure {
            attrs: Vec::new(),
            asyncness: asyncness,
            movability: movability,
            capture: capture,
            or1_token: or1_token,
            inputs: inputs,
            or2_token: or2_token,
            output: output,
            body: Box::new(body),
        })
    }

    #[cfg(feature = "full")]
    fn expr_async(input: ParseStream) -> Result<ExprAsync> {
        Ok(ExprAsync {
            attrs: Vec::new(),
            async_token: input.parse()?,
            capture: input.parse()?,
            block: input.parse()?,
        })
    }

    #[cfg(feature = "full")]
    fn fn_arg(input: ParseStream) -> Result<syn::FnArg> {
        let pat: syn::Pat = input.parse()?;

        if input.peek(syn::Token![:]) {
            Ok(syn::FnArg::Captured(syn::ArgCaptured {
                pat: pat,
                colon_token: input.parse()?,
                ty: input.parse()?,
            }))
        } else {
            Ok(syn::FnArg::Inferred(pat))
        }
    }

    #[cfg(feature = "full")]
    impl Parse for ExprWhile {
        fn parse(input: ParseStream) -> Result<Self> {
            let label: Option<syn::Label> = input.parse()?;
            let while_token: syn::Token![while] = input.parse()?;
            let cond = expr_no_struct(input)?;

            let content;
            let brace_token = syn::braced!(content in input);
            let inner_attrs = content.call(syn::Attribute::parse_inner)?;
            let stmts = content.call(Block::parse_within)?;

            Ok(ExprWhile {
                attrs: inner_attrs,
                label: label,
                while_token: while_token,
                cond: Box::new(cond),
                body: Block {
                    brace_token: brace_token,
                    stmts: stmts,
                },
            })
        }
    }

    /*
    #[cfg(feature = "full")]
    impl Parse for Label {
        fn parse(input: ParseStream) -> Result<Self> {
            Ok(Label {
                name: input.parse()?,
                colon_token: input.parse()?,
            })
        }
    }
    */

    /*
    #[cfg(feature = "full")]
    impl Parse for Option<Label> {
        fn parse(input: ParseStream) -> Result<Self> {
            if input.peek(Lifetime) {
                input.parse().map(Some)
            } else {
                Ok(None)
            }
        }
    }
    */

    #[cfg(feature = "full")]
    fn expr_continue(input: ParseStream) -> Result<ExprContinue> {
        Ok(ExprContinue {
            attrs: Vec::new(),
            continue_token: input.parse()?,
            label: input.parse()?,
        })
    }

    #[cfg(feature = "full")]
    fn expr_break(input: ParseStream, allow_struct: AllowStruct) -> Result<ExprBreak> {
        Ok(ExprBreak {
            attrs: Vec::new(),
            break_token: input.parse()?,
            label: input.parse()?,
            expr: {
                if input.is_empty()
                    || input.peek(syn::Token![,])
                    || input.peek(syn::Token![;])
                    || !allow_struct.0 && input.peek(syn::token::Brace)
                {
                    None
                } else {
                    let expr = ambiguous_expr(input, allow_struct)?;
                    Some(Box::new(expr))
                }
            },
        })
    }

    #[cfg(feature = "full")]
    fn expr_ret(input: ParseStream, allow_struct: AllowStruct) -> Result<ExprReturn> {
        Ok(ExprReturn {
            attrs: Vec::new(),
            return_token: input.parse()?,
            expr: {
                if input.is_empty() || input.peek(syn::Token![,]) || input.peek(syn::Token![;]) {
                    None
                } else {
                    // NOTE: return is greedy and eats blocks after it even when in a
                    // position where structs are not allowed, such as in if statement
                    // conditions. For example:
                    //
                    // if return { println!("A") } {} // Prints "A"
                    let expr = ambiguous_expr(input, allow_struct)?;
                    Some(Box::new(expr))
                }
            },
        })
    }

    #[cfg(feature = "full")]
    impl Parse for FieldValue {
        fn parse(input: ParseStream) -> Result<Self> {
            let member: Member = input.parse()?;
            let (colon_token, value) = if input.peek(syn::Token![:]) || !member.is_named() {
                let colon_token: syn::Token![:] = input.parse()?;
                let value: Expr = input.parse()?;
                (Some(colon_token), value)
            } else if let Member::Named(ref ident) = member {
                let value = Expr::Path(ExprPath {
                    attrs: Vec::new(),
                    qself: None,
                    path: syn::Path::from(ident.clone()),
                });
                (None, value)
            } else {
                unreachable!()
            };

            Ok(FieldValue {
                attrs: Vec::new(),
                member: member,
                colon_token: colon_token,
                expr: value,
            })
        }
    }

    #[cfg(feature = "full")]
    fn expr_struct_helper(
        input: ParseStream,
        outer_attrs: Vec<syn::Attribute>,
        path: syn::Path,
    ) -> Result<ExprStruct> {
        let content;
        let brace_token = syn::braced!(content in input);
        let inner_attrs = content.call(syn::Attribute::parse_inner)?;

        let mut fields = Punctuated::new();
        loop {
            let attrs = content.call(syn::Attribute::parse_outer)?;
            if content.fork().parse::<Member>().is_err() {
                if attrs.is_empty() {
                    break;
                } else {
                    return Err(content.error("expected struct field"));
                }
            }

            fields.push(FieldValue {
                attrs: attrs,
                ..content.parse()?
            });

            if !content.peek(syn::Token![,]) {
                break;
            }
            let punct: syn::Token![,] = content.parse()?;
            fields.push_punct(punct);
        }

        let (dot2_token, rest) = if fields.empty_or_trailing() && content.peek(syn::Token![..]) {
            let dot2_token: syn::Token![..] = content.parse()?;
            let rest: Expr = content.parse()?;
            (Some(dot2_token), Some(Box::new(rest)))
        } else {
            (None, None)
        };

        Ok(ExprStruct {
            attrs: syn::private::attrs(outer_attrs, inner_attrs),
            brace_token: brace_token,
            path: path,
            fields: fields,
            dot2_token: dot2_token,
            rest: rest,
        })
    }

    #[cfg(feature = "full")]
    fn expr_unsafe(input: ParseStream) -> Result<ExprUnsafe> {
        let unsafe_token: syn::Token![unsafe] = input.parse()?;

        let content;
        let brace_token = syn::braced!(content in input);
        let inner_attrs = content.call(syn::Attribute::parse_inner)?;
        let stmts = content.call(Block::parse_within)?;

        Ok(ExprUnsafe {
            attrs: inner_attrs,
            unsafe_token: unsafe_token,
            block: Block {
                brace_token: brace_token,
                stmts: stmts,
            },
        })
    }

    #[cfg(feature = "full")]
    pub fn expr_block(input: ParseStream) -> Result<ExprBlock> {
        let label: Option<syn::Label> = input.parse()?;
        let content;
        let brace_token = syn::braced!(content in input);
        let inner_attrs = content.call(syn::Attribute::parse_inner)?;
        let stmts = content.call(Block::parse_within)?;

        Ok(ExprBlock {
            attrs: inner_attrs,
            label: label,
            block: Block {
                brace_token: brace_token,
                stmts: stmts,
            },
        })
    }

    #[cfg(feature = "full")]
    fn expr_range(input: ParseStream, allow_struct: AllowStruct) -> Result<ExprRange> {
        Ok(ExprRange {
            attrs: Vec::new(),
            from: None,
            limits: input.parse()?,
            to: {
                if input.is_empty()
                    || input.peek(syn::Token![,])
                    || input.peek(syn::Token![;])
                    || !allow_struct.0 && input.peek(syn::token::Brace)
                {
                    None
                } else {
                    let to = ambiguous_expr(input, allow_struct)?;
                    Some(Box::new(to))
                }
            },
        })
    }

    #[cfg(feature = "full")]
    impl Parse for RangeLimits {
        fn parse(input: ParseStream) -> Result<Self> {
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Token![..=]) {
                input.parse().map(RangeLimits::Closed)
            } else if lookahead.peek(syn::Token![...]) {
                let dot3: syn::Token![...] = input.parse()?;
                Ok(RangeLimits::Closed(syn::Token![..=](dot3.spans)))
            } else if lookahead.peek(syn::Token![..]) {
                input.parse().map(RangeLimits::HalfOpen)
            } else {
                Err(lookahead.error())
            }
        }
    }

    // modification of syn::path::parse_helper
    fn parse_helper(input: ParseStream, expr_style: bool) -> Result<syn::Path> {
        if input.peek(syn::Token![dyn]) {
            return Err(input.error("expected path"));
        }

        Ok(syn::Path {
            leading_colon: input.parse()?,
            segments: {
                let mut segments = syn::punctuated::Punctuated::new();
                let value = syn::PathSegment::parse_helper(input, expr_style)?;
                segments.push_value(value);
                while input.peek(syn::Token![::])
                    && !input.peek3(syn::token::Paren) {
                    let punct: syn::Token![::] = input.parse()?;
                    segments.push_punct(punct);
                    let value = syn::PathSegment::parse_helper(input, expr_style)?;
                    segments.push_value(value);
                }
                segments
            },
        })
    }

    impl Parse for ExprPath {
        fn parse(input: ParseStream) -> Result<Self> {
            #[cfg(not(feature = "full"))]
            let attrs = Vec::new();
            #[cfg(feature = "full")]
            let attrs = input.call(syn::Attribute::parse_outer)?;

            let (qself, path) = if !input.peek(syn::Token![<]) {
                let path = parse_helper(input, true)?;
                (None, path)
            } else {
                syn::path::parsing::qpath(input, true)?
            };


            Ok(ExprPath {
                attrs: attrs,
                qself: qself,
                path: path,
            })
        }
    }

    #[cfg(feature = "full")]
    impl Parse for Block {
        fn parse(input: ParseStream) -> Result<Self> {
            let content;
            Ok(Block {
                brace_token: syn::braced!(content in input),
                stmts: content.call(Block::parse_within)?,
            })
        }
    }

    #[cfg(feature = "full")]
    impl Block {
        /// Parse the body of a block as zero or more statements, possibly
        /// including one trailing expression.
        ///
        /// *This function is available if Syn is built with the `"parsing"`
        /// feature.*
        ///
        /// # Example
        ///
        /// ```ignore
        /// use syn::{braced, token, syn::Attribute, Block, Ident, Result, Stmt, Token};
        /// use syn::parse::{Parse, ParseStream};
        ///
        /// // Parse a function with no generics or parameter list.
        /// //
        /// //     fn playground {
        /// //         let mut x = 1;
        /// //         x += 1;
        /// //         println!("{}", x);
        /// //     }
        /// struct MiniFunction {
        ///     attrs: Vec<syn::Attribute>,
        ///     fn_token: syn::Token![fn],
        ///     name: Ident,
        ///     brace_token: syn::token::Brace,
        ///     stmts: Vec<Stmt>,
        /// }
        ///
        /// impl Parse for MiniFunction {
        ///     fn parse(input: ParseStream) -> Result<Self> {
        ///         let outer_attrs = input.call(syn::Attribute::parse_outer)?;
        ///         let fn_token: syn::Token![fn] = input.parse()?;
        ///         let name: Ident = input.parse()?;
        ///
        ///         let content;
        ///         let brace_token = braced!(content in input);
        ///         let inner_attrs = content.call(syn::Attribute::parse_inner)?;
        ///         let stmts = content.call(Block::parse_within)?;
        ///
        ///         Ok(MiniFunction {
        ///             attrs: {
        ///                 let mut attrs = outer_attrs;
        ///                 attrs.extend(inner_attrs);
        ///                 attrs
        ///             },
        ///             fn_token: fn_token,
        ///             name: name,
        ///             brace_token: brace_token,
        ///             stmts: stmts,
        ///         })
        ///     }
        /// }
        /// ```
        pub fn parse_within(input: ParseStream) -> Result<Vec<Stmt>> {
            let mut stmts = Vec::new();
            loop {
                while input.peek(syn::Token![;]) {
                    input.parse::<syn::Token![;]>()?;
                }
                if input.is_empty() {
                    break;
                }
                let s = parse_stmt(input, true)?;
                let requires_semicolon = if let Stmt::Expr(ref s) = s {
                    requires_terminator(s)
                } else {
                    false
                };
                stmts.push(s);
                if input.is_empty() {
                    break;
                } else if requires_semicolon {
                    return Err(input.error("unexpected token"));
                }
            }
            Ok(stmts)
        }
    }

    #[cfg(feature = "full")]
    impl Parse for Stmt {
        fn parse(input: ParseStream) -> Result<Self> {
            parse_stmt(input, false)
        }
    }

    #[cfg(feature = "full")]
    fn parse_stmt(input: ParseStream, allow_nosemi: bool) -> Result<Stmt> {
        let ahead = input.fork();
        ahead.call(syn::Attribute::parse_outer)?;

        if {
            let ahead = ahead.fork();
            // Only parse braces here; paren and bracket will get parsed as
            // expression statements
            ahead.call(syn::Path::parse_mod_style).is_ok()
                && ahead.parse::<syn::Token![!]>().is_ok()
                && (ahead.peek(syn::token::Brace) || ahead.peek(Ident))
        } {
            stmt_mac(input)
        } else if ahead.peek(syn::Token![let]) {
            stmt_local(input).map(Stmt::Local)
        } else if ahead.peek(syn::Token![pub])
            || ahead.peek(syn::Token![crate]) && !ahead.peek2(syn::Token![::])
            || ahead.peek(syn::Token![extern]) && !ahead.peek2(syn::Token![::])
            || ahead.peek(syn::Token![use])
            || ahead.peek(syn::Token![static]) && (ahead.peek2(syn::Token![mut]) || ahead.peek2(Ident))
            || ahead.peek(syn::Token![const])
            || ahead.peek(syn::Token![unsafe]) && !ahead.peek2(syn::token::Brace)
            || ahead.peek(syn::Token![async]) && (ahead.peek2(syn::Token![extern]) || ahead.peek2(syn::Token![fn]))
            || ahead.peek(syn::Token![fn])
            || ahead.peek(syn::Token![mod])
            || ahead.peek(syn::Token![type])
            || ahead.peek(syn::Token![existential]) && ahead.peek2(syn::Token![type])
            || ahead.peek(syn::Token![struct])
            || ahead.peek(syn::Token![enum])
            || ahead.peek(syn::Token![union]) && ahead.peek2(Ident)
            || ahead.peek(syn::Token![auto]) && ahead.peek2(syn::Token![trait])
            || ahead.peek(syn::Token![trait])
            || ahead.peek(syn::Token![default])
                && (ahead.peek2(syn::Token![unsafe]) || ahead.peek2(syn::Token![impl]))
            || ahead.peek(syn::Token![impl])
            || ahead.peek(syn::Token![macro])
        {
            input.parse().map(Stmt::Item)
        } else {
            stmt_expr(input, allow_nosemi)
        }
    }

    #[cfg(feature = "full")]
    fn stmt_mac(input: ParseStream) -> Result<Stmt> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let path = input.call(syn::Path::parse_mod_style)?;
        let bang_token: syn::Token![!] = input.parse()?;
        let ident: Option<Ident> = input.parse()?;
        let (delimiter, tts) = syn::mac::parse_delimiter(input)?;
        let semi_token: Option<syn::Token![;]> = input.parse()?;

        Ok(Stmt::Item(syn::Item::Macro(syn::ItemMacro {
            attrs: attrs,
            ident: ident,
            mac: syn::Macro {
                path: path,
                bang_token: bang_token,
                delimiter: delimiter,
                tts: tts,
            },
            semi_token: semi_token,
        })))
    }

    #[cfg(feature = "full")]
    fn stmt_local(input: ParseStream) -> Result<Local> {
        Ok(Local {
            attrs: input.call(syn::Attribute::parse_outer)?,
            let_token: input.parse()?,
            pats: {
                let mut pats = Punctuated::new();
                let value: Pat = input.parse()?;
                pats.push_value(value);
                while input.peek(syn::Token![|]) && !input.peek(syn::Token![||]) && !input.peek(syn::Token![|=]) {
                    let punct = input.parse()?;
                    pats.push_punct(punct);
                    let value: Pat = input.parse()?;
                    pats.push_value(value);
                }
                pats
            },
            ty: {
                if input.peek(syn::Token![:]) {
                    let colon_token: syn::Token![:] = input.parse()?;
                    let ty: syn::Type = input.parse()?;
                    Some((colon_token, Box::new(ty)))
                } else {
                    None
                }
            },
            init: {
                if input.peek(syn::Token![=]) {
                    let eq_token: syn::Token![=] = input.parse()?;
                    let init: Expr = input.parse()?;
                    Some((eq_token, Box::new(init)))
                } else {
                    None
                }
            },
            semi_token: input.parse()?,
        })
    }

    #[cfg(feature = "full")]
    fn stmt_expr(input: ParseStream, allow_nosemi: bool) -> Result<Stmt> {
        let mut attrs = input.call(syn::Attribute::parse_outer)?;
        let mut e = expr_early(input)?;

        attrs.extend(e.replace_attrs(Vec::new()));
        e.replace_attrs(attrs);

        if input.peek(syn::Token![;]) {
            return Ok(Stmt::Semi(e, input.parse()?));
        }

        if allow_nosemi || !requires_terminator(&e) {
            Ok(Stmt::Expr(e))
        } else {
            Err(input.error("expected semicolon"))
        }
    }

    #[cfg(feature = "full")]
    impl Parse for Pat {
        fn parse(input: ParseStream) -> Result<Self> {
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Token![_]) {
                input.call(pat_wild).map(Pat::Wild)
            } else if lookahead.peek(syn::Token![box]) {
                input.call(pat_box).map(Pat::Box)
            } else if lookahead.peek(syn::Token![-]) || lookahead.peek(syn::Lit) {
                pat_lit_or_range(input)
            } else if input.peek(Ident)
                && ({
                    input.peek2(syn::Token![::])
                        || input.peek2(syn::Token![!])
                        || input.peek2(syn::token::Brace)
                        || input.peek2(syn::token::Paren)
                        || input.peek2(syn::Token![..])
                            && !{
                                let ahead = input.fork();
                                ahead.parse::<Ident>()?;
                                ahead.parse::<RangeLimits>()?;
                                ahead.is_empty() || ahead.peek(syn::Token![,])
                            }
                })
                || input.peek(syn::Token![self]) && input.peek2(syn::Token![::])
                || input.peek(syn::Token![::])
                || input.peek(syn::Token![<])
                || input.peek(syn::Token![Self])
                || input.peek(syn::Token![super])
                || input.peek(syn::Token![extern])
                || input.peek(syn::Token![crate])
            {
                pat_path_or_macro_or_struct_or_range(input)
            } else if input.peek(syn::Token![ref])
                || input.peek(syn::Token![mut])
                || input.peek(syn::Token![self])
                || input.peek(Ident)
            {
                input.call(pat_ident).map(Pat::Ident)
            } else if lookahead.peek(syn::token::Paren) {
                input.call(pat_tuple).map(Pat::Tuple)
            } else if lookahead.peek(syn::Token![&]) {
                input.call(pat_ref).map(Pat::Ref)
            } else if lookahead.peek(syn::token::Bracket) {
                input.call(pat_slice).map(Pat::Slice)
            } else {
                Err(lookahead.error())
            }
        }
    }

    #[cfg(feature = "full")]
    fn pat_path_or_macro_or_struct_or_range(input: ParseStream) -> Result<Pat> {
        let (qself, path) = syn::path::parsing::qpath(input, true)?;

        if input.peek(syn::Token![..]) {
            return pat_range(input, qself, path).map(Pat::Range);
        }

        if qself.is_some() {
            return Ok(Pat::Path(PatPath {
                qself: qself,
                path: path,
            }));
        }

        if input.peek(syn::Token![!]) && !input.peek(syn::Token![!=]) {
            let mut contains_arguments = false;
            for segment in &path.segments {
                match segment.arguments {
                    syn::PathArguments::None => {}
                    syn::PathArguments::AngleBracketed(_) | syn::PathArguments::Parenthesized(_) => {
                        contains_arguments = true;
                    }
                }
            }

            if !contains_arguments {
                let bang_token: syn::Token![!] = input.parse()?;
                let (delimiter, tts) = syn::mac::parse_delimiter(input)?;
                return Ok(Pat::Macro(PatMacro {
                    mac: syn::Macro {
                        path: path,
                        bang_token: bang_token,
                        delimiter: delimiter,
                        tts: tts,
                    },
                }));
            }
        }

        if input.peek(syn::token::Brace) {
            pat_struct(input, path).map(Pat::Struct)
        } else if input.peek(syn::token::Paren) {
            pat_tuple_struct(input, path).map(Pat::TupleStruct)
        } else if input.peek(syn::Token![..]) {
            pat_range(input, qself, path).map(Pat::Range)
        } else {
            Ok(Pat::Path(PatPath {
                qself: qself,
                path: path,
            }))
        }
    }

    #[cfg(feature = "full")]
    fn pat_wild(input: ParseStream) -> Result<PatWild> {
        Ok(PatWild {
            underscore_token: input.parse()?,
        })
    }

    #[cfg(feature = "full")]
    fn pat_box(input: ParseStream) -> Result<PatBox> {
        Ok(PatBox {
            box_token: input.parse()?,
            pat: input.parse()?,
        })
    }

    #[cfg(feature = "full")]
    fn pat_ident(input: ParseStream) -> Result<PatIdent> {
        Ok(PatIdent {
            by_ref: input.parse()?,
            mutability: input.parse()?,
            ident: input.call(Ident::parse_any)?,
            subpat: {
                if input.peek(syn::Token![@]) {
                    let at_token: syn::Token![@] = input.parse()?;
                    let subpat: Pat = input.parse()?;
                    Some((at_token, Box::new(subpat)))
                } else {
                    None
                }
            },
        })
    }

    #[cfg(feature = "full")]
    fn pat_tuple_struct(input: ParseStream, path: syn::Path) -> Result<PatTupleStruct> {
        Ok(PatTupleStruct {
            path: path,
            pat: input.call(pat_tuple)?,
        })
    }

    #[cfg(feature = "full")]
    fn pat_struct(input: ParseStream, path: syn::Path) -> Result<PatStruct> {
        let content;
        let brace_token = syn::braced!(content in input);

        let mut fields = Punctuated::new();
        while !content.is_empty() && !content.peek(syn::Token![..]) {
            let value = content.call(field_pat)?;
            fields.push_value(value);
            if !content.peek(syn::Token![,]) {
                break;
            }
            let punct: syn::Token![,] = content.parse()?;
            fields.push_punct(punct);
        }

        let dot2_token = if fields.empty_or_trailing() && content.peek(syn::Token![..]) {
            Some(content.parse()?)
        } else {
            None
        };

        Ok(PatStruct {
            path: path,
            brace_token: brace_token,
            fields: fields,
            dot2_token: dot2_token,
        })
    }

    #[cfg(feature = "full")]
    fn field_pat(input: ParseStream) -> Result<FieldPat> {
        let boxed: Option<syn::Token![box]> = input.parse()?;
        let by_ref: Option<syn::Token![ref]> = input.parse()?;
        let mutability: Option<syn::Token![mut]> = input.parse()?;
        let member: Member = input.parse()?;

        if boxed.is_none() && by_ref.is_none() && mutability.is_none() && input.peek(syn::Token![:])
            || member.is_unnamed()
        {
            return Ok(FieldPat {
                attrs: Vec::new(),
                member: member,
                colon_token: input.parse()?,
                pat: input.parse()?,
            });
        }

        let ident = match member {
            Member::Named(ident) => ident,
            Member::Unnamed(_) => unreachable!(),
        };

        let mut pat = syn::Pat::Ident(syn::PatIdent {
            by_ref: by_ref,
            mutability: mutability,
            ident: ident.clone(),
            subpat: None,
        });

        if let Some(boxed) = boxed {
            pat = syn::Pat::Box(syn::PatBox {
                pat: Box::new(pat),
                box_token: boxed,
            });
        }

        Ok(FieldPat {
            member: Member::Named(ident),
            pat: Box::new(pat),
            attrs: Vec::new(),
            colon_token: None,
        })
    }

    impl Parse for Member {
        fn parse(input: ParseStream) -> Result<Self> {
            if input.peek(syn::Ident) {
                input.parse().map(Member::Named)
            } else if input.peek(syn::LitInt) {
                input.parse().map(Member::Unnamed)
            } else {
                Err(input.error("expected identifier or integer"))
            }
        }
    }

    #[cfg(feature = "full")]
    impl Parse for Arm {
        fn parse(input: ParseStream) -> Result<Arm> {
            let requires_comma;
            Ok(Arm {
                attrs: input.call(syn::Attribute::parse_outer)?,
                leading_vert: input.parse()?,
                pats: {
                    let mut pats = Punctuated::new();
                    let value: Pat = input.parse()?;
                    pats.push_value(value);
                    loop {
                        if !input.peek(syn::Token![|]) {
                            break;
                        }
                        let punct = input.parse()?;
                        pats.push_punct(punct);
                        let value: Pat = input.parse()?;
                        pats.push_value(value);
                    }
                    pats
                },
                guard: {
                    if input.peek(syn::Token![if]) {
                        let if_token: syn::Token![if] = input.parse()?;
                        let guard: Expr = input.parse()?;
                        Some((if_token, Box::new(guard)))
                    } else {
                        None
                    }
                },
                fat_arrow_token: input.parse()?,
                body: {
                    let body = input.call(expr_early)?;
                    requires_comma = requires_terminator(&body);
                    Box::new(body)
                },
                comma: {
                    if requires_comma && !input.is_empty() {
                        Some(input.parse()?)
                    } else {
                        input.parse()?
                    }
                },
            })
        }
    }

    /*
    impl Parse for Index {
        fn parse(input: ParseStream) -> Result<Self> {
            let lit: LitInt = input.parse()?;
            if let IntSuffix::None = lit.suffix() {
                Ok(Index {
                    index: lit.value() as u32,
                    span: lit.span(),
                })
            } else {
                Err(Error::new(lit.span(), "expected unsuffixed integer"))
            }
        }
    }
    */

    #[cfg(feature = "full")]
    fn pat_range(input: ParseStream, qself: Option<syn::QSelf>, path: syn::Path) -> Result<PatRange> {
        Ok(PatRange {
            lo: Box::new(Expr::Path(ExprPath {
                attrs: Vec::new(),
                qself: qself,
                path: path,
            })),
            limits: input.parse()?,
            hi: input.call(pat_lit_expr)?,
        })
    }

    #[cfg(feature = "full")]
    fn pat_tuple(input: ParseStream) -> Result<PatTuple> {
        let content;
        let paren_token = syn::parenthesized!(content in input);

        let mut front = syn::punctuated::Punctuated::new();
        let mut dot2_token = None::<syn::Token![..]>;
        let mut comma_token = None::<syn::Token![,]>;
        loop {
            if content.is_empty() {
                break;
            }
            if content.peek(syn::Token![..]) {
                dot2_token = Some(content.parse()?);
                comma_token = content.parse()?;
                break;
            }
            let value: syn::Pat = content.parse()?;
            front.push_value(value);
            if content.is_empty() {
                break;
            }
            let punct = content.parse()?;
            front.push_punct(punct);
        }

        let mut back = syn::punctuated::Punctuated::new();
        while !content.is_empty() {
            let value: syn::Pat = content.parse()?;
            back.push_value(value);
            if content.is_empty() {
                break;
            }
            let punct = content.parse()?;
            back.push_punct(punct);
        }

        Ok(PatTuple {
            paren_token: paren_token,
            front: front,
            dot2_token: dot2_token,
            comma_token: comma_token,
            back: back,
        })
    }

    #[cfg(feature = "full")]
    fn pat_ref(input: ParseStream) -> Result<PatRef> {
        Ok(PatRef {
            and_token: input.parse()?,
            mutability: input.parse()?,
            pat: input.parse()?,
        })
    }

    #[cfg(feature = "full")]
    fn pat_lit_or_range(input: ParseStream) -> Result<Pat> {
        let lo = input.call(pat_lit_expr)?;
        if input.peek(syn::Token![..]) {
            Ok(Pat::Range(PatRange {
                lo: lo,
                limits: input.parse()?,
                hi: input.call(pat_lit_expr)?,
            }))
        } else {
            Ok(Pat::Lit(PatLit { expr: lo }))
        }
    }

    #[cfg(feature = "full")]
    fn pat_lit_expr(input: ParseStream) -> Result<Box<Expr>> {
        let neg: Option<syn::Token![-]> = input.parse()?;

        let lookahead = input.lookahead1();
        let expr = if lookahead.peek(syn::Lit) {
            Expr::Lit(input.parse()?)
        } else if lookahead.peek(Ident)
            || lookahead.peek(syn::Token![::])
            || lookahead.peek(syn::Token![<])
            || lookahead.peek(syn::Token![self])
            || lookahead.peek(syn::Token![Self])
            || lookahead.peek(syn::Token![super])
            || lookahead.peek(syn::Token![extern])
            || lookahead.peek(syn::Token![crate])
        {
            Expr::Path(input.parse()?)
        } else {
            return Err(lookahead.error());
        };

        Ok(Box::new(if let Some(neg) = neg {
            Expr::Unary(ExprUnary {
                attrs: Vec::new(),
                op: syn::UnOp::Neg(neg),
                expr: Box::new(expr),
            })
        } else {
            expr
        }))
    }

    #[cfg(feature = "full")]
    fn pat_slice(input: ParseStream) -> Result<PatSlice> {
        let content;
        let bracket_token = syn::bracketed!(content in input);

        let mut front = syn::punctuated::Punctuated::new();
        let mut middle = None;
        loop {
            if content.is_empty() || content.peek(syn::Token![..]) {
                break;
            }
            let value: Pat = content.parse()?;
            if content.peek(syn::Token![..]) {
                middle = Some(Box::new(value));
                break;
            }
            front.push_value(value);
            if content.is_empty() {
                break;
            }
            let punct = content.parse()?;
            front.push_punct(punct);
        }

        let dot2_token: Option<syn::Token![..]> = content.parse()?;
        let mut comma_token = None::<syn::Token![,]>;
        let mut back = Punctuated::new();
        if dot2_token.is_some() {
            comma_token = content.parse()?;
            if comma_token.is_some() {
                loop {
                    if content.is_empty() {
                        break;
                    }
                    let value: Pat = content.parse()?;
                    back.push_value(value);
                    if content.is_empty() {
                        break;
                    }
                    let punct = content.parse()?;
                    back.push_punct(punct);
                }
            }
        }

        Ok(PatSlice {
            bracket_token: bracket_token,
            front: front,
            middle: middle,
            dot2_token: dot2_token,
            comma_token: comma_token,
            back: back,
        })
    }

    #[cfg(feature = "full")]
    impl Member {
        fn is_named(&self) -> bool {
            match *self {
                Member::Named(_) => true,
                Member::Unnamed(_) => false,
            }
        }

        fn is_unnamed(&self) -> bool {
            match *self {
                Member::Named(_) => false,
                Member::Unnamed(_) => true,
            }
        }
    }
}

#[cfg(feature = "printing")]
mod printing {
    use super::*;

    use proc_macro2::{Literal, TokenStream};
    use quote::{ToTokens, TokenStreamExt};

    #[cfg(feature = "full")]
    use syn::attr::FilterAttrs;
    #[cfg(feature = "full")]
    use syn::print::TokensOrDefault;

    // If the given expression is a bare `ExprStruct`, wraps it in parenthesis
    // before appending it to `TokenStream`.
    #[cfg(feature = "full")]
    pub fn wrap_bare_struct(tokens: &mut TokenStream, e: &Expr) {
        if let Expr::Struct(_) = *e {
            syn::token::Paren::default().surround(tokens, |tokens| {
                e.to_tokens(tokens);
            });
        } else {
            e.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    pub fn outer_attrs_to_tokens(attrs: &[syn::Attribute], tokens: &mut TokenStream) {
        tokens.append_all(attrs.outer());
    }

    #[cfg(feature = "full")]
    pub fn inner_attrs_to_tokens(attrs: &[syn::Attribute], tokens: &mut TokenStream) {
        tokens.append_all(attrs.inner());
    }

    #[cfg(not(feature = "full"))]
    pub fn outer_attrs_to_tokens(_attrs: &[syn::Attribute], _tokens: &mut TokenStream) {}

    #[cfg(not(feature = "full"))]
    pub fn inner_attrs_to_tokens(_attrs: &[syn::Attribute], _tokens: &mut TokenStream) {}

    #[cfg(feature = "full")]
    impl ToTokens for ExprBox {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.box_token.to_tokens(tokens);
            self.expr.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprInPlace {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.place.to_tokens(tokens);
            self.arrow_token.to_tokens(tokens);
            self.value.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprArray {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.bracket_token.surround(tokens, |tokens| {
                inner_attrs_to_tokens(&self.attrs, tokens);
                self.elems.to_tokens(tokens);
            })
        }
    }

    impl ToTokens for ExprCall {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.func.to_tokens(tokens);
            self.paren_token.surround(tokens, |tokens| {
                self.args.to_tokens(tokens);
            })
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprMethodCall {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.receiver.to_tokens(tokens);
            self.dot_token.to_tokens(tokens);
            self.method.to_tokens(tokens);
            self.turbofish.to_tokens(tokens);
            self.paren_token.surround(tokens, |tokens| {
                self.args.to_tokens(tokens);
            });
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for MethodTurbofish {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.colon2_token.to_tokens(tokens);
            self.lt_token.to_tokens(tokens);
            self.args.to_tokens(tokens);
            self.gt_token.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for GenericMethodArgument {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match *self {
                GenericMethodArgument::Type(ref t) => t.to_tokens(tokens),
                GenericMethodArgument::Const(ref c) => c.to_tokens(tokens),
            }
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprTuple {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.paren_token.surround(tokens, |tokens| {
                inner_attrs_to_tokens(&self.attrs, tokens);
                self.elems.to_tokens(tokens);
                // If we only have one argument, we need a trailing comma to
                // distinguish ExprTuple from ExprParen.
                if self.elems.len() == 1 && !self.elems.trailing_punct() {
                    <syn::Token![,]>::default().to_tokens(tokens);
                }
            })
        }
    }

    impl ToTokens for ExprBinary {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.left.to_tokens(tokens);
            self.op.to_tokens(tokens);
            self.right.to_tokens(tokens);
        }
    }

    impl ToTokens for ExprUnary {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.op.to_tokens(tokens);
            self.expr.to_tokens(tokens);
        }
    }

    impl ToTokens for ExprLit {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.lit.to_tokens(tokens);
        }
    }

    impl ToTokens for ExprCast {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.expr.to_tokens(tokens);
            self.as_token.to_tokens(tokens);
            self.ty.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprType {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.expr.to_tokens(tokens);
            self.colon_token.to_tokens(tokens);
            self.ty.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    pub fn maybe_wrap_else(tokens: &mut TokenStream, else_: &Option<(syn::Token![else], Box<Expr>)>) {
        if let Some((ref else_token, ref else_)) = *else_ {
            else_token.to_tokens(tokens);

            // If we are not one of the valid expressions to exist in an else
            // clause, wrap ourselves in a block.
            match **else_ {
                Expr::If(_) | Expr::Block(_) => {
                    else_.to_tokens(tokens);
                }
                _ => {
                    syn::token::Brace::default().surround(tokens, |tokens| {
                        else_.to_tokens(tokens);
                    });
                }
            }
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprLet {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.let_token.to_tokens(tokens);
            self.pats.to_tokens(tokens);
            self.eq_token.to_tokens(tokens);
            wrap_bare_struct(tokens, &self.expr);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprIf {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.if_token.to_tokens(tokens);
            wrap_bare_struct(tokens, &self.cond);
            self.then_branch.to_tokens(tokens);
            maybe_wrap_else(tokens, &self.else_branch);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprWhile {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.label.to_tokens(tokens);
            self.while_token.to_tokens(tokens);
            wrap_bare_struct(tokens, &self.cond);
            self.body.brace_token.surround(tokens, |tokens| {
                inner_attrs_to_tokens(&self.attrs, tokens);
                tokens.append_all(&self.body.stmts);
            });
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprForLoop {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.label.to_tokens(tokens);
            self.for_token.to_tokens(tokens);
            self.pat.to_tokens(tokens);
            self.in_token.to_tokens(tokens);
            wrap_bare_struct(tokens, &self.expr);
            self.body.brace_token.surround(tokens, |tokens| {
                inner_attrs_to_tokens(&self.attrs, tokens);
                tokens.append_all(&self.body.stmts);
            });
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprLoop {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.label.to_tokens(tokens);
            self.loop_token.to_tokens(tokens);
            self.body.brace_token.surround(tokens, |tokens| {
                inner_attrs_to_tokens(&self.attrs, tokens);
                tokens.append_all(&self.body.stmts);
            });
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprMatch {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.match_token.to_tokens(tokens);
            wrap_bare_struct(tokens, &self.expr);
            self.brace_token.surround(tokens, |tokens| {
                inner_attrs_to_tokens(&self.attrs, tokens);
                for (i, arm) in self.arms.iter().enumerate() {
                    arm.to_tokens(tokens);
                    // Ensure that we have a comma after a non-block arm, except
                    // for the last one.
                    let is_last = i == self.arms.len() - 1;
                    if !is_last && requires_terminator(&arm.body) && arm.comma.is_none() {
                        <syn::Token![,]>::default().to_tokens(tokens);
                    }
                }
            });
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprAsync {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.async_token.to_tokens(tokens);
            self.capture.to_tokens(tokens);
            self.block.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprTryBlock {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.try_token.to_tokens(tokens);
            self.block.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprYield {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.yield_token.to_tokens(tokens);
            self.expr.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprClosure {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.asyncness.to_tokens(tokens);
            self.movability.to_tokens(tokens);
            self.capture.to_tokens(tokens);
            self.or1_token.to_tokens(tokens);
            for input in self.inputs.pairs() {
                match **input.value() {
                    syn::FnArg::Captured(syn::ArgCaptured {
                        ref pat,
                        ty: syn::Type::Infer(_),
                        ..
                    }) => {
                        pat.to_tokens(tokens);
                    }
                    _ => input.value().to_tokens(tokens),
                }
                input.punct().to_tokens(tokens);
            }
            self.or2_token.to_tokens(tokens);
            self.output.to_tokens(tokens);
            self.body.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprUnsafe {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.unsafe_token.to_tokens(tokens);
            self.block.brace_token.surround(tokens, |tokens| {
                inner_attrs_to_tokens(&self.attrs, tokens);
                tokens.append_all(&self.block.stmts);
            });
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprBlock {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.label.to_tokens(tokens);
            self.block.brace_token.surround(tokens, |tokens| {
                inner_attrs_to_tokens(&self.attrs, tokens);
                tokens.append_all(&self.block.stmts);
            });
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprAssign {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.left.to_tokens(tokens);
            self.eq_token.to_tokens(tokens);
            self.right.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprAssignOp {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.left.to_tokens(tokens);
            self.op.to_tokens(tokens);
            self.right.to_tokens(tokens);
        }
    }

    impl ToTokens for ExprField {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.base.to_tokens(tokens);
            self.dot_token.to_tokens(tokens);
            self.member.to_tokens(tokens);
        }
    }

    impl ToTokens for Member {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match *self {
                Member::Named(ref ident) => ident.to_tokens(tokens),
                Member::Unnamed(ref index) => index.to_tokens(tokens),
            }
        }
    }

    impl ToTokens for Index {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            let mut lit = Literal::i64_unsuffixed(i64::from(self.index));
            lit.set_span(self.span);
            tokens.append(lit);
        }
    }

    impl ToTokens for ExprIndex {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.expr.to_tokens(tokens);
            self.bracket_token.surround(tokens, |tokens| {
                self.index.to_tokens(tokens);
            });
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprRange {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.from.to_tokens(tokens);
            match self.limits {
                syn::RangeLimits::HalfOpen(ref t) => t.to_tokens(tokens),
                syn::RangeLimits::Closed(ref t) => t.to_tokens(tokens),
            }
            self.to.to_tokens(tokens);
        }
    }

    impl ToTokens for ExprPath {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            syn::private::print_path(tokens, &self.qself, &self.path);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprReference {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.and_token.to_tokens(tokens);
            self.mutability.to_tokens(tokens);
            self.expr.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprBreak {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.break_token.to_tokens(tokens);
            self.label.to_tokens(tokens);
            self.expr.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprContinue {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.continue_token.to_tokens(tokens);
            self.label.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprReturn {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.return_token.to_tokens(tokens);
            self.expr.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprMacro {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.mac.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.path.to_tokens(tokens);
            self.brace_token.surround(tokens, |tokens| {
                inner_attrs_to_tokens(&self.attrs, tokens);
                self.fields.to_tokens(tokens);
                if self.rest.is_some() {
                    TokensOrDefault(&self.dot2_token).to_tokens(tokens);
                    self.rest.to_tokens(tokens);
                }
            })
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprRepeat {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.bracket_token.surround(tokens, |tokens| {
                inner_attrs_to_tokens(&self.attrs, tokens);
                self.expr.to_tokens(tokens);
                self.semi_token.to_tokens(tokens);
                self.len.to_tokens(tokens);
            })
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprGroup {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.group_token.surround(tokens, |tokens| {
                self.expr.to_tokens(tokens);
            });
        }
    }

    impl ToTokens for ExprParen {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.paren_token.surround(tokens, |tokens| {
                inner_attrs_to_tokens(&self.attrs, tokens);
                self.expr.to_tokens(tokens);
            });
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprTry {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.expr.to_tokens(tokens);
            self.question_token.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for ExprTurboball {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.expr_mark.to_tokens(tokens);
            self.expr.to_tokens(tokens);
            self.post_mark.to_tokens(tokens);
        }
    }

    impl ToTokens for ExprVerbatim {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.tts.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for Label {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.name.to_tokens(tokens);
            self.colon_token.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for FieldValue {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.member.to_tokens(tokens);
            if let Some(ref colon_token) = self.colon_token {
                colon_token.to_tokens(tokens);
                self.expr.to_tokens(tokens);
            }
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for Arm {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            tokens.append_all(&self.attrs);
            self.leading_vert.to_tokens(tokens);
            self.pats.to_tokens(tokens);
            if let Some((ref if_token, ref guard)) = self.guard {
                if_token.to_tokens(tokens);
                guard.to_tokens(tokens);
            }
            self.fat_arrow_token.to_tokens(tokens);
            self.body.to_tokens(tokens);
            self.comma.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for PatWild {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.underscore_token.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for PatIdent {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.by_ref.to_tokens(tokens);
            self.mutability.to_tokens(tokens);
            self.ident.to_tokens(tokens);
            if let Some((ref at_token, ref subpat)) = self.subpat {
                at_token.to_tokens(tokens);
                subpat.to_tokens(tokens);
            }
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for PatStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.path.to_tokens(tokens);
            self.brace_token.surround(tokens, |tokens| {
                self.fields.to_tokens(tokens);
                // NOTE: We need a comma before the dot2 token if it is present.
                if !self.fields.empty_or_trailing() && self.dot2_token.is_some() {
                    <syn::Token![,]>::default().to_tokens(tokens);
                }
                self.dot2_token.to_tokens(tokens);
            });
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for PatTupleStruct {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.path.to_tokens(tokens);
            self.pat.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for PatPath {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            syn::private::print_path(tokens, &self.qself, &self.path);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for PatTuple {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.paren_token.surround(tokens, |tokens| {
                self.front.to_tokens(tokens);
                if let Some(ref dot2_token) = self.dot2_token {
                    if !self.front.empty_or_trailing() {
                        // Ensure there is a comma before the .. token.
                        <syn::Token![,]>::default().to_tokens(tokens);
                    }
                    dot2_token.to_tokens(tokens);
                    self.comma_token.to_tokens(tokens);
                    if self.comma_token.is_none() && !self.back.is_empty() {
                        // Ensure there is a comma after the .. token.
                        <syn::Token![,]>::default().to_tokens(tokens);
                    }
                }
                self.back.to_tokens(tokens);
            });
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for PatBox {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.box_token.to_tokens(tokens);
            self.pat.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for PatRef {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.and_token.to_tokens(tokens);
            self.mutability.to_tokens(tokens);
            self.pat.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for PatLit {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.expr.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for PatRange {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.lo.to_tokens(tokens);
            match self.limits {
                syn::RangeLimits::HalfOpen(ref t) => t.to_tokens(tokens),
                syn::RangeLimits::Closed(ref t) => syn::Token![...](t.spans).to_tokens(tokens),
            }
            self.hi.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for PatSlice {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.bracket_token.surround(tokens, |tokens| {
                self.front.to_tokens(tokens);

                // If we need a comma before the middle or standalone .. token,
                // then make sure it's present.
                if !self.front.empty_or_trailing()
                    && (self.middle.is_some() || self.dot2_token.is_some())
                {
                    <syn::Token![,]>::default().to_tokens(tokens);
                }

                // If we have an identifier, we always need a .. token.
                if self.middle.is_some() {
                    self.middle.to_tokens(tokens);
                    TokensOrDefault(&self.dot2_token).to_tokens(tokens);
                } else if self.dot2_token.is_some() {
                    self.dot2_token.to_tokens(tokens);
                }

                // Make sure we have a comma before the back half.
                if !self.back.is_empty() {
                    TokensOrDefault(&self.comma_token).to_tokens(tokens);
                    self.back.to_tokens(tokens);
                } else {
                    self.comma_token.to_tokens(tokens);
                }
            })
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for PatMacro {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.mac.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for PatVerbatim {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.tts.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for FieldPat {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            if let Some(ref colon_token) = self.colon_token {
                self.member.to_tokens(tokens);
                colon_token.to_tokens(tokens);
            }
            self.pat.to_tokens(tokens);
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for Block {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.brace_token.surround(tokens, |tokens| {
                tokens.append_all(&self.stmts);
            });
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for Stmt {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match *self {
                Stmt::Local(ref local) => local.to_tokens(tokens),
                Stmt::Item(ref item) => item.to_tokens(tokens),
                Stmt::Expr(ref expr) => expr.to_tokens(tokens),
                Stmt::Semi(ref expr, ref semi) => {
                    expr.to_tokens(tokens);
                    semi.to_tokens(tokens);
                }
            }
        }
    }

    #[cfg(feature = "full")]
    impl ToTokens for Local {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            outer_attrs_to_tokens(&self.attrs, tokens);
            self.let_token.to_tokens(tokens);
            self.pats.to_tokens(tokens);
            if let Some((ref colon_token, ref ty)) = self.ty {
                colon_token.to_tokens(tokens);
                ty.to_tokens(tokens);
            }
            if let Some((ref eq_token, ref init)) = self.init {
                eq_token.to_tokens(tokens);
                init.to_tokens(tokens);
            }
            self.semi_token.to_tokens(tokens);
        }
    }
}
