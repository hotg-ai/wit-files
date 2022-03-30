#![allow(dead_code)]

include!("./bindings.rs");

use std::fmt::Display;

use crate::{
    rune_v1::{BadArgumentReason, GraphError, InvalidArgument, KernelError},
    runtime_v1::{
        ArgumentMetadata, Dimensions, ElementType, GraphContext, KernelContext, Metadata,
        TensorDataParam, TensorDataResult, TensorMetadata, TensorParam, TensorResult,
    },
};
use num_traits::{FromPrimitive, ToPrimitive};
use wit_bindgen_rust::Handle;

pub struct RuneV1;

impl rune_v1::RuneV1 for RuneV1 {
    fn start() {
        let metadata = Metadata::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        metadata.set_description(env!("CARGO_PKG_DESCRIPTION"));

        let modulo = ArgumentMetadata::new("modulo");
        modulo.add_hint(&runtime_v1::non_negative_number());
        metadata.add_argument(&modulo);
        let element_type = ArgumentMetadata::new("element_type");
        element_type.set_description("The type of tensor this proc-block will accept");
        element_type.set_default_value("f64");
        element_type.add_hint(&runtime_v1::interpret_as_string_in_enum(&[
            "u8", "i8", "u16", "i16", "u32", "i32", "f32", "u64", "i64", "f64",
        ]));
        metadata.add_argument(&element_type);

        let input = TensorMetadata::new("input");
        metadata.add_input(&input);

        let output = TensorMetadata::new("output");
        metadata.add_output(&output);

        runtime_v1::register_node(&metadata);
    }

    fn graph(ctx: Handle<GraphContext>) -> Result<(), GraphError> {
        // make sure the modulus is valid
        let _ = get_modulus(|n| ctx.get_argument(n)).map_err(GraphError::InvalidArgument)?;

        let element_type = match ctx.get_argument("element_type").as_deref() {
            Some("u8") => ElementType::U8,
            Some("i8") => ElementType::I8,
            Some("u16") => ElementType::U16,
            Some("i16") => ElementType::I16,
            Some("u32") => ElementType::U32,
            Some("i32") => ElementType::I32,
            Some("f32") => ElementType::F32,
            Some("u64") => ElementType::U64,
            Some("i64") => ElementType::I64,
            Some("f64") | None => ElementType::F64,
            Some(_) => {
                return Err(GraphError::InvalidArgument(InvalidArgument {
                    name: "element_type".to_string(),
                    reason: BadArgumentReason::InvalidValue("Unsupported element type".to_string()),
                }))
            }
        };

        ctx.add_input_tensor("input", element_type, Dimensions::Dynamic);
        ctx.add_output_tensor("output", element_type, Dimensions::Dynamic);

        Ok(())
    }

    fn kernel(ctx: Handle<KernelContext>) -> Result<(), KernelError> {
        let modulus = get_modulus(|n| ctx.get_argument(n)).map_err(KernelError::InvalidArgument)?;

        let TensorResult { dimensions, data } = ctx
            .get_input_tensor("input")
            .ok_or_else(|| KernelError::MissingInput("input".to_string()))?;

        let m = Modulus {
            ctx: &ctx,
            dimensions: &dimensions,
            modulus,
        };

        // Note: The "element_type" argument is only used while constructing the
        // ML pipeline. We see its effect at runtime in the form of the tensor
        // data variant that gets used.

        match data {
            TensorDataResult::U8(d) => m.evaluate(&d, |v| TensorDataParam::U8(v)),
            TensorDataResult::I8(d) => m.evaluate(&d, |v| TensorDataParam::I8(v)),
            TensorDataResult::U16(d) => m.evaluate(&d, |v| TensorDataParam::U16(v)),
            TensorDataResult::I16(d) => m.evaluate(&d, |v| TensorDataParam::I16(v)),
            TensorDataResult::U32(d) => m.evaluate(&d, |v| TensorDataParam::U32(v)),
            TensorDataResult::I32(d) => m.evaluate(&d, |v| TensorDataParam::I32(v)),
            TensorDataResult::F32(d) => m.evaluate(&d, |v| TensorDataParam::F32(v)),
            TensorDataResult::U64(d) => m.evaluate(&d, |v| TensorDataParam::U64(v)),
            TensorDataResult::I64(d) => m.evaluate(&d, |v| TensorDataParam::I64(v)),
            TensorDataResult::F64(d) => m.evaluate(&d, |v| TensorDataParam::F64(v)),
            TensorDataResult::Utf8(_) => Err(KernelError::Other(
                "String tensors aren't supported".to_string(),
            )),
        }
    }
}

struct Modulus<'a> {
    ctx: &'a KernelContext,
    dimensions: &'a [u32],
    modulus: f64,
}

impl<'a> Modulus<'a> {
    fn evaluate<T, F>(&self, values: &[T], to_params: F) -> Result<(), KernelError>
    where
        T: ToPrimitive + FromPrimitive + Display,
        F: Fn(&[T]) -> TensorDataParam<'_>,
    {
        let Modulus {
            ctx,
            dimensions,
            modulus,
        } = self;
        let mut results = Vec::new();

        for value in values {
            let as_float = value.to_f64().ok_or_else(|| error(value))?;
            let after_modulus = as_float % modulus;
            let round_tripped = T::from_f64(after_modulus).ok_or_else(|| error(value))?;
            results.push(round_tripped);
        }

        let data = to_params(&results);

        ctx.set_output_tensor("output", TensorParam { dimensions, data });

        Ok(())
    }
}

fn error(value: impl Display) -> KernelError {
    KernelError::Other(format!("Unable to convert `{}` to/from a double", value))
}

fn get_modulus(get_argument: impl FnOnce(&str) -> Option<String>) -> Result<f64, InvalidArgument> {
    let value = match get_argument("modulus") {
        Some(s) => s,
        None => {
            return Err(InvalidArgument {
                name: "modulus".to_string(),
                reason: BadArgumentReason::NotFound,
            })
        }
    };

    let value = value.parse::<f64>().map_err(|e| InvalidArgument {
        name: "element_type".to_string(),
        reason: BadArgumentReason::InvalidValue(e.to_string()),
    })?;

    if value > 0.0 {
        Ok(value)
    } else {
        Err(InvalidArgument {
            name: "element_type".to_string(),
            reason: BadArgumentReason::InvalidValue(
                "The modulus must be a positive, non-zero number".to_string(),
            ),
        })
    }
}
