import ReactDOM from "react-dom";
import { ChangeEvent, useState } from "react";
import { rune_v1, runtime_v1 } from "@hotg-ai/rune-wit-files";

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

class Argument implements runtime_v1.ArgumentMetadata {
    name: string;
    description?: string;
    defaultValue?: string;
    hints: runtime_v1.ArgumentHint[] = [];

    constructor(name: string) {
        this.name = name;
    }

    setDescription(description: string) {
        this.description = description;
    }

    setDefaultValue(defaultValue: string) {
        this.defaultValue = defaultValue;
    }

    addHint(hint: runtime_v1.ArgumentHint): void {
        this.hints.push(hint);
    }
}

class Tensor implements runtime_v1.TensorMetadata {
    name: string;
    description?: string;
    hints: TensorHint[] = [];

    constructor(name: string) {
        this.name = name;
    }

    setDescription(description: string) {
        this.description = description;
    }

    addHint(hint: runtime_v1.TensorHint) {
        this.hints.push(hint as TensorHint);
    }
}

type MediaHint = {
    type: "interpret-as";
    media: "audio" | "image";
};
type ExampleShapeHint = {
    type: "supported-shape";
    supportedElementTypes: runtime_v1.ElementType[];
    dimensions: runtime_v1.Dimensions;
}

type TensorHint = MediaHint | ExampleShapeHint;

type NumberInRange = {
    type: "number-in-range";
    min: number;
    max: number;
};
type StringEnum = {
    type: "string-enum";
    values: string[];
};
type NonNegativeNumber = { type: "non-negative-number" };
type SupportedArgumentType = {
    type: "supported-argument-type";
    argumentType: runtime_v1.ArgumentType;
};
type ArgumentHint = NumberInRange | StringEnum | NonNegativeNumber | SupportedArgumentType;

class ProcBlockMetadata implements runtime_v1.Metadata {
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

    addArgument(arg: runtime_v1.ArgumentMetadata) {
        if (arg instanceof Argument) {
            this.arguments.push(arg);
        } else {
            throw new Error();
        }
    }

    addInput(metadata: runtime_v1.TensorMetadata) {
        if (metadata instanceof Tensor) {
            this.inputs.push(metadata);
        } else {
            throw new Error();
        }
    }

    addOutput(metadata: runtime_v1.TensorMetadata) {
        if (metadata instanceof Tensor) {
            this.outputs.push(metadata);
        } else {
            throw new Error();
        }
    }
};

class Runtime implements runtime_v1.RuntimeV1 {
    metadata?: ProcBlockMetadata;
    graphContext?: runtime_v1.GraphContext;
    kernelContext?: runtime_v1.KernelContext;

    metadataNew(name: string, version: string): ProcBlockMetadata {
        return new ProcBlockMetadata(name, version);
    }

    argumentMetadataNew(name: string): Argument {
        return new Argument(name);
    }

    tensorMetadataNew(name: string): Tensor {
        return new Tensor(name);
    }

    interpretAsImage(): TensorHint {
        return { type: "interpret-as", media: "image" };
    }

    interpretAsAudio(): TensorHint {
        return { type: "interpret-as", media: "audio" };
    }

    interpretAsNumberInRange(min: string, max: string): ArgumentHint {
        return { type: "number-in-range", min: parseFloat(min), max: parseFloat(max) };
    }

    interpretAsStringInEnum(stringEnum: string[]): ArgumentHint {
        return { type: "string-enum", values: stringEnum };
    }

    nonNegativeNumber(): ArgumentHint {
        return { type: "non-negative-number" };
    }

    supportedArgumentType(hint: runtime_v1.ArgumentType): ArgumentHint {
        return { type: "supported-argument-type", argumentType: hint };
    }

    supportedShapes(supportedElementTypes: runtime_v1.ElementType[], dimensions: runtime_v1.Dimensions): TensorHint {
        return {
            type: "supported-shape",
            supportedElementTypes,
            dimensions,
        };
    }

    registerNode(m: runtime_v1.Metadata) {
        if (m instanceof ProcBlockMetadata) {
            this.metadata = m;
        } else {
            throw new Error();
        }
    }

    graphContextCurrent(): runtime_v1.GraphContext | null {
        return this.graphContext || null;
    }

    kernelContextCurrent(): runtime_v1.KernelContext | null {
        return this.kernelContext || null;
    }
}

async function loadMetadata(wasm: ArrayBuffer): Promise<ProcBlockMetadata> {
    const rune = new rune_v1.RuneV1();
    const runtime = new Runtime();

    // Next, create the imports object and add our runtime functions to it. We
    // can't use the runtimeFunctions object directly because our Rune may import
    // functions from multiple host modules.
    const imports = {};

    runtime_v1.addRuntimeV1ToImports(imports, runtime, (name: string) => rune.instance.exports[name])

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
