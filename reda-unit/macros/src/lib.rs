use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Expr};

struct Input {
    value: Expr,
    suffix_unit: String,
}

impl Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let value = input.parse::<Expr>()?;

        let rest: proc_macro2::TokenStream = input.parse()?;
        let mut suffix_unit = String::new();
        for tt in rest {
            suffix_unit.push_str(&tt.to_string());
        }

        Ok(Input { value, suffix_unit })
    }
}

#[proc_macro]
pub fn u(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Input);

    for (unit_name, unit) in UNITS_MAP {
        if input.suffix_unit.ends_with(unit_name) {
            let suffix_len = input.suffix_unit.len() - unit_name.len();
            let cpath = get_crate_path();
            let value = input.value.clone();
            let unit_ident = syn::Ident::new(unit, proc_macro2::Span::call_site());

            let input_suffix = &input.suffix_unit[..suffix_len];
            for &(suffix_name, suffix) in SUFFIX_MAP.iter() {
                if suffix_name == input_suffix {
                    let suffix = syn::Ident::new(suffix, proc_macro2::Span::call_site());
                    let code = quote! {
                        #cpath::#unit_ident::new(#cpath::Number::new(#value as f64, #cpath::Suffix::#suffix))
                    };

                    return code.into();
                }
            }

            return syn::Error::new_spanned(quote! { #input_suffix }, "Invalid suffix")
                .to_compile_error()
                .into();
        }
    }

    let suffix_unit = input.suffix_unit;
    syn::Error::new_spanned(quote! { #suffix_unit }, "Invalid suffix+unit")
        .to_compile_error()
        .into()
}

fn get_crate_path() -> proc_macro2::TokenStream {
    match crate_name("reda-unit") {
        Ok(FoundCrate::Itself) => {
            quote! { crate }
        }
        Ok(FoundCrate::Name(name)) => {
            let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
            quote! { #ident }
        }
        Err(_) => {
            quote! { reda-unit }
        }
    }
}

const SUFFIX_MAP: &[(&str, &str)] = &[
    ("m", "Milli"),
    ("k", "Kilo"),
    ("K", "Kilo"),
    ("M", "Mega"),
    ("G", "Giga"),
    ("u", "Micro"),
    ("n", "Nano"),
    ("p", "Pico"),
    ("",  "None"),
];

const UNITS_MAP: &[(&str, &str)] = &[
    ("m", "Length"),
    ("m²", "Area"),
    ("N", "Force"),
    ("Pa", "Pressure"),
    ("Wb", "MagneticFlux"),
    ("T", "FluxDensity"),
    ("S", "Conductance"),
    ("m/s", "Velocity"),
    ("m/s²", "Accel"),
    ("K", "Temperature"),
    ("rad", "Angle"),
    ("V", "Voltage"),
    ("v", "Voltage"),
    ("A", "Current"),
    ("Ω", "Resistance"),
    ("F", "Capacitance"),
    ("H", "Inductance"),
    ("Q", "Charge"),
    ("W", "Power"),
    ("J", "Energy"),
    ("s", "Time"),
    ("Hz", "Frequency"),
    ("HZ", "Frequency"),
    ("hz", "Frequency"),
];
