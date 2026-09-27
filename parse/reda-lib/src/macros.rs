#[macro_export]
macro_rules! lib_enum {
    (
        $(#[$meta:meta])*
        $name:ident, $first:ident $(, $rest:ident)* $(,)?
    ) => {
        paste::paste! {
            $(#[$meta])*
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub enum $name {
                [<$first:camel>],
                $([<$rest:camel>]),*
            }

            impl Default for $name {
                fn default() -> Self {
                    Self::[<$first:camel>]
                }
            }

            impl std::str::FromStr for $name {
                type Err = crate::error::LibError;
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    match s {
                        stringify!($first) => Ok(Self::[<$first:camel>]),
                        $(stringify!($rest) => Ok(Self::[<$rest:camel>])),*,
                        _ => Err(crate::error::LibError::UnsupportEnumValue(stringify!($name), s.to_string())),
                    }
                }
            }

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        Self::[<$first:camel>] => write!(f, stringify!($first)),
                        $(Self::[<$rest:camel>] => write!(f, stringify!($rest))),*,
                    }
                }
            }
        }
    };
}

