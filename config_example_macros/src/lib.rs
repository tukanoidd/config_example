use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_crate::FoundCrate;
use proc_macro_error::{abort, abort_call_site, emit_warning, proc_macro_error};
use quote::{quote, ToTokens};
use syn::{
    braced, bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token, Expr, Ident, Token,
};

#[proc_macro_error]
#[proc_macro]
pub fn node(input: TokenStream) -> TokenStream {
    let crate_name = crate_name();

    let NodeMacro { ty, tabs, comments } = parse_macro_input!(input);

    let tabs = tabs.map(|tabs| quote!(.with_tabs(#tabs)));
    let comment_call = |comment| {
        let tokens = match comment {
            NodeComment::Line(line) => line.into_token_stream(),
            NodeComment::MultiLine(lines) => lines.into_token_stream(),
        };

        quote!(#crate_name::comment!(#tokens))
    };
    let comments = comments.map(|NodeComments { top, right }| {
        let top = top.map(|top| {
            let top_comment = comment_call(top);
            quote!(.with_top_comment(#top_comment))
        });
        let right = right.map(|right| {
            let right_comment = comment_call(right);
            quote!(.with_right_comment(#right_comment))
        });

        quote!(#top #right)
    });

    quote! {
        #crate_name::node::Node::new(#ty) #tabs #comments
    }
    .into()
}

struct NodeMacro {
    ty: Expr,
    tabs: Option<Expr>,
    comments: Option<NodeComments>,
}

impl NodeMacro {
    const OPTIONS: [&'static str; 2] = ["tabs", "comments"];
}

impl Parse for NodeMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty = input.parse()?;

        let (tabs, comments) = match input.peek(Token![,]) {
            true => {
                let _comma: token::Comma = input.parse()?;
                let mut tabs = None;
                let mut comments = None;

                while !input.is_empty() {
                    let ident: Ident = input.parse()?;

                    match ident.to_string().as_str() {
                        "tabs" => {
                            let _eq: Token![=] = input.parse()?;
                            tabs = Some(input.parse()?)
                        }
                        "comments" => {
                            let _eq: Token![=] = input.parse()?;
                            comments = Some(input.parse()?);
                        }
                        unknown => {
                            emit_warning_unknown_option(
                                ident.span(),
                                unknown,
                                NodeMacro::OPTIONS.as_slice(),
                            );
                        }
                    }

                    if input.peek(Token![,]) {
                        let _comma: token::Comma = input.parse()?;
                    }
                }

                (tabs, comments)
            }
            false => (None, None),
        };

        Ok(Self { ty, tabs, comments })
    }
}

struct NodeComments {
    top: Option<NodeComment>,
    right: Option<NodeComment>,
}

impl NodeComments {
    const OPTIONS: [&'static str; 2] = ["top", "right"];
}

impl Parse for NodeComments {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let data;
        let _brace = braced!(data in input);

        let options = Punctuated::<NodeCommentsFieldValue, Token![,]>::parse_terminated(&data)?;

        if options.is_empty() {
            abort!(
                data.span(),
                format!(
                    "Please provide at least one of the options: {:?}",
                    NodeComments::OPTIONS
                )
            )
        }

        let mut top = None;
        let mut right = None;

        for NodeCommentsFieldValue { ident, val } in options {
            match ident.to_string().as_str() {
                "top" => top = Some(val),
                "right" => right = Some(val),
                unknown => emit_warning_unknown_option(
                    ident.span(),
                    unknown,
                    NodeComments::OPTIONS.as_slice(),
                ),
            }
        }

        Ok(Self { top, right })
    }
}

struct NodeCommentsFieldValue {
    ident: Ident,
    val: NodeComment,
}

impl Parse for NodeCommentsFieldValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        let _colon: Token![:] = input.parse()?;
        let val = input.parse()?;

        Ok(Self { ident, val })
    }
}

enum NodeComment {
    Line(Expr),
    MultiLine(Punctuated<Expr, Token![,]>),
}

impl Parse for NodeComment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match input.peek(token::Bracket) {
            true => {
                let lines;
                let _bracket = bracketed!(lines in input);

                Ok(Self::MultiLine(Punctuated::parse_terminated(&lines)?))
            }
            false => Ok(Self::Line(input.parse()?)),
        }
    }
}

fn crate_name() -> Ident {
    match &proc_macro_crate::crate_name("config_example") {
        Ok(found) => Ident::new(
            match found {
                FoundCrate::Itself => "crate",
                FoundCrate::Name(name) => name,
            },
            Span::call_site(),
        ),
        Err(err) => abort_call_site!("{}", err),
    }
}

fn emit_warning_unknown_option(span: Span, option: impl std::fmt::Display, options: &[&str]) {
    emit_warning!(
        span,
        "Unknown option '{}' used. Available options: {:?}",
        option,
        options
    );
}
