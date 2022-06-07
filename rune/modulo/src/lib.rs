use bytemuck::{AnyBitPattern, NoUninit};
pub use Modulo as Node;

use crate::proc_block_v2::*;
use num_traits::{FromPrimitive, ToPrimitive};
use wit_bindgen_rust::Handle;

wit_bindgen_rust::export!("../proc-block-v2.wit");
wit_bindgen_rust::import!("../runtime-v2.wit");

pub struct ProcBlockV2;

impl proc_block_v2::ProcBlockV2 for ProcBlockV2 {
    fn metadata() -> Metadata {
        Metadata {
            name: "Modulo".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            tags: Vec::new(),
            description: Some(include_str!("../README.md").to_string()),
            homepage: Some(env!("CARGO_PKG_HOMEPAGE").to_string()),
            repository: Some(env!("CARGO_PKG_REPOSITORY").to_string()),
            arguments: vec![ArgumentMetadata {
                name: "modulus".to_string(),
                description: Some("The modulus operand".to_string()),
                default_value: None,
                hints: vec![ArgumentHint::ArgumentType(ArgumentType::Float)],
            }],
            inputs: vec![TensorMetadata {
                name: "input".to_string(),
                description: None,
                hints: Vec::new(),
            }],
            outputs: vec![TensorMetadata {
                name: "output".to_string(),
                description: None,
                hints: Vec::new(),
            }],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Modulo {
    modulus: f64,
}

impl TryFrom<Vec<Argument>> for Modulo {
    type Error = ArgumentError;
    fn try_from(args: Vec<Argument>) -> Result<Self, Self::Error> {
        let modulus = support::parse_arg(&args, "modulus")?;

        Ok(Modulo { modulus })
    }
}

impl proc_block_v2::Node for Modulo {
    fn new(args: Vec<Argument>) -> Result<Handle<crate::Modulo>, ArgumentError> {
        Modulo::try_from(args).map(Handle::new)
    }

    fn tensor_constraints(&self) -> TensorConstraints {
        TensorConstraints {
            inputs: vec![TensorConstraint {
                name: "input".to_string(),
                element_type: !ElementTypeConstraint::UTF8,
                dimensions: Dimensions::Dynamic,
            }],
            outputs: vec![TensorConstraint {
                name: "output".to_string(),
                element_type: !ElementTypeConstraint::UTF8,
                dimensions: Dimensions::Dynamic,
            }],
        }
    }

    fn run(&self, inputs: Vec<Tensor>) -> Result<Vec<Tensor>, proc_block_v2::KernelError> {
        let mut tensor = inputs
            .into_iter()
            .find(|tensor| tensor.name == "input")
            .ok_or_else(|| {
                KernelError::InvalidInput(InvalidInput {
                    name: "input".to_string(),
                    reason: InvalidInputReason::NotFound,
                })
            })?;

        tensor.name = "output".to_string();

        match tensor.element_type {
            ElementType::U8 => modulo_in_place::<u8>(&mut tensor.buffer, self.modulus),
            ElementType::I8 => modulo_in_place::<i8>(&mut tensor.buffer, self.modulus),
            ElementType::U16 => modulo_in_place::<u16>(&mut tensor.buffer, self.modulus),
            ElementType::I16 => modulo_in_place::<i16>(&mut tensor.buffer, self.modulus),
            ElementType::U32 => modulo_in_place::<u32>(&mut tensor.buffer, self.modulus),
            ElementType::I32 => modulo_in_place::<i32>(&mut tensor.buffer, self.modulus),
            ElementType::F32 => modulo_in_place::<f32>(&mut tensor.buffer, self.modulus),
            ElementType::U64 => modulo_in_place::<u64>(&mut tensor.buffer, self.modulus),
            ElementType::I64 => modulo_in_place::<i64>(&mut tensor.buffer, self.modulus),
            ElementType::F64 => modulo_in_place::<f64>(&mut tensor.buffer, self.modulus),
            ElementType::Utf8 | ElementType::Complex128 | ElementType::Complex64 => {
                return Err(KernelError::InvalidInput(InvalidInput {
                    name: tensor.name,
                    reason: InvalidInputReason::UnsupportedShape,
                }))
            }
        }

        Ok(vec![tensor])
    }
}

fn modulo_in_place<T>(bytes: &mut [u8], modulus: f64)
where
    T: NoUninit + AnyBitPattern + ToPrimitive + FromPrimitive + Copy,
{
    let items: &mut [T] = bytemuck::cast_slice_mut(bytes);

    for item in items {
        let result = item
            .to_f64()
            .map(|n| n % modulus)
            .and_then(|n| T::from_f64(n));

        if let Some(updated) = result {
            *item = updated;
        }
    }
}

macro_rules! support {
    ($($proc_block:ident)::*) => {
        mod support {
            use std::{fmt::Display, str::FromStr};
            use $($proc_block)::*::{
                Argument,
                ArgumentError,
                ArgumentErrorReason,
            };

            pub fn parse_arg<T>(args: &[Argument], name: &str) -> Result<T, ArgumentError>
            where
                T: FromStr,
                T::Err: Display,
            {
                for arg in args {
                    if arg.name == name {
                        return arg.value.parse::<T>().map_err(|e| ArgumentError {
                            name: name.to_string(),
                            reason: ArgumentErrorReason::InvalidValue(e.to_string()),
                        });
                    }
                }

                Err(ArgumentError {
                    name: name.to_string(),
                    reason: ArgumentErrorReason::NotFound,
                })
            }
        }
    };
}

support!(crate::proc_block_v2);

#[cfg(test)]
mod tests {
    use super::*;

    fn args(arguments: &[(&str, &str)]) -> Vec<Argument> {
        arguments
            .iter()
            .map(|(n, v)| Argument {
                name: n.to_string(),
                value: v.to_string(),
            })
            .collect()
    }

    #[test]
    fn create_modulo_with_good_modulus() {
        let args = args(&[("modulus", "42.0")]);

        let proc_block = Modulo::try_from(args).unwrap();

        assert_eq!(proc_block, Modulo { modulus: 42.0 });
    }
}
