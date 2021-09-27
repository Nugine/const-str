use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{LitStr, Token};

pub struct RegexAssertMatch {
    re: LitStr,
    text: LitStr,
}

impl Parse for RegexAssertMatch {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let re = input.parse::<LitStr>()?;
        let _ = input.parse::<Token![,]>()?;
        let text = input.parse::<LitStr>()?;
        Ok(Self { re, text })
    }
}

impl RegexAssertMatch {
    pub fn eval(&self) -> TokenStream {
        use regex::Regex;

        let re: Regex = match Regex::new(&self.re.value()) {
            Ok(re) => re,
            Err(e) => emit_error!(self.re, e.to_string()),
        };

        let text = self.text.value();

        if !re.is_match(&text) {
            emit_error!(self.text, "the string literal does not match the pattern")
        }

        TokenStream::new()
    }
}
