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
  * Hint that a tensor may have a particular shape.
  * 
  * Note: This hint will be removed in the future in favour of a more flexible
  * mechanism.
  */
  exampleShape(elementType: ElementType, dimensions: Dimensions): TensorHint;
  /**
  * Register a node type with the runtime.
  */
  registerNode(metadata: Metadata): void;
  dropMetadata?: (val: Metadata) => void;
  dropArgumentMetadata?: (val: ArgumentMetadata) => void;
  dropTensorMetadata?: (val: TensorMetadata) => void;
  dropTensorHint?: (val: TensorHint) => void;
}
export interface Metadata {
  /**
  * A human-friendly description of the node.
  * 
  * The text may include markdown.
  */
  setDescription(description: string): void;
  /**
  * The source repository containing this node.
  */
  setRepository(url: string): void;
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
