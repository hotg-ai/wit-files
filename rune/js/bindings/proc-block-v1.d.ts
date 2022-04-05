export type Result<T, E> = { tag: "ok", val: T } | { tag: "err", val: E };
export type GraphError = GraphErrorInvalidArgument | GraphErrorOther;
export interface GraphErrorInvalidArgument {
  tag: "invalid-argument",
  val: InvalidArgument,
}
export interface GraphErrorOther {
  tag: "other",
  val: string,
}
export interface InvalidArgument {
  name: string,
  reason: BadArgumentReason,
}
/**
* The reason error is of type string that is thrown by the
* for example modulo(n: 0) => graph-error(invalid-argument(name: 'n', reason: invalid-value("N must be positive")))
*/
export type BadArgumentReason = BadArgumentReasonNotFound | BadArgumentReasonInvalidValue | BadArgumentReasonOther;
export interface BadArgumentReasonNotFound {
  tag: "not-found",
}
export interface BadArgumentReasonInvalidValue {
  tag: "invalid-value",
  val: string,
}
export interface BadArgumentReasonOther {
  tag: "other",
  val: string,
}
export type KernelError = KernelErrorInvalidArgument | KernelErrorMissingInput | KernelErrorOther;
export interface KernelErrorInvalidArgument {
  tag: "invalid-argument",
  val: InvalidArgument,
}
export interface KernelErrorMissingInput {
  tag: "missing-input",
  val: string,
}
export interface KernelErrorOther {
  tag: "other",
  val: string,
}
export class ProcBlockV1 {
  
  /**
  * The WebAssembly instance that this class is operating with.
  * This is only available after the `instantiate` method has
  * been called.
  */
  instance: WebAssembly.Instance;
  
  /**
  * Constructs a new instance with internal state necessary to
  * manage a wasm instance.
  *
  * Note that this does not actually instantiate the WebAssembly
  * instance or module, you'll need to call the `instantiate`
  * method below to "activate" this class.
  */
  constructor();
  
  /** 
  * This is a low-level method which can be used to add any
  * intrinsics necessary for this instance to operate to an
  * import object.
  *
  * The `import` object given here is expected to be used later
  * to actually instantiate the module this class corresponds to.
  * If the `instantiate` method below actually does the
  * instantiation then there's no need to call this method, but
  * if you're instantiating manually elsewhere then this can be
  * used to prepare the import object for external instantiation.
  */
  addToImports(imports: any): void;
  
  /**
  * Initializes this object with the provided WebAssembly
  * module/instance.
  *
  * This is intended to be a flexible method of instantiating
  * and completion of the initialization of this class. This
  * method must be called before interacting with the
  * WebAssembly object.
  *
  * The first argument to this method is where to get the
  * wasm from. This can be a whole bunch of different types,
  * for example:
  *
  * * A precompiled `WebAssembly.Module`
  * * A typed array buffer containing the wasm bytecode.
  * * A `Promise` of a `Response` which is used with
  *   `instantiateStreaming`
  * * A `Response` itself used with `instantiateStreaming`.
  * * An already instantiated `WebAssembly.Instance`
  *
  * If necessary the module is compiled, and if necessary the
  * module is instantiated. Whether or not it's necessary
  * depends on the type of argument provided to
  * instantiation.
  *
  * If instantiation is performed then the `imports` object
  * passed here is the list of imports used to instantiate
  * the instance. This method may add its own intrinsics to
  * this `imports` object too.
  */
  instantiate(
  module: WebAssembly.Module | BufferSource | Promise<Response> | Response | WebAssembly.Instance,
  imports?: any,
  ): Promise<void>;
  /**
  * A function that is called by the compiler/Forge while constructing the ML
  * pipeline to find out this node's inputs and outputs.
  * 
  * The implementation can use the provided node ID to retrieve the graph
  * context for this node.
  */
  graph(nodeId: string): Result<undefined, GraphError>;
  /**
  * The function called when doing inference.
  * 
  * The implementation can use the provided node ID to retrieve the kernel
  * context for this node.
  */
  kernel(nodeId: string): Result<undefined, KernelError>;
}
