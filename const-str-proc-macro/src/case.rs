use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitStr, Token};

enum Case {
    Lower,
    Upper,
    #[cfg(feature = "heck")]
    Camel,
    #[cfg(feature = "heck")]
    Snake,
    #[cfg(feature = "heck")]
    Kebab,
    #[cfg(feature = "heck")]
    ShoutySnake,
    #[cfg(feature = "heck")]
    ShoutyKebab,
}

impl Case {
    fn convert(&self, s: &str) -> String {
        #[cfg(feature = "heck")]
        use heck::{CamelCase, KebabCase, ShoutyKebabCase, ShoutySnakeCase, SnakeCase};
        match self {
            Case::Lower => s.to_lowercase(),
            Case::Upper => s.to_uppercase(),
            #[cfg(feature = "heck")]
            Case::Camel => s.to_camel_case(),
            #[cfg(feature = "heck")]
            Case::Snake => s.to_snake_case(),
            #[cfg(feature = "heck")]
            Case::Kebab => s.to_kebab_case(),
            #[cfg(feature = "heck")]
            Case::ShoutySnake => s.to_shouty_snake_case(),
            #[cfg(feature = "heck")]
            Case::ShoutyKebab => s.to_shouty_kebab_case(),
        }
    }
}

pub struct ConvertCase {
    case: Case,
    src: LitStr,
}

impl Parse for ConvertCase {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let case = input.parse::<Ident>()?.to_string();

        let case = match case.as_str() {
            "lower" => Case::Lower,
            "upper" => Case::Upper,
            #[cfg(feature = "heck")]
            "camel" => Case::Camel,
            #[cfg(feature = "heck")]
            "snake" => Case::Snake,
            #[cfg(feature = "heck")]
            "kebab" => Case::Kebab,
            #[cfg(feature = "heck")]
            "shouty_snake" => Case::ShoutySnake,
            #[cfg(feature = "heck")]
            "shouty_kebab" => Case::ShoutyKebab,
            _ => return Err(input.error("unsupported case")),
        };

        input.parse::<Token![,]>()?;

        let src = input.parse::<LitStr>()?;
        Ok(Self { case, src })
    }
}

impl ConvertCase {
    pub fn eval(&self) -> TokenStream {
        let src = self.src.value();
        let dst = self.case.convert(&src);
        let dst_token = LitStr::new(&dst, self.src.span());
        dst_token.into_token_stream().into()
    }
}
