import fs from "fs";
import { RuneV1, RuntimeV1, addRuntimeV1ToImports, Metadata } from "@hotg-ai/rune-wit-files";

async function main() {
    if (process.argv.length < 2) {
        console.log(`Usage: node --loader ts-node/esm ./index.ts ./path/to/my/proc-block.wasm`);
        return;
    }

    // Retrieve the WebAssembly from somewhere
    const wasm: ArrayBuffer = fs.readFileSync(process.argv[2]);

    // First we construct an uninitialized Rune
    const rune = new RuneV1();

    // We need to implement the functions our Rune will import
    const runtimeFunctions: RuntimeV1 = {
        registerNode: (meta: Metadata) => {
            console.log("[Registered Metadata]", meta);
        },
    };

    // Next, create the imports object and add our runtime functions to it. We
    // can't use the runtimeFunctions object directly because our Rune may import
    // functions from multiple host modules.
    const imports = {};

    addRuntimeV1ToImports(imports, runtimeFunctions, (name: string) => rune.instance.exports[name])

    // Finally, we can finish initializing our Rune
    await rune.instantiate(wasm, imports);

    // Now, let's call a function from the Rune
    rune.start();
}

main();
