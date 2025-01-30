use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Idable)]
pub fn derive_idable(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_idable(&ast)
}


fn impl_idable(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let mut variant_names: Vec::<syn::Ident>  = vec!();
    let variants = if let  syn::Data::Enum(data) = &ast.data {
        data.variants.iter()
    } else {
        panic!("Idable cannot be derived on non-enum");
    };

    let variant_count = variants.len();
    let mut indexes=vec!();
    let mut i = 0 as usize;
    for variant in variants {
        variant_names.push(variant.ident.clone());
        indexes.push(i);
        i += 1;
    }

    let gen = quote! {
        impl Idable for #name {
            const MAX: usize = #variant_count;
            fn idx(&self) -> usize {
                match self {
                    #(Self:: #variant_names => #indexes),* 
                }
            }
        }
    };
    gen.into()
}