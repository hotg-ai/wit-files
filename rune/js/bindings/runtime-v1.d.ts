/**
* The format an argument's value may be in.
*/
export enum TypeHint {
  Integer = 0,
  Float = 1,
  OnelineString = 2,
  MultilineString = 3,
}
/**
* The various types of values a tensor may contain.
*/
export enum ElementType {
  Uint8 = 0,
  Int8 = 1,
  Uint16 = 2,
  Int16 = 3,
  Uint32 = 4,
  Int32 = 5,
  Float32 = 6,
  Uint64 = 7,
  Int64 = 8,
  Float64 = 9,
  /**
  * A string as UTF-8 encoded bytes.
  */
  Utf8 = 10,
}
/**
* The dimensions that a tensor may have.
*/
export type Dimensions = DimensionsDynamic | DimensionsFixed;
/**
* There can be an arbitrary number of dimensions with arbitrary sizes.
*/
export interface DimensionsDynamic {
  tag: "dynamic",
}
/**
* The tensor has a fixed rank with the provided dimension sizes.
* 
* If a particular dimension's length is zero, that is interpreted as the
* dimension being allowed to have any arbitrary length.
*/
export interface DimensionsFixed {
  tag: "fixed",
  val: Uint32Array,
}
export function addRuntimeV1ToImports(imports: any, obj: RuntimeV1, get_export: (name: string) => WebAssembly.ExportValue): void;
export interface RuntimeV1 {
  /**
  * Create a new metadata object with the provided name and version number.
  * 
  * The name should typically be one or two words that concisely describe
  * the node and will be used as the human-friendly label shown to users
  * when referring to it.
  */
  metadataNew(name: string, version: string): Metadata;
  /**
  * Create a new named argument.
  */
  argumentMetadataNew(name: string): ArgumentMetadata;
  /**
  * Create a new named tensor.
  */
  tensorMetadataNew(name: string): TensorMetadata;
  /**
  * Hint to the runtime that a tensor may be displayed as an image.
  */
  interpretAsImage(): TensorHint;
  /**
  * Hint to the runtime that a tensor may be interpreted as an audio clip.
  */
  interpretAsAudio(): TensorHint;
  /**
  * Hint that a tensor may have a particular shape and the element types it
  * supports.
  * 
  * Note: This hint will be removed in the future in favour of a more flexible
  * mechanism.
  */
  supportedShapes(supportedElementTypes: ElementType[], dimensions: Dimensions): TensorHint;
  /**
  * Hint to the runtime that an argument may be interpreted as a number in `[min, max]`
  */
  interpretAsNumberInRange(min: number, max: number): ArgumentHint;
  /**
  * Register a node type with the runtime.
  */
  registerNode(metadata: Metadata): void;
  dropMetadata?: (val: Metadata) => void;
  dropArgumentMetadata?: (val: ArgumentMetadata) => void;
  dropTensorMetadata?: (val: TensorMetadata) => void;
  dropTensorHint?: (val: TensorHint) => void;
  dropArgumentHint?: (val: ArgumentHint) => void;
}
export interface Metadata {
  /**
  * A human-friendly description of the node.
  * 
  * The text may include markdown.
  */
  setDescription(description: string): void;
  /**
  * A repository containing this node's source code.
  */
  setRepository(url: string): void;
  /**
  * The node's home page.
  * 
  * This will typically point to a `README` file or a page on the internet
  * that users can go to when they want to find out more about the node.
  */
  setHomepage(url: string): void;
  /**
  * Associate this node with a particular tag.
  * 
  * Tags are typically used to assist in search and filtering.
  */
  addTag(tag: string): void;
  /**
  * Arguments this node accepts.
  */
  addArgument(arg: ArgumentMetadata): void;
  /**
  * Information about this node's input tensors.
  */
  addInput(metadata: TensorMetadata): void;
  /**
  * Information about this node's output tensors.
  */
  addOutput(metadata: TensorMetadata): void;
}
export interface ArgumentMetadata {
  /**
  * A human-friendly description of the argument.
  * 
  * The text may include markdown.
  */
  setDescription(description: string): void;
  /**
  * A useful default value for this argument.
  */
  setDefaultValue(defaultValue: string): void;
  /**
  * A hint about what type this argument may contain.
  */
  setTypeHint(hint: TypeHint): void;
  addHint(hint: ArgumentHint): void;
}
export interface TensorMetadata {
  /**
  * A human-friendly description of the tensor.
  * 
  * The text may include markdown.
  */
  setDescription(description: string): void;
  /**
  * Add a hint that provides the runtime with contextual information about
  * this node.
  */
  addHint(hint: TensorHint): void;
}
export interface TensorHint {
}
export interface ArgumentHint {
}