/**
* Metadata describing a single node in the Machine Learning pipeline.
*/
export interface Metadata {
  /**
  * The node's name.
  */
  name: string,
  /**
  * The node's version.
  */
  version: string,
  /**
  * A human-friendly description of the node.
  * 
  * The text may include markdown.
  */
  description: string | null,
  /**
  * The source repository containing this node.
  */
  repository: string | null,
  /**
  * An array of strings that describe this package.
  * 
  * Tags are typically used to assist in search and filtering.
  */
  tags: string[],
  /**
  * Arguments this node accepts.
  */
  arguments: ArgumentMetadata[],
  /**
  * Information about this node's input tensors.
  */
  inputs: TensorMetadata[],
  /**
  * Information about this node's output tensors.
  */
  outputs: TensorMetadata[],
}
/**
* Information about a node's argument.
*/
export interface ArgumentMetadata {
  /**
  * The argument's name.
  */
  name: string,
  /**
  * A human-friendly description of the argument.
  * 
  * The text may include markdown.
  */
  description: string | null,
  /**
  * A useful default value for this argument.
  */
  defaultValue: string | null,
  /**
  * A hint about what type this argument may contain.
  */
  typeHint: TypeHint | null,
}
export enum TypeHint {
  Integer = 0,
  Float = 1,
  OnelineString = 2,
  MultilineString = 3,
}
/**
* Information about a tensor.
*/
export interface TensorMetadata {
  /**
  * The tensor's name.
  */
  name: string,
  /**
  * A human-friendly description of the tensor.
  * 
  * The text may include markdown.
  */
  description: string | null,
}
export function addRuntimeV1ToImports(imports: any, obj: RuntimeV1, get_export: (name: string) => WebAssembly.ExportValue): void;
export interface RuntimeV1 {
  /**
  * Register a node type with the runtime.
  */
  registerNode(metadata: Metadata): void;
}
