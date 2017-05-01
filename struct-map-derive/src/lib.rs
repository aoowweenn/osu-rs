extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(StructMap)]
pub fn struct_map_generator(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).expect("Can't parse rust code to AST");
    let gen = impl_struct_map(&ast);
    gen.parse().expect("Can't generate implementation code")
}

fn impl_struct_map(ast: &syn::MacroInput) -> quote::Tokens {
    let fields = match ast.body {
        syn::Body::Struct(ref data) => data.fields(),
        syn::Body::Enum(_) => panic!("#[derive(StructMap)] can only be used with structs"),
    };
    let field_indices = 0..fields.len();
    let ref field_names = fields.iter()
                            .filter_map(|field| field.ident.as_ref())
                            .collect::<Vec<_>>();
    let field_names_copy = field_names;

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
    let field_names_camel = field_names.iter()
                                       .filter_map(to_camel)
                                       .collect::<Vec<_>>();

    let name = &ast.ident;

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let t = quote! {
        impl #impl_generics StructMap for #name #ty_generics #where_clause {
            fn from_hashmap(hm: &std::collections::HashMap<&str, &str>) -> #name {
                let dummy = "";
                #name {
                    #(#field_names: FromStr::from_str(hm.get(#field_names_camel).unwrap_or(&dummy)).unwrap_or_default()),*
                    ,..Default::default()
                }
            }
            fn from_vec(v: Vec<&str>) -> #name {
                #name {
                    #(#field_names_copy: FromStr::from_str(v[#field_indices]).unwrap_or_default()),*
                }
            }
        }
    };

    println!("Debug:\n{}", t.as_str());

    t
}
