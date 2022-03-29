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
        metadata.add_argument(&modulo);

        runtime_v1::register_node(&metadata);
    }

    fn graph(_ctx: Handle<GraphContext>) -> Result<(), GraphError> {
        todo!();
    }

    fn kernel(_ctx: Handle<KernelContext>) -> Result<(), KernelError> {
        todo!()
    }
}
