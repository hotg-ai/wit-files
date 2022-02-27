import ReactDOM from "react-dom";
import React, { ChangeEvent, useState } from "react";
import { RuneV1, RuntimeV1, addRuntimeV1ToImports, Metadata, ArgumentMetadata, TensorMetadata, TensorHint, TypeHint, ElementType, Dimensions } from "@hotg-ai/rune-wit-files";

function App() {
    const [procBlocks, setProcBlocks] = useState<ProcBlockMetadata[]>([]);
    const [filter, setFilter] = useState("");

    const onFileUploaded = async (e: ChangeEvent<HTMLInputElement>) => {
        const file = e.target.files?.[0];
        if (!file) {
            return;
        }

        const wasm = await file.arrayBuffer();
        const metadata = await loadMetadata(wasm);
        setProcBlocks([...procBlocks, metadata]);
        console.log("[Loaded]", metadata);
    };

    const selected = procBlocks.filter(meta => {
        return meta.name.includes(filter) || meta.tags.some(tag => tag.includes(filter));
    })
        .map((meta, i) => (<li key={i} title={JSON.stringify(meta, null, 2)}>{meta.name} v{meta.version}</li>));

    return (
        <div>
            <form>
                <input type="text" placeholder="Filter..." onChange={e => setFilter(e.target.value)} />
                <input type="file" onChange={onFileUploaded} />
            </form>

            <ul>
                {selected}
            </ul>
        </div>
    );
}

class Argument implements ArgumentMetadata {
    name: string;
    description?: string;
    defaultValue?: string;
    typeHint?: TypeHint;

    constructor(name: string) {
        this.name = name;
    }

    setDescription(description: string) {
        this.description = description;
    }

    setDefaultValue(defaultValue: string) {
        this.defaultValue = defaultValue;
    }

    setTypeHint(hint: TypeHint) {
        this.typeHint = hint;
    }
}

class Tensor implements TensorMetadata {
    name: string;
    description?: string;
    hints: Hint[] = [];

    constructor(name: string) {
        this.name = name;
    }

    setDescription(description: string) {
        this.description = description;
    }

    addHint(hint: TensorHint) {
        this.hints.push(hint as Hint);
    }
}

type MediaHint = {
    type: "interpret-as";
    media: "audio" | "image";
};
type ExampleShapeHint = {
    type: "supported-shape";
    supportedElementTypes: ElementType[];
    dimensions: Dimensions;
}

type Hint = MediaHint | ExampleShapeHint;

class ProcBlockMetadata implements Metadata {
    name: string;
    version: string;
    description?: string;
    repository?: string;
    homepage?: string;
    tags: string[] = [];
    arguments: Argument[] = [];
    inputs: Tensor[] = [];
    outputs: Tensor[] = [];

    constructor(name: string, version: string) {
        this.name = name;
        this.version = version;
    }

    setHomepage(url: string) {
        this.homepage = url;
    }

    setDescription(description: string) {
        this.description = description;
    }
    setRepository(url: string) {
        this.repository = url;
    }
    addTag(tag: string) {
        this.tags.push(tag);
    }

    addArgument(arg: ArgumentMetadata) {
        if (arg instanceof Argument) {
            this.arguments.push(arg);
        } else {
            throw new Error();
        }
    }

    addInput(metadata: TensorMetadata) {
        if (metadata instanceof Tensor) {
            this.inputs.push(metadata);
        } else {
            throw new Error();
        }
    }

    addOutput(metadata: TensorMetadata) {
        if (metadata instanceof Tensor) {
            this.outputs.push(metadata);
        } else {
            throw new Error();
        }
    }
};

class Runtime implements RuntimeV1 {
    metadata?: ProcBlockMetadata;

    metadataNew(name: string, version: string): ProcBlockMetadata {
        return new ProcBlockMetadata(name, version);
    }

    argumentMetadataNew(name: string): Argument {
        return new Argument(name);
    }

    tensorMetadataNew(name: string): Tensor {
        return new Tensor(name);
    }

    interpretAsImage(): Hint {
        return { type: "interpret-as", media: "image" };
    }

    interpretAsAudio(): Hint {
        return { type: "interpret-as", media: "audio" };
    }

    supportedShapes(supportedElementTypes: ElementType[], dimensions: Dimensions): Hint {
        return {
            type: "supported-shape",
            supportedElementTypes,
            dimensions,
        };
    }

    registerNode(m: Metadata) {
        if (m instanceof ProcBlockMetadata) {
            this.metadata = m;
        } else {
            throw new Error();
        }
    }
}

async function loadMetadata(wasm: ArrayBuffer): Promise<ProcBlockMetadata> {
    const rune = new RuneV1();
    const runtime = new Runtime();

    // Next, create the imports object and add our runtime functions to it. We
    // can't use the runtimeFunctions object directly because our Rune may import
    // functions from multiple host modules.
    const imports = {};

    addRuntimeV1ToImports(imports, runtime, (name: string) => rune.instance.exports[name])

    // Finally, we can finish initializing our Rune
    await rune.instantiate(wasm, imports);

    // Now, let's call a function from the Rune
    rune.start();

    if (runtime.metadata) {
        return runtime.metadata;
    } else {
        throw new Error("No metadata was registered");
    }
}

const app = document.getElementById("app");
ReactDOM.render(<App />, app);
