use runtime_v1::*;

wit_bindgen_rust::import!("../runtime-v1.wit");
wit_bindgen_rust::export!("../rune-v1.wit");

pub struct RuneV1;

impl rune_v1::RuneV1 for RuneV1 {
    fn start() {
        let meta = Metadata::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        meta.set_description(env!("CARGO_PKG_DESCRIPTION"));
        meta.set_repository(env!("CARGO_PKG_REPOSITORY"));
    }
}
