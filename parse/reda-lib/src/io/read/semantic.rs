use std::collections::HashMap;
use crate::{error::LibError, model::{LibAst, LibGroup}};

pub trait FromLibGroup: Sized {
    type Error: Sized;
    fn from_group(group: LibGroup) -> Result<Self, Self::Error>;
}

impl LibGroup {
    pub fn from_ast(ast: LibAst) -> Result<Self, LibError> {
        match ast {
            LibAst::LibGroup { kind, params, items } => {
                let mut attrs = HashMap::new();
                let mut groups = Vec::new();
                
                for item_ast in items {
                    if let LibAst::Attr { key, value } = item_ast {
                        attrs.insert(key, value);
                    } else {
                        groups.push(LibGroup::from_ast(item_ast)?);
                    }
                }

                Ok(Self {
                    kind, 
                    params,
                    attrs,
                    groups
                })
            }
            LibAst::Attr { key:_, value:_ } => Err(LibError::ExceptLibGroup),
        }
    }
}