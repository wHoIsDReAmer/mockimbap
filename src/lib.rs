use proc_macro::TokenStream;

use syn::{parse_macro_input, ItemStruct, ItemTrait, Path};
use quote::{format_ident, quote, ToTokens};
use std::{collections::HashMap, sync::{Mutex, LazyLock}};

static RETURN_VALUES: LazyLock<Mutex<HashMap<String, String>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
// 트레잇 이름: 메소드 이름, 입력, 출력
static MOCK_MAP: LazyLock<Mutex<HashMap<String, Vec<TraitItemInfo>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

struct TraitItemInfo {
    method_name: String,
    inputs: Vec<String>,
    output: String,
}

/// 트레잇 정보를 컴파일 타임 때 저장하여 모킹할 수 있게 합니다.
#[proc_macro_attribute]
pub fn mockable(_: TokenStream, item: TokenStream) -> TokenStream {
    let item_return = item.clone();

    let item_trait = parse_macro_input!(item as ItemTrait);
    let trait_name = item_trait.ident.to_string();

    let mut mock_map = match MOCK_MAP.lock() {
        Ok(map) => map,
        Err(poisoned) => poisoned.into_inner(),
    };
    mock_map.insert(trait_name.clone(), Vec::new());

    let trait_items = item_trait.items;
    for item in trait_items {   
        if let syn::TraitItem::Fn(method) = item {
            let method_name = method.sig.ident.to_string();
            let inputs = method.sig.inputs.iter().map(|arg| arg.to_token_stream().to_string()).collect();
            let output = method.sig.output.to_token_stream().to_string();

            mock_map.entry(trait_name.clone())
                .or_insert(Vec::new())
                .push(TraitItemInfo { method_name, inputs, output });
        }
    }
    println!("mockable called");
    item_return
}

/// 트레잇 동작들을 가져와서 받은 구조체에 코드를 생성합니다.
#[proc_macro_attribute]
pub fn mock(trait_name: TokenStream, item: TokenStream) -> TokenStream {
    // 구조체 정보
    let ident = parse_macro_input!(item as ItemStruct);
    let ident_copy = ident.clone();

    let ident_name = ident.ident;
    let item_trait = parse_macro_input!(trait_name as Path);

    let mock_map = match MOCK_MAP.lock() {
        Ok(map) => map,
        Err(poisoned) => poisoned.into_inner(),
    };

    let trait_items = mock_map.get(&item_trait.to_token_stream().to_string()).expect("Trait not found in MOCK_MAP");
    
    // 각 메소드에 대한 구현 생성
    let implementations = trait_items.iter().map(|method| {
        let method_name = format_ident!("{}", method.method_name);
        // Vec<Ident> 대신 각 입력 파라미터를 개별적으로 처리
        let inputs = method.inputs.iter()
            .map(|input| syn::parse_str::<syn::FnArg>(input).unwrap())
            .collect::<Vec<_>>();
        let output = syn::parse_str::<syn::ReturnType>(&method.output).unwrap();
        
        
        // 저장된 반환값이 있는지 확인
        let key = format!("{}::{}", ident_name, method_name);

        let return_value = match RETURN_VALUES.lock() {
            Ok(map) => map.get(&key).cloned(),
            Err(poisoned) => poisoned.into_inner().get(&key).cloned(),
        };
        let return_value_str = return_value.unwrap_or("unimplemented!()".to_string());
        let return_value_expr = syn::parse_str::<syn::Expr>(&return_value_str).expect("Failed to parse return value");
        
        quote! {
            fn #method_name(#(#inputs),*) #output {
                #return_value_expr
            }
        }
    });

    let expanded = quote! {
        #ident_copy

        impl #item_trait for #ident_name {
            #(#implementations)*
        }
    };

    println!("mock called");

    expanded.into()
}

/// return_at 매크로의 인자를 파싱하는 헬퍼 함수
fn parse_return_at_args(attr: TokenStream) -> (String, String) {
    // 여기서 "foo, 1" 같은 형식의 인자를 파싱
    // 실제 구현에서는 더 견고한 파싱이 필요합니다
    let args = attr.to_string();
    let parts: Vec<&str> = args.split(',').collect();
    (parts[0].trim().to_string(), parts[1].trim().to_string())
}

#[proc_macro_attribute]
pub fn return_at(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_return = item.clone();

    let item_struct = parse_macro_input!(item as ItemStruct);
    let (method_name, return_value) = parse_return_at_args(attr);
    
    // 반환값을 저장
    let key = format!("{}::{}", item_struct.ident, method_name);
    match RETURN_VALUES.lock() {
        Ok(mut map) => map.insert(key, return_value),
        Err(poisoned) => poisoned.into_inner().insert(key, return_value),
    };

    item_return
}
