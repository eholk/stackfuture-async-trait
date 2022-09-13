use proc_macro2::Span;
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::{Expr, Token};

#[derive(Clone)]
pub struct Args {
    pub local: bool,
    pub stack_size: Expr,
}

mod kw {
    syn::custom_keyword!(Send);
}

impl Parse for Args {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        match try_parse(input) {
            Ok(args) if input.is_empty() => Ok(args),
            _ => Err(error()),
        }
    }
}

fn try_parse(input: ParseStream<'_>) -> Result<Args> {
    let stack_size = input.parse()?;

    if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
        input.parse::<Token![?]>()?;
        input.parse::<kw::Send>()?;
        Ok(Args {
            local: true,
            stack_size,
        })
    } else {
        Ok(Args {
            local: false,
            stack_size,
        })
    }
}

fn error() -> Error {
    let msg = "expected #[async_trait(SIZE)] or #[async_trait(SIZE, ?Send)]";
    Error::new(Span::call_site(), msg)
}
