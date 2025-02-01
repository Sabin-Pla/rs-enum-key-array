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
    let mut variant_names_field : Vec::<syn::Ident>  = vec!();
    let mut variant_names_named_field : Vec::<syn::Ident>  = vec!();


    let variants = if let  syn::Data::Enum(data) = &ast.data {
        data.variants.iter()
    } else {
        panic!("Idable cannot be derived on non-enum");
    };

    let variant_count = variants.len();
    for variant in variants {
        match variant.fields {
            syn::Fields::Named(_) => variant_names_named_field.push(variant.ident.clone()),
            syn::Fields::Unnamed(_) => variant_names_field.push(variant.ident.clone()),
            syn::Fields::Unit => variant_names.push(variant.ident.clone())
        }
    }

    let mut indexes=vec!();
    let mut indexes_field=vec!();
    let mut indexes_named_field=vec!();
    for i in 0..variant_names.len() {
        indexes.push(i);
    }
    let end = variant_names.len() + variant_names_field.len();
    for i in variant_names.len()..end {
        indexes_field.push(i);
    }
    for i in end..(end + variant_names_named_field.len()) {
        indexes_named_field.push(i);
    }


    let gen = quote! {
        impl Idable for #name {
            const MAX: usize = #variant_count;
            fn idx(&self) -> usize {
                match self {
                    #(Self::#variant_names => #indexes,)* 
                    #(Self::#variant_names_field(..) => #indexes_field,)* 
                    #(Self::#variant_names_named_field{..} => #indexes_named_field,)* 

                }
            }
        }
    };
    print!("{}", gen);
    gen.into()
}