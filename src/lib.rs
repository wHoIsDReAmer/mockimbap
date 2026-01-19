use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, Ident, ItemTrait, Result, Token,
};

struct MockableArg {
    name: Ident,
    value: Option<Expr>,
}

impl Parse for MockableArg {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let name: Ident = input.parse()?;
        if input.peek(Token![=]) {
            let _eq: Token![=] = input.parse()?;
            let value: Expr = input.parse()?;
            Ok(Self {
                name,
                value: Some(value),
            })
        } else {
            Ok(Self { name, value: None })
        }
    }
}

/// Create mock struct and implementation from trait.
#[proc_macro_attribute]
pub fn mockable(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_trait = parse_macro_input!(item as ItemTrait);
    let trait_ident = item_trait.ident.clone();

    let args = parse_macro_input!(attr with syn::punctuated::Punctuated<MockableArg, Token![,]>::parse_terminated);

    let mut mock_name: Option<Ident> = None;
    let mut return_map: std::collections::HashMap<String, Expr> = std::collections::HashMap::new();

    for arg in args {
        match arg.value {
            None => {
                if mock_name.is_some() {
                    return syn::Error::new(arg.name.span(), "duplicate mock struct name")
                        .to_compile_error()
                        .into();
                }
                mock_name = Some(arg.name);
            }
            Some(expr) => {
                return_map.insert(arg.name.to_string(), expr);
            }
        }
    }

    let mock_ident = mock_name.unwrap_or_else(|| format_ident!("Mock{}", trait_ident));

    let implementations = item_trait.items.iter().filter_map(|item| {
        let syn::TraitItem::Fn(method) = item else {
            return None;
        };
        let method_name = method.sig.ident.clone();
        let inputs = method.sig.inputs.clone();
        let output = method.sig.output.clone();

        if matches!(output, syn::ReturnType::Default) {
            let err = syn::Error::new(
                method.sig.ident.span(),
                "mockimbap only supports trait methods that return a value",
            )
            .to_compile_error();
            return Some(quote! { #err });
        }

        let return_expr = return_map
            .get(&method_name.to_string())
            .cloned()
            .unwrap_or_else(|| syn::parse_quote! { unimplemented!() });

        Some(quote! {
            fn #method_name(#inputs) #output {
                #return_expr
            }
        })
    });

    let expanded = quote! {
        #item_trait

        struct #mock_ident;

        impl #trait_ident for #mock_ident {
            #(#implementations)*
        }
    };

    expanded.into()
}
