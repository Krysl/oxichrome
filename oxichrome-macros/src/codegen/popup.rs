use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

pub fn expand(item: TokenStream) -> Result<TokenStream, syn::Error> {
    let func: ItemFn = syn::parse2(item)?;

    let fn_name = &func.sig.ident;
    let fn_body = &func.block;
    let vis = &func.vis;
    let attrs = &func.attrs;
    #[cfg(not(feature = "ui-dioxus"))]
    let ret = &func.sig.output;

    let output = {
        #[cfg(not(feature = "ui-dioxus"))]
        {
            quote! {
                #(#attrs)*
                #[::leptos::component]
                #vis fn #fn_name() #ret #fn_body

                #[doc(hidden)]
                #[oxichrome::__private::wasm_bindgen::prelude::wasm_bindgen]
                pub fn __oxichrome_mount_popup() {
                    let handle = ::leptos::mount::mount_to_body(#fn_name);
                    std::mem::forget(handle);
                }
            }
        }

        #[cfg(feature = "ui-dioxus")]
        {
            quote! {
                #(#attrs)*
                #[::dioxus::prelude::component]
                #vis fn #fn_name() -> ::dioxus::prelude::Element #fn_body

                #[doc(hidden)]
                #[oxichrome::__private::wasm_bindgen::prelude::wasm_bindgen]
                pub fn __oxichrome_mount_popup() {
                    ::dioxus::launch(#fn_name);
                }
            }
        }
    };

    Ok(output)
}
