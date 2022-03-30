#![allow(dead_code)]

include!("./bindings.rs");

use crate::{
    rune_v1::{GraphError, KernelError},
    runtime_v1::{ArgumentMetadata, GraphContext, KernelContext, Metadata},
};
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
        element_type.set_default_value("f32");
        element_type.add_hint(&runtime_v1::interpret_as_string_in_enum(&[
            "u8", "i8", "u16", "i16", "u32", "i32", "f32", "u64", "i64", "f64",
        ]));
        metadata.add_argument(&element_type);

        runtime_v1::register_node(&metadata);
    }

    fn graph(ctx: Handle<GraphContext>) -> Result<(), GraphError> {
        let element_type = ctx
            .get_argument("element_type")
            .unwrap_or_else(|| "f32".to_string());

        todo!();
    }

    fn kernel(_ctx: Handle<KernelContext>) -> Result<(), KernelError> {
        todo!()
    }
}
