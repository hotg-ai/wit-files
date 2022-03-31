//! Automatically re-generate the `wit-bindgen` glue code.
//!
//! # Patching the Bindings
//!
//! This build script automatically applies a patch to work around [a known
//! wit-bindgen error][wit-bindgen-170].
//!
//! If you need to update the patch file, do the following:
//!
//! 1. Set the `SKIP_REGENERATE=1` environment variable
//! 2. Delete the existing `src/bindings.rs` and `bindings.patch`
//! 3. Manually generate the bindings using
//!
//!    ```console
//!    $ wit-bindgen rust-wasm --rustfmt --import ../runtime-v1.wit --export ../rune-v1.wit --out-dir src
//!    ```
//!
//! 4. Build the project like normal (this will probably fail)
//! 5. Fix the compile errors
//! 6. Once the crate compiles again, save the patch file using
//!
//!    ```console
//!    $ git diff -- src/bindings.rs > bindings.patch
//!    ```
//! 7. Clear the `SKIP_REGENERATE` variable ()
//!
//! [wit-bindgen-170]: https://github.com/bytecodealliance/wit-bindgen/issues/170

use std::{
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
};

fn main() {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let patch_file = crate_dir.join("bindings.patch");
    let wit_dir = crate_dir.parent().unwrap();
    let imports = [wit_dir.join("runtime-v1.wit")];
    let exports = [wit_dir.join("rune-v1.wit")];

    if !env_is_set("CI") && !env_is_set("SKIP_REGENERATE") {
        run_command(regenerate(&imports, &exports));

        if patch_file.exists() {
            run_command(apply_patch(&patch_file));
        }
    }

    for path in imports.iter().chain(&exports) {
        println!("cargo:rerun-if-changed={}", path.display());
    }
    println!("cargo:rerun-if-changed={}", patch_file.display());
}

fn env_is_set(name: &str) -> bool {
    println!("cargo:rerun-if-env-changed={}", name);
    std::env::var(name).is_ok()
}

fn regenerate(imports: &[PathBuf], exports: &[PathBuf]) -> Command {
    let mut cmd = Command::new("wit-bindgen");

    cmd.arg("rust-wasm")
        .arg("--rustfmt")
        .arg("--out-dir")
        .arg("src");

    for import in imports {
        cmd.arg("--import").arg(import);
    }
    for export in exports {
        cmd.arg("--export").arg(export);
    }

    cmd
}

fn apply_patch(patch: &Path) -> Command {
    let mut cmd = Command::new("git");
    cmd.arg("apply").arg(patch);
    cmd
}

fn run_command(mut cmd: Command) -> ExitStatus {
    println!("Running {:?}", cmd);

    let status = cmd
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .status()
        .unwrap_or_else(|e| {
            panic!(
                "Unable to run `{:?}`: {}.\nIs `{}` installed?",
                cmd,
                e,
                cmd.get_program().to_string_lossy(),
            )
        });

    if !status.success() {
        panic!("the `wit-bindgen` command failed");
    }

    status
}
