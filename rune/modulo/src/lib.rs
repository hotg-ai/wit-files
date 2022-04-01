#![allow(dead_code)]

wit_bindgen_rust::import!("../runtime-v1.wit");
wit_bindgen_rust::export!("../rune-v1.wit");

use std::fmt::Display;

use crate::{
    hotg_proc_blocks::BufferExt,
    rune_v1::{BadArgumentReason, GraphError, InvalidArgument, KernelError},
    runtime_v1::{
        ArgumentMetadata, Dimensions, ElementType, GraphContext, KernelContext, Metadata,
        TensorMetadata, TensorParam, TensorResult,
    },
};
use num_traits::{FromPrimitive, ToPrimitive};

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

    fn graph() -> Result<(), GraphError> {
        let ctx = GraphContext::current()
            .ok_or_else(|| GraphError::Other("Unable to load the graph context".to_string()))?;

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

    fn kernel() -> Result<(), KernelError> {
        let ctx = KernelContext::current()
            .ok_or_else(|| KernelError::Other("Unable to load the kernel context".to_string()))?;

        let modulus = get_modulus(|n| ctx.get_argument(n)).map_err(KernelError::InvalidArgument)?;

        let TensorResult {
            dimensions,
            element_type,
            mut buffer,
        } = ctx
            .get_input_tensor("input")
            .ok_or_else(|| KernelError::MissingInput("input".to_string()))?;

        // Note: The "element_type" argument is only used while constructing the
        // ML pipeline. We see its effect at runtime in the form of the tensor
        // data variant that gets used.

        match element_type {
            ElementType::U8 => modulus_in_place(buffer.elements_mut::<u8>(), modulus)?,
            ElementType::I8 => modulus_in_place(buffer.elements_mut::<i8>(), modulus)?,
            ElementType::U16 => modulus_in_place(buffer.elements_mut::<u16>(), modulus)?,
            ElementType::I16 => modulus_in_place(buffer.elements_mut::<i16>(), modulus)?,
            ElementType::U32 => modulus_in_place(buffer.elements_mut::<u32>(), modulus)?,
            ElementType::I32 => modulus_in_place(buffer.elements_mut::<i32>(), modulus)?,
            ElementType::F32 => modulus_in_place(buffer.elements_mut::<f32>(), modulus)?,
            ElementType::U64 => modulus_in_place(buffer.elements_mut::<u64>(), modulus)?,
            ElementType::I64 => modulus_in_place(buffer.elements_mut::<i64>(), modulus)?,
            ElementType::F64 => modulus_in_place(buffer.elements_mut::<f64>(), modulus)?,
            ElementType::Utf8 => {
                return Err(KernelError::Other(
                    "String tensors aren't supported".to_string(),
                ))
            }
        }

        ctx.set_output_tensor(
            "output",
            TensorParam {
                element_type,
                dimensions: &dimensions,
                buffer: &buffer,
            },
        );

        Ok(())
    }
}

fn modulus_in_place<T>(values: &mut [T], modulus: f64) -> Result<(), KernelError>
where
    T: ToPrimitive + FromPrimitive + Copy + Display,
{
    for value in values {
        let as_float = value.to_f64().ok_or_else(|| error(*value))?;
        let after_modulus = as_float % modulus;
        *value = T::from_f64(after_modulus).ok_or_else(|| error(*value))?;
    }

    Ok(())
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
        name: "modulus".to_string(),
        reason: BadArgumentReason::InvalidValue(e.to_string()),
    })?;

    if value > 0.0 {
        Ok(value)
    } else {
        Err(InvalidArgument {
            name: "modulus".to_string(),
            reason: BadArgumentReason::InvalidValue(
                "The modulus must be a positive, non-zero number".to_string(),
            ),
        })
    }
}

/// Support crate provided by hotg.
mod hotg_proc_blocks {
    pub trait BufferExt {
        fn elements_mut<T: ValueType>(&mut self) -> &mut [T];
    }

    impl BufferExt for [u8] {
        fn elements_mut<T: ValueType>(&mut self) -> &mut [T] {
            unsafe { T::from_bytes_mut(self) }
        }
    }

    pub unsafe trait ValueType: Sized {
        unsafe fn from_bytes_mut(bytes: &mut [u8]) -> &mut [Self];
    }

    macro_rules! impl_value_type {
        ($( $type:ty ),* $(,)?) => {
            $(
                unsafe impl ValueType for $type {
                    unsafe fn from_bytes_mut(bytes: &mut [u8]) -> &mut [Self] {
                        let (start, middle, end) = bytes.align_to_mut::<$type>();
                        assert!(start.is_empty());
                        assert!(end.is_empty());
                        middle
                    }
                }
            )*
        };
    }

    impl_value_type!(u8, i8, u16, i16, u32, i32, f32, u64, i64, f64);
}
