extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(FromHashMap)]
pub fn from_hashmap_parser(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).expect("Can't parse rust code to AST");
    let gen = impl_from_hashmap(&ast);
    gen.parse().expect("Can't generate implementation code")
}

fn impl_from_hashmap(ast: &syn::MacroInput) -> quote::Tokens {
    let fields = match ast.body {
        syn::Body::Struct(ref data) => data.fields(),
        syn::Body::Enum(_) => panic!("#[derive(FromHashMap)] can only be used with structs"),
    };
    let field_names = fields.iter()
                            .filter_map(|field| field.ident.as_ref())
                            .collect::<Vec<_>>();

    let to_camel = |iden: &&syn::Ident| -> Option<String> {
        let name = iden.as_ref();
        let it = name.split('_');
        let mut ss: Vec<String> = Vec::new();
        for word in it {
            let mut v: Vec<char> = word.chars().collect();
            v[0] = v[0].to_uppercase().next().unwrap();
            let s: String = v.into_iter().collect();
            ss.push(s);
        }
        Some(ss.into_iter().collect::<String>().to_owned())
    };
    let tmp_field_names = field_names.clone();
    let field_names_camel = tmp_field_names.iter()
                                       .filter_map(to_camel)
                                       .collect::<Vec<_>>();

    let name = &ast.ident;

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let t = quote! {
        impl #impl_generics FromHashMap for #name #ty_generics #where_clause {
            fn from_hashmap(hm: &std::collections::HashMap<&str, &str>) -> #name {
                let unwrap_get = |x| hm.get(x).unwrap();
                #name {
                    #(#field_names: FromStr::from_str(unwrap_get(#field_names_camel)).unwrap()),*
                    ,..Default::default()
                }
            }
        }
    };

    println!("Debug: {}", t.as_str());

    t
}
