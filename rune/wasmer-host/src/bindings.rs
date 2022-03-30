pub mod rune_v1 {
  #[allow(unused_imports)]
  use wit_bindgen_wasmer::{anyhow, wasmer};
  #[derive(Clone)]
  pub enum GraphError{
    InvalidArgument(InvalidArgument),
    Other(String),
  }
  impl std::fmt::Debug for GraphError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        GraphError::InvalidArgument(e) => {
          f.debug_tuple("GraphError::InvalidArgument").field(e).finish()
        }
        GraphError::Other(e) => {
          f.debug_tuple("GraphError::Other").field(e).finish()
        }
      }
    }
  }
  #[derive(Clone)]
  pub struct InvalidArgument {
    pub name: String,
    pub reason: BadArgumentReason,
  }
  impl std::fmt::Debug for InvalidArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("InvalidArgument").field("name", &self.name).field("reason", &self.reason).finish()}
  }
  /// The reason error is of type string that is thrown by the
  /// for example modulo(n: 0) => graph-error(invalid-argument(name: 'n', reason: invalid-value("N must be positive")))
  #[derive(Clone)]
  pub enum BadArgumentReason{
    NotFound,
    InvalidValue(String),
    Other(String),
  }
  impl std::fmt::Debug for BadArgumentReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        BadArgumentReason::NotFound => {
          f.debug_tuple("BadArgumentReason::NotFound").finish()
        }
        BadArgumentReason::InvalidValue(e) => {
          f.debug_tuple("BadArgumentReason::InvalidValue").field(e).finish()
        }
        BadArgumentReason::Other(e) => {
          f.debug_tuple("BadArgumentReason::Other").field(e).finish()
        }
      }
    }
  }
  #[derive(Clone)]
  pub enum KernelError{
    InvalidArgument(InvalidArgument),
    MissingInput(String),
    Other(String),
  }
  impl std::fmt::Debug for KernelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        KernelError::InvalidArgument(e) => {
          f.debug_tuple("KernelError::InvalidArgument").field(e).finish()
        }
        KernelError::MissingInput(e) => {
          f.debug_tuple("KernelError::MissingInput").field(e).finish()
        }
        KernelError::Other(e) => {
          f.debug_tuple("KernelError::Other").field(e).finish()
        }
      }
    }
  }
  /// Contextual information used when determining the ML / Data Processing pipeline.
  /// This is defined by the runtime but available for logic within the container (rune)
  #[derive(Debug)]
  pub struct GraphContext(wit_bindgen_wasmer::rt::ResourceIndex);
  #[derive(Debug)]
  pub struct KernelContext(wit_bindgen_wasmer::rt::ResourceIndex);

  /// Auxiliary data associated with the wasm exports.
  #[derive(Default)]
  pub struct RuneV1Data {

    index_slab0: wit_bindgen_wasmer::rt::IndexSlab,
    resource_slab0: wit_bindgen_wasmer::rt::ResourceSlab,
    dtor0: wasmer::LazyInit<wasmer::NativeFunc<i32, ()>>,

    index_slab1: wit_bindgen_wasmer::rt::IndexSlab,
    resource_slab1: wit_bindgen_wasmer::rt::ResourceSlab,
    dtor1: wasmer::LazyInit<wasmer::NativeFunc<i32, ()>>,
  }
  impl wasmer::WasmerEnv for RuneV1Data {
    fn init_with_instance(&mut self, instance: &wasmer::Instance) -> Result<(), wasmer::HostEnvInitError>{
      let _ = instance;
      self.dtor0.initialize(instance.exports.get_with_generics_weak("canonical_abi_drop_graph-context")?);
      self.dtor1.initialize(instance.exports.get_with_generics_weak("canonical_abi_drop_kernel-context")?);
      Ok(())}
  }
  impl Clone for RuneV1Data {
    fn clone(&self) -> Self {
      Self::default()
    }}
    pub struct RuneV1 {
      state: std::sync::Arc<std::sync::Mutex<RuneV1Data>>,
      func_canonical_abi_free: wasmer::NativeFunc<(i32, i32, i32), ()>,
      func_graph: wasmer::NativeFunc<i32, i32>,
      func_kernel: wasmer::NativeFunc<i32, i32>,
      func_start: wasmer::NativeFunc<(), ()>,
      memory: wasmer::Memory,
    }
    impl RuneV1 {

      /// Adds any intrinsics, if necessary for this exported wasm
      /// functionality to the `ImportObject` provided.
      ///
      /// This function returns the `RuneV1Data` which needs to be
      /// passed through to `RuneV1::new`.
      fn add_to_imports(
      store: &wasmer::Store,
      imports: &mut wasmer::ImportObject,
      ) -> std::sync::Arc<std::sync::Mutex<RuneV1Data>> {
        let state = std::sync::Arc::new(std::sync::Mutex::new(Default::default()));
        let mut canonical_abi = imports.get_namespace_exports("canonical_abi").unwrap_or_else(wasmer::Exports::new);

        canonical_abi.insert(
        "resource_drop_graph-context",
        wasmer::Function::new_native_with_env(store, state.clone(),
        move |env: &std::sync::Arc<std::sync::Mutex<RuneV1Data>>, idx: u32| -> Result<(), wasmer::RuntimeError> {
          let state = &mut *env.lock().unwrap();
          let resource_idx = state.index_slab0.remove(idx)?;
          let wasm = match state.resource_slab0.drop(resource_idx) {
            Some(wasm) => wasm,
            None => return Ok(()),
          };
          let dtor = state.dtor0.get_ref().unwrap();
          dtor.call(wasm)?;
          Ok(())
        },
        )
        );
        canonical_abi.insert(
        "resource_clone_graph-context",
        wasmer::Function::new_native_with_env(store, state.clone(),
        move |env: &std::sync::Arc<std::sync::Mutex<RuneV1Data>>, idx: u32| -> Result<u32, wasmer::RuntimeError>  {
          let state = &mut *env.lock().unwrap();
          let resource_idx = state.index_slab0.get(idx)?;
          state.resource_slab0.clone(resource_idx)?;
          Ok(state.index_slab0.insert(resource_idx))
        },
        )
        );
        canonical_abi.insert(
        "resource_get_graph-context",
        wasmer::Function::new_native_with_env(store, state.clone(),
        move |env: &std::sync::Arc<std::sync::Mutex<RuneV1Data>>, idx: u32| -> Result<i32, wasmer::RuntimeError>  {
          let state = &mut *env.lock().unwrap();
          let resource_idx = state.index_slab0.get(idx)?;
          Ok(state.resource_slab0.get(resource_idx))
        },
        )
        );
        canonical_abi.insert(
        "resource_new_graph-context",
        wasmer::Function::new_native_with_env(store, state.clone(),
        move |env: &std::sync::Arc<std::sync::Mutex<RuneV1Data>>, val: i32| -> Result<u32, wasmer::RuntimeError>  {
          let state = &mut *env.lock().unwrap();
          let resource_idx = state.resource_slab0.insert(val);
          Ok(state.index_slab0.insert(resource_idx))
        },
        )
        );

        canonical_abi.insert(
        "resource_drop_kernel-context",
        wasmer::Function::new_native_with_env(store, state.clone(),
        move |env: &std::sync::Arc<std::sync::Mutex<RuneV1Data>>, idx: u32| -> Result<(), wasmer::RuntimeError> {
          let state = &mut *env.lock().unwrap();
          let resource_idx = state.index_slab1.remove(idx)?;
          let wasm = match state.resource_slab1.drop(resource_idx) {
            Some(wasm) => wasm,
            None => return Ok(()),
          };
          let dtor = state.dtor1.get_ref().unwrap();
          dtor.call(wasm)?;
          Ok(())
        },
        )
        );
        canonical_abi.insert(
        "resource_clone_kernel-context",
        wasmer::Function::new_native_with_env(store, state.clone(),
        move |env: &std::sync::Arc<std::sync::Mutex<RuneV1Data>>, idx: u32| -> Result<u32, wasmer::RuntimeError>  {
          let state = &mut *env.lock().unwrap();
          let resource_idx = state.index_slab1.get(idx)?;
          state.resource_slab1.clone(resource_idx)?;
          Ok(state.index_slab1.insert(resource_idx))
        },
        )
        );
        canonical_abi.insert(
        "resource_get_kernel-context",
        wasmer::Function::new_native_with_env(store, state.clone(),
        move |env: &std::sync::Arc<std::sync::Mutex<RuneV1Data>>, idx: u32| -> Result<i32, wasmer::RuntimeError>  {
          let state = &mut *env.lock().unwrap();
          let resource_idx = state.index_slab1.get(idx)?;
          Ok(state.resource_slab1.get(resource_idx))
        },
        )
        );
        canonical_abi.insert(
        "resource_new_kernel-context",
        wasmer::Function::new_native_with_env(store, state.clone(),
        move |env: &std::sync::Arc<std::sync::Mutex<RuneV1Data>>, val: i32| -> Result<u32, wasmer::RuntimeError>  {
          let state = &mut *env.lock().unwrap();
          let resource_idx = state.resource_slab1.insert(val);
          Ok(state.index_slab1.insert(resource_idx))
        },
        )
        );
        imports.register("canonical_abi", canonical_abi);
        state
      }

      /// Instantiates the provided `module` using the specified
      /// parameters, wrapping up the result in a structure that
      /// translates between wasm and the host.
      ///
      /// The `imports` provided will have intrinsics added to it
      /// automatically, so it's not necessary to call
      /// `add_to_imports` beforehand. This function will
      /// instantiate the `module` otherwise using `imports`, and
      /// both an instance of this structure and the underlying
      /// `wasmer::Instance` will be returned.
      pub fn instantiate(
      store: &wasmer::Store,
      module: &wasmer::Module,
      imports: &mut wasmer::ImportObject,
      ) -> anyhow::Result<(Self, wasmer::Instance)> {
        let state = Self::add_to_imports(store, imports);
        let instance = wasmer::Instance::new(module, &*imports)?;
        Ok((Self::new(&instance, state)?, instance))
      }

      /// Low-level creation wrapper for wrapping up the exports
      /// of the `instance` provided in this structure of wasm
      /// exports.
      ///
      /// This function will extract exports from the `instance`
      /// and wrap them all up in the returned structure which can
      /// be used to interact with the wasm module.
      pub fn new(
      instance: &wasmer::Instance,
      state: std::sync::Arc<std::sync::Mutex<RuneV1Data>>,
      ) -> Result<Self, wasmer::ExportError> {
        let func_canonical_abi_free= instance.exports.get_native_function::<(i32, i32, i32), ()>("canonical_abi_free")?;
        let func_graph= instance.exports.get_native_function::<i32, i32>("graph")?;
        let func_kernel= instance.exports.get_native_function::<i32, i32>("kernel")?;
        let func_start= instance.exports.get_native_function::<(), ()>("start")?;
        let memory= instance.exports.get_memory("memory")?.clone();
        Ok(RuneV1{
          func_canonical_abi_free,
          func_graph,
          func_kernel,
          func_start,
          memory,
          state,

        })
      }
      /// A function called when the module is first loaded.
      pub fn start(&self,)-> Result<(), wasmer::RuntimeError> {
        self.func_start.call()?;
        Ok(())
      }
      /// A function that is called by the compiler/Forge while constructing the ML
      /// pipeline to find out this node's inputs and outputs.
      pub fn graph(&self,ctx: & GraphContext,)-> Result<Result<(),GraphError>, wasmer::RuntimeError> {
        let func_canonical_abi_free = &self.func_canonical_abi_free;
        let memory = &self.memory;

        let obj0 = ctx;
        let handle0 = {
          let state = &mut *self.state.lock().unwrap();
          state.resource_slab0.clone(obj0.0)?;
          state.index_slab0.insert(obj0.0)
        };
        let result1 = self.func_graph.call(handle0 as i32, )?;
        let load2 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result1 + 0)?;
        let load3 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result1 + 8)?;
        let load4 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result1 + 16)?;
        let load5 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result1 + 24)?;
        let load6 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result1 + 32)?;
        let load7 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result1 + 40)?;
        let load8 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result1 + 48)?;
        Ok(match load2 {
          0 => Ok(()),
          1 => Err(match load3 {
            0 => GraphError::InvalidArgument({
              let ptr9 = load4;
              let len9 = load5;
              InvalidArgument{name:String::from_utf8(
              copy_slice(
              memory,
              func_canonical_abi_free,
              ptr9, len9, 1
              )?
              )
              .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?, reason:match load6 {
                0 => BadArgumentReason::NotFound,
                1 => BadArgumentReason::InvalidValue({
                  let ptr10 = load7;
                  let len10 = load8;
                  String::from_utf8(
                  copy_slice(
                  memory,
                  func_canonical_abi_free,
                  ptr10, len10, 1
                  )?
                  )
                  .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?
                }),
                2 => BadArgumentReason::Other({
                  let ptr11 = load7;
                  let len11 = load8;
                  String::from_utf8(
                  copy_slice(
                  memory,
                  func_canonical_abi_free,
                  ptr11, len11, 1
                  )?
                  )
                  .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?
                }),
                _ => return Err(invalid_variant("BadArgumentReason")),
              }, }
            }),
            1 => GraphError::Other({
              let ptr12 = load4;
              let len12 = load5;
              String::from_utf8(
              copy_slice(
              memory,
              func_canonical_abi_free,
              ptr12, len12, 1
              )?
              )
              .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?
            }),
            _ => return Err(invalid_variant("GraphError")),
          }),
          _ => return Err(invalid_variant("Result")),
        })
      }
      /// The function called when doing inference.
      pub fn kernel(&self,ctx: & KernelContext,)-> Result<Result<(),KernelError>, wasmer::RuntimeError> {
        let func_canonical_abi_free = &self.func_canonical_abi_free;
        let memory = &self.memory;

        let obj0 = ctx;
        let handle0 = {
          let state = &mut *self.state.lock().unwrap();
          state.resource_slab1.clone(obj0.0)?;
          state.index_slab1.insert(obj0.0)
        };
        let result1 = self.func_kernel.call(handle0 as i32, )?;
        let load2 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result1 + 0)?;
        let load3 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result1 + 8)?;
        let load4 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result1 + 16)?;
        let load5 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result1 + 24)?;
        let load6 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result1 + 32)?;
        let load7 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result1 + 40)?;
        let load8 = unsafe { memory.data_unchecked_mut() }.load::<i32>(result1 + 48)?;
        Ok(match load2 {
          0 => Ok(()),
          1 => Err(match load3 {
            0 => KernelError::InvalidArgument({
              let ptr9 = load4;
              let len9 = load5;
              InvalidArgument{name:String::from_utf8(
              copy_slice(
              memory,
              func_canonical_abi_free,
              ptr9, len9, 1
              )?
              )
              .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?, reason:match load6 {
                0 => BadArgumentReason::NotFound,
                1 => BadArgumentReason::InvalidValue({
                  let ptr10 = load7;
                  let len10 = load8;
                  String::from_utf8(
                  copy_slice(
                  memory,
                  func_canonical_abi_free,
                  ptr10, len10, 1
                  )?
                  )
                  .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?
                }),
                2 => BadArgumentReason::Other({
                  let ptr11 = load7;
                  let len11 = load8;
                  String::from_utf8(
                  copy_slice(
                  memory,
                  func_canonical_abi_free,
                  ptr11, len11, 1
                  )?
                  )
                  .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?
                }),
                _ => return Err(invalid_variant("BadArgumentReason")),
              }, }
            }),
            1 => KernelError::MissingInput({
              let ptr12 = load4;
              let len12 = load5;
              String::from_utf8(
              copy_slice(
              memory,
              func_canonical_abi_free,
              ptr12, len12, 1
              )?
              )
              .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?
            }),
            2 => KernelError::Other({
              let ptr13 = load4;
              let len13 = load5;
              String::from_utf8(
              copy_slice(
              memory,
              func_canonical_abi_free,
              ptr13, len13, 1
              )?
              )
              .map_err(|_| wasmer::RuntimeError::new("invalid utf-8"))?
            }),
            _ => return Err(invalid_variant("KernelError")),
          }),
          _ => return Err(invalid_variant("Result")),
        })
      }

      /// Drops the host-owned handle to the resource
      /// specified.
      ///
      /// Note that this may execute the WebAssembly-defined
      /// destructor for this type. This also may not run
      /// the destructor if there are still other references
      /// to this type.
      pub fn drop_graph_context(
      &self,
      val: GraphContext,
      ) -> Result<(), wasmer::RuntimeError> {
        let data = &mut *self.state.lock().unwrap();
        let wasm = match data.resource_slab0.drop(val.0) {
          Some(val) => val,
          None => return Ok(()),
        };
        data.dtor0.get_ref().unwrap().call(wasm)?;
        Ok(())
      }

      /// Drops the host-owned handle to the resource
      /// specified.
      ///
      /// Note that this may execute the WebAssembly-defined
      /// destructor for this type. This also may not run
      /// the destructor if there are still other references
      /// to this type.
      pub fn drop_kernel_context(
      &self,
      val: KernelContext,
      ) -> Result<(), wasmer::RuntimeError> {
        let data = &mut *self.state.lock().unwrap();
        let wasm = match data.resource_slab1.drop(val.0) {
          Some(val) => val,
          None => return Ok(()),
        };
        data.dtor1.get_ref().unwrap().call(wasm)?;
        Ok(())
      }
    }
    use wit_bindgen_wasmer::rt::RawMem;
    use wit_bindgen_wasmer::rt::invalid_variant;
    use wit_bindgen_wasmer::rt::copy_slice;
  }
  pub mod runtime_v1 {
  #[allow(unused_imports)]
  use wit_bindgen_wasmer::{anyhow, wasmer};
  /// The various types of values a tensor may contain.
  #[repr(u8)]
  #[derive(Clone, Copy, PartialEq, Eq)]
  pub enum ElementType{
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    F32,
    U64,
    I64,
    F64,
    /// A string as UTF-8 encoded bytes.
    Utf8,
  }
  impl std::fmt::Debug for ElementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        ElementType::U8 => {
          f.debug_tuple("ElementType::U8").finish()
        }
        ElementType::I8 => {
          f.debug_tuple("ElementType::I8").finish()
        }
        ElementType::U16 => {
          f.debug_tuple("ElementType::U16").finish()
        }
        ElementType::I16 => {
          f.debug_tuple("ElementType::I16").finish()
        }
        ElementType::U32 => {
          f.debug_tuple("ElementType::U32").finish()
        }
        ElementType::I32 => {
          f.debug_tuple("ElementType::I32").finish()
        }
        ElementType::F32 => {
          f.debug_tuple("ElementType::F32").finish()
        }
        ElementType::U64 => {
          f.debug_tuple("ElementType::U64").finish()
        }
        ElementType::I64 => {
          f.debug_tuple("ElementType::I64").finish()
        }
        ElementType::F64 => {
          f.debug_tuple("ElementType::F64").finish()
        }
        ElementType::Utf8 => {
          f.debug_tuple("ElementType::Utf8").finish()
        }
      }
    }
  }
  /// The dimensions that a tensor may have.
  #[derive(Clone)]
  pub enum Dimensions<'a,>{
    /// There can be an arbitrary number of dimensions with arbitrary sizes.
    Dynamic,
    /// The tensor has a fixed rank with the provided dimension sizes.
    ///
    /// If a particular dimension's length is zero, that is interpreted as the
    /// dimension being allowed to have any arbitrary length.
    Fixed(&'a [Le<u32>]),
  }
  impl<'a,> std::fmt::Debug for Dimensions<'a,> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        Dimensions::Dynamic => {
          f.debug_tuple("Dimensions::Dynamic").finish()
        }
        Dimensions::Fixed(e) => {
          f.debug_tuple("Dimensions::Fixed").field(e).finish()
        }
      }
    }
  }
  /// A tensor with fixed dimensions.
  #[derive(Clone)]
  pub struct TensorParam<'a,> {
    pub dimensions: &'a [Le<u32>],
    pub data: TensorDataParam<'a,>,
  }
  impl<'a,> std::fmt::Debug for TensorParam<'a,> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("TensorParam").field("dimensions", &self.dimensions).field("data", &self.data).finish()}
  }
  /// A tensor with fixed dimensions.
  #[derive(Clone)]
  pub struct TensorResult {
    pub dimensions: Vec<u32>,
    pub data: TensorDataResult,
  }
  impl std::fmt::Debug for TensorResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("TensorResult").field("dimensions", &self.dimensions).field("data", &self.data).finish()}
  }
  /// The underlying data inside a tensor.
  #[derive(Clone)]
  pub enum TensorDataParam<'a,>{
    U8(&'a [u8]),
    I8(&'a [i8]),
    U16(&'a [Le<u16>]),
    I16(&'a [Le<i16>]),
    U32(&'a [Le<u32>]),
    I32(&'a [Le<i32>]),
    F32(&'a [Le<f32>]),
    U64(&'a [Le<u64>]),
    I64(&'a [Le<i64>]),
    F64(&'a [Le<f64>]),
    Utf8(Vec<&'a  str>),
  }
  impl<'a,> std::fmt::Debug for TensorDataParam<'a,> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        TensorDataParam::U8(e) => {
          f.debug_tuple("TensorDataParam::U8").field(e).finish()
        }
        TensorDataParam::I8(e) => {
          f.debug_tuple("TensorDataParam::I8").field(e).finish()
        }
        TensorDataParam::U16(e) => {
          f.debug_tuple("TensorDataParam::U16").field(e).finish()
        }
        TensorDataParam::I16(e) => {
          f.debug_tuple("TensorDataParam::I16").field(e).finish()
        }
        TensorDataParam::U32(e) => {
          f.debug_tuple("TensorDataParam::U32").field(e).finish()
        }
        TensorDataParam::I32(e) => {
          f.debug_tuple("TensorDataParam::I32").field(e).finish()
        }
        TensorDataParam::F32(e) => {
          f.debug_tuple("TensorDataParam::F32").field(e).finish()
        }
        TensorDataParam::U64(e) => {
          f.debug_tuple("TensorDataParam::U64").field(e).finish()
        }
        TensorDataParam::I64(e) => {
          f.debug_tuple("TensorDataParam::I64").field(e).finish()
        }
        TensorDataParam::F64(e) => {
          f.debug_tuple("TensorDataParam::F64").field(e).finish()
        }
        TensorDataParam::Utf8(e) => {
          f.debug_tuple("TensorDataParam::Utf8").field(e).finish()
        }
      }
    }
  }
  /// The underlying data inside a tensor.
  #[derive(Clone)]
  pub enum TensorDataResult{
    U8(Vec<u8>),
    I8(Vec<i8>),
    U16(Vec<u16>),
    I16(Vec<i16>),
    U32(Vec<u32>),
    I32(Vec<i32>),
    F32(Vec<f32>),
    U64(Vec<u64>),
    I64(Vec<i64>),
    F64(Vec<f64>),
    Utf8(Vec<String>),
  }
  impl std::fmt::Debug for TensorDataResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        TensorDataResult::U8(e) => {
          f.debug_tuple("TensorDataResult::U8").field(e).finish()
        }
        TensorDataResult::I8(e) => {
          f.debug_tuple("TensorDataResult::I8").field(e).finish()
        }
        TensorDataResult::U16(e) => {
          f.debug_tuple("TensorDataResult::U16").field(e).finish()
        }
        TensorDataResult::I16(e) => {
          f.debug_tuple("TensorDataResult::I16").field(e).finish()
        }
        TensorDataResult::U32(e) => {
          f.debug_tuple("TensorDataResult::U32").field(e).finish()
        }
        TensorDataResult::I32(e) => {
          f.debug_tuple("TensorDataResult::I32").field(e).finish()
        }
        TensorDataResult::F32(e) => {
          f.debug_tuple("TensorDataResult::F32").field(e).finish()
        }
        TensorDataResult::U64(e) => {
          f.debug_tuple("TensorDataResult::U64").field(e).finish()
        }
        TensorDataResult::I64(e) => {
          f.debug_tuple("TensorDataResult::I64").field(e).finish()
        }
        TensorDataResult::F64(e) => {
          f.debug_tuple("TensorDataResult::F64").field(e).finish()
        }
        TensorDataResult::Utf8(e) => {
          f.debug_tuple("TensorDataResult::Utf8").field(e).finish()
        }
      }
    }
  }
  pub trait RuntimeV1: Sized + wasmer::WasmerEnv + 'static{
    type ArgumentHint: std::fmt::Debug;
    type ArgumentMetadata: std::fmt::Debug;
    type GraphContext: std::fmt::Debug;
    type KernelContext: std::fmt::Debug;
    type Metadata: std::fmt::Debug;
    type TensorHint: std::fmt::Debug;
    type TensorMetadata: std::fmt::Debug;
    /// Create a new metadata object with the provided name and version number.
    ///
    /// The name should typically be one or two words that concisely describe
    /// the node and will be used as the human-friendly label shown to users
    /// when referring to it.
    fn metadata_new(&mut self,name: & str,version: & str,) -> Self::Metadata;

    /// A human-friendly description of the node.
    ///
    /// The text may include markdown.
    fn metadata_set_description(&mut self,self_: & Self::Metadata,description: & str,);

    /// A repository containing this node's source code.
    fn metadata_set_repository(&mut self,self_: & Self::Metadata,url: & str,);

    /// The node's home page.
    ///
    /// This will typically point to a `README` file or a page on the internet
    /// that users can go to when they want to find out more about the node.
    fn metadata_set_homepage(&mut self,self_: & Self::Metadata,url: & str,);

    /// Associate this node with a particular tag.
    ///
    /// Tags are typically used to assist in search and filtering.
    fn metadata_add_tag(&mut self,self_: & Self::Metadata,tag: & str,);

    /// Arguments this node accepts.
    fn metadata_add_argument(&mut self,self_: & Self::Metadata,arg: & Self::ArgumentMetadata,);

    /// Information about this node's input tensors.
    fn metadata_add_input(&mut self,self_: & Self::Metadata,metadata: & Self::TensorMetadata,);

    /// Information about this node's output tensors.
    fn metadata_add_output(&mut self,self_: & Self::Metadata,metadata: & Self::TensorMetadata,);

    /// Create a new named argument.
    fn argument_metadata_new(&mut self,name: & str,) -> Self::ArgumentMetadata;

    /// A human-friendly description of the argument.
    ///
    /// The text may include markdown.
    fn argument_metadata_set_description(&mut self,self_: & Self::ArgumentMetadata,description: & str,);

    /// A useful default value for this argument.
    fn argument_metadata_set_default_value(&mut self,self_: & Self::ArgumentMetadata,default_value: & str,);

    fn argument_metadata_add_hint(&mut self,self_: & Self::ArgumentMetadata,hint: & Self::ArgumentHint,);

    /// Create a new named tensor.
    fn tensor_metadata_new(&mut self,name: & str,) -> Self::TensorMetadata;

    /// A human-friendly description of the tensor.
    ///
    /// The text may include markdown.
    fn tensor_metadata_set_description(&mut self,self_: & Self::TensorMetadata,description: & str,);

    /// Add a hint that provides the runtime with contextual information about
    /// this node.
    fn tensor_metadata_add_hint(&mut self,self_: & Self::TensorMetadata,hint: & Self::TensorHint,);

    /// Hint to the runtime that a tensor may be displayed as an image.
    fn interpret_as_image(&mut self,) -> Self::TensorHint;

    /// Hint to the runtime that a tensor may be interpreted as an audio clip.
    fn interpret_as_audio(&mut self,) -> Self::TensorHint;

    /// Hint that a tensor may have a particular shape and the element types it
    /// supports.
    ///
    /// Note: This hint will be removed in the future in favour of a more flexible
    /// mechanism.
    fn supported_shapes(&mut self,supported_element_types: Vec<ElementType>,dimensions: Dimensions<'_,>,) -> Self::TensorHint;

    /// Hint to the runtime that an argument may be interpreted as a number in `[min, max]`
    fn interpret_as_number_in_range(&mut self,min: & str,max: & str,) -> Self::ArgumentHint;

    /// Hint to the runtime that an argument may be interpreted as a string in a defined list
    fn interpret_as_string_in_enum(&mut self,string_enum: Vec<& str>,) -> Self::ArgumentHint;

    /// Hint to the runtime that an argument may be interpreted as a non-negative number
    fn non_negative_number(&mut self,) -> Self::ArgumentHint;

    /// Register a node type with the runtime.
    fn register_node(&mut self,metadata: & Self::Metadata,);

    /// Name of the argument and returns the value of the argument
    /// Analogy: Getting an environment variable docker container
    fn graph_context_get_argument(&mut self,self_: & Self::GraphContext,name: & str,) -> Option<String>;

    fn graph_context_add_input_tensor(&mut self,self_: & Self::GraphContext,name: & str,element_type: ElementType,dimensions: Dimensions<'_,>,);

    fn graph_context_add_output_tensor(&mut self,self_: & Self::GraphContext,name: & str,element_type: ElementType,dimensions: Dimensions<'_,>,);

    fn kernel_context_get_argument(&mut self,self_: & Self::KernelContext,name: & str,) -> Option<String>;

    fn kernel_context_get_input_tensor(&mut self,self_: & Self::KernelContext,name: & str,) -> Option<TensorResult>;

    fn kernel_context_set_output_tensor(&mut self,self_: & Self::KernelContext,name: & str,tensor: TensorParam<'_,>,);

    fn drop_argument_hint(&mut self, state: Self::ArgumentHint) {
      drop(state);
    }
    fn drop_argument_metadata(&mut self, state: Self::ArgumentMetadata) {
      drop(state);
    }
    fn drop_graph_context(&mut self, state: Self::GraphContext) {
      drop(state);
    }
    fn drop_kernel_context(&mut self, state: Self::KernelContext) {
      drop(state);
    }
    fn drop_metadata(&mut self, state: Self::Metadata) {
      drop(state);
    }
    fn drop_tensor_hint(&mut self, state: Self::TensorHint) {
      drop(state);
    }
    fn drop_tensor_metadata(&mut self, state: Self::TensorMetadata) {
      drop(state);
    }
  }

  pub struct RuntimeV1Tables<T: RuntimeV1> {
    pub(crate) argument_hint_table: wit_bindgen_wasmer::Table<T::ArgumentHint>,
    pub(crate) argument_metadata_table: wit_bindgen_wasmer::Table<T::ArgumentMetadata>,
    pub(crate) graph_context_table: wit_bindgen_wasmer::Table<T::GraphContext>,
    pub(crate) kernel_context_table: wit_bindgen_wasmer::Table<T::KernelContext>,
    pub(crate) metadata_table: wit_bindgen_wasmer::Table<T::Metadata>,
    pub(crate) tensor_hint_table: wit_bindgen_wasmer::Table<T::TensorHint>,
    pub(crate) tensor_metadata_table: wit_bindgen_wasmer::Table<T::TensorMetadata>,
  }
  impl<T: RuntimeV1> Default for RuntimeV1Tables<T> {
    fn default() -> Self { Self {argument_hint_table: Default::default(),argument_metadata_table: Default::default(),graph_context_table: Default::default(),kernel_context_table: Default::default(),metadata_table: Default::default(),tensor_hint_table: Default::default(),tensor_metadata_table: Default::default(),}}}impl<T: RuntimeV1> Clone for RuntimeV1Tables<T> {
      fn clone(&self) -> Self {
        Self::default()
      }}

      pub fn add_to_imports<T>(store: &wasmer::Store, imports: &mut wasmer::ImportObject, data: T)
      where T: RuntimeV1
      {
        #[derive(Clone)]struct EnvWrapper<T: RuntimeV1> {
          data: T,
          tables: RuntimeV1Tables<T>,
          memory: wasmer::LazyInit<wasmer::Memory>,
          func_canonical_abi_realloc: wasmer::LazyInit<wasmer::NativeFunc<(i32, i32, i32, i32), i32>>,
        }
        unsafe impl<T: RuntimeV1> Send for EnvWrapper<T> {}
        unsafe impl<T: RuntimeV1> Sync for EnvWrapper<T> {}
        impl<T: RuntimeV1> wasmer::WasmerEnv for EnvWrapper<T> {
          fn init_with_instance(&mut self, instance: &wasmer::Instance) -> Result<(), wasmer::HostEnvInitError>{
            self.data.init_with_instance(instance)?;self.memory.initialize(instance.exports.get_with_generics_weak("memory")?);
            self.func_canonical_abi_realloc.initialize(instance.exports.get_with_generics_weak("canonical_abi_realloc")?);
            Ok(())}
        }
        let env = std::sync::Arc::new(std::sync::Mutex::new(EnvWrapper {
          data,
          tables: RuntimeV1Tables::default(),
          memory: wasmer::LazyInit::new(),
          func_canonical_abi_realloc: wasmer::LazyInit::new(),
        }));
        let mut exports = wasmer::Exports::new();
        exports.insert("metadata::new", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32,arg2:i32,arg3:i32| -> Result<i32, wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let mut _bc = unsafe { wit_bindgen_wasmer::BorrowChecker::new(env.memory.get_ref().unwrap().data_unchecked_mut()) };
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let ptr0 = arg0;
          let len0 = arg1;
          let ptr1 = arg2;
          let len1 = arg3;
          let param0 = _bc.slice_str(ptr0, len0)?;
          let param1 = _bc.slice_str(ptr1, len1)?;
          let result2 = host.metadata_new(param0, param1, );
          Ok(_tables.metadata_table.insert(result2) as i32)
        }));
        exports.insert("metadata::set-description", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32,arg2:i32| -> Result<(), wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let mut _bc = unsafe { wit_bindgen_wasmer::BorrowChecker::new(env.memory.get_ref().unwrap().data_unchecked_mut()) };
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let ptr0 = arg1;
          let len0 = arg2;
          let param0 = _tables.metadata_table.get((arg0) as u32).ok_or_else(|| {
            wasmer::RuntimeError::new("invalid handle index")
          })?;
          let param1 = _bc.slice_str(ptr0, len0)?;
          host.metadata_set_description(param0, param1, );
          Ok(())
        }));
        exports.insert("metadata::set-repository", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32,arg2:i32| -> Result<(), wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let mut _bc = unsafe { wit_bindgen_wasmer::BorrowChecker::new(env.memory.get_ref().unwrap().data_unchecked_mut()) };
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let ptr0 = arg1;
          let len0 = arg2;
          let param0 = _tables.metadata_table.get((arg0) as u32).ok_or_else(|| {
            wasmer::RuntimeError::new("invalid handle index")
          })?;
          let param1 = _bc.slice_str(ptr0, len0)?;
          host.metadata_set_repository(param0, param1, );
          Ok(())
        }));
        exports.insert("metadata::set-homepage", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32,arg2:i32| -> Result<(), wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let mut _bc = unsafe { wit_bindgen_wasmer::BorrowChecker::new(env.memory.get_ref().unwrap().data_unchecked_mut()) };
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let ptr0 = arg1;
          let len0 = arg2;
          let param0 = _tables.metadata_table.get((arg0) as u32).ok_or_else(|| {
            wasmer::RuntimeError::new("invalid handle index")
          })?;
          let param1 = _bc.slice_str(ptr0, len0)?;
          host.metadata_set_homepage(param0, param1, );
          Ok(())
        }));
        exports.insert("metadata::add-tag", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32,arg2:i32| -> Result<(), wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let mut _bc = unsafe { wit_bindgen_wasmer::BorrowChecker::new(env.memory.get_ref().unwrap().data_unchecked_mut()) };
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let ptr0 = arg1;
          let len0 = arg2;
          let param0 = _tables.metadata_table.get((arg0) as u32).ok_or_else(|| {
            wasmer::RuntimeError::new("invalid handle index")
          })?;
          let param1 = _bc.slice_str(ptr0, len0)?;
          host.metadata_add_tag(param0, param1, );
          Ok(())
        }));
        exports.insert("metadata::add-argument", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32| -> Result<(), wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let param0 = _tables.metadata_table.get((arg0) as u32).ok_or_else(|| {
            wasmer::RuntimeError::new("invalid handle index")
          })?;
          let param1 = _tables.argument_metadata_table.get((arg1) as u32).ok_or_else(|| {
            wasmer::RuntimeError::new("invalid handle index")
          })?;
          host.metadata_add_argument(param0, param1, );
          Ok(())
        }));
        exports.insert("metadata::add-input", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32| -> Result<(), wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let param0 = _tables.metadata_table.get((arg0) as u32).ok_or_else(|| {
            wasmer::RuntimeError::new("invalid handle index")
          })?;
          let param1 = _tables.tensor_metadata_table.get((arg1) as u32).ok_or_else(|| {
            wasmer::RuntimeError::new("invalid handle index")
          })?;
          host.metadata_add_input(param0, param1, );
          Ok(())
        }));
        exports.insert("metadata::add-output", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32| -> Result<(), wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let param0 = _tables.metadata_table.get((arg0) as u32).ok_or_else(|| {
            wasmer::RuntimeError::new("invalid handle index")
          })?;
          let param1 = _tables.tensor_metadata_table.get((arg1) as u32).ok_or_else(|| {
            wasmer::RuntimeError::new("invalid handle index")
          })?;
          host.metadata_add_output(param0, param1, );
          Ok(())
        }));
        exports.insert("argument-metadata::new", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32| -> Result<i32, wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let mut _bc = unsafe { wit_bindgen_wasmer::BorrowChecker::new(env.memory.get_ref().unwrap().data_unchecked_mut()) };
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let ptr0 = arg0;
          let len0 = arg1;
          let param0 = _bc.slice_str(ptr0, len0)?;
          let result1 = host.argument_metadata_new(param0, );
          Ok(_tables.argument_metadata_table.insert(result1) as i32)
        }));
        exports.insert("argument-metadata::set-description", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32,arg2:i32| -> Result<(), wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let mut _bc = unsafe { wit_bindgen_wasmer::BorrowChecker::new(env.memory.get_ref().unwrap().data_unchecked_mut()) };
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let ptr0 = arg1;
          let len0 = arg2;
          let param0 = _tables.argument_metadata_table.get((arg0) as u32).ok_or_else(|| {
            wasmer::RuntimeError::new("invalid handle index")
          })?;
          let param1 = _bc.slice_str(ptr0, len0)?;
          host.argument_metadata_set_description(param0, param1, );
          Ok(())
        }));
        exports.insert("argument-metadata::set-default-value", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32,arg2:i32| -> Result<(), wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let mut _bc = unsafe { wit_bindgen_wasmer::BorrowChecker::new(env.memory.get_ref().unwrap().data_unchecked_mut()) };
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let ptr0 = arg1;
          let len0 = arg2;
          let param0 = _tables.argument_metadata_table.get((arg0) as u32).ok_or_else(|| {
            wasmer::RuntimeError::new("invalid handle index")
          })?;
          let param1 = _bc.slice_str(ptr0, len0)?;
          host.argument_metadata_set_default_value(param0, param1, );
          Ok(())
        }));
        exports.insert("argument-metadata::add-hint", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32| -> Result<(), wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let param0 = _tables.argument_metadata_table.get((arg0) as u32).ok_or_else(|| {
            wasmer::RuntimeError::new("invalid handle index")
          })?;
          let param1 = _tables.argument_hint_table.get((arg1) as u32).ok_or_else(|| {
            wasmer::RuntimeError::new("invalid handle index")
          })?;
          host.argument_metadata_add_hint(param0, param1, );
          Ok(())
        }));
        exports.insert("tensor-metadata::new", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32| -> Result<i32, wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let mut _bc = unsafe { wit_bindgen_wasmer::BorrowChecker::new(env.memory.get_ref().unwrap().data_unchecked_mut()) };
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let ptr0 = arg0;
          let len0 = arg1;
          let param0 = _bc.slice_str(ptr0, len0)?;
          let result1 = host.tensor_metadata_new(param0, );
          Ok(_tables.tensor_metadata_table.insert(result1) as i32)
        }));
        exports.insert("tensor-metadata::set-description", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32,arg2:i32| -> Result<(), wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let mut _bc = unsafe { wit_bindgen_wasmer::BorrowChecker::new(env.memory.get_ref().unwrap().data_unchecked_mut()) };
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let ptr0 = arg1;
          let len0 = arg2;
          let param0 = _tables.tensor_metadata_table.get((arg0) as u32).ok_or_else(|| {
            wasmer::RuntimeError::new("invalid handle index")
          })?;
          let param1 = _bc.slice_str(ptr0, len0)?;
          host.tensor_metadata_set_description(param0, param1, );
          Ok(())
        }));
        exports.insert("tensor-metadata::add-hint", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32| -> Result<(), wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let param0 = _tables.tensor_metadata_table.get((arg0) as u32).ok_or_else(|| {
            wasmer::RuntimeError::new("invalid handle index")
          })?;
          let param1 = _tables.tensor_hint_table.get((arg1) as u32).ok_or_else(|| {
            wasmer::RuntimeError::new("invalid handle index")
          })?;
          host.tensor_metadata_add_hint(param0, param1, );
          Ok(())
        }));
        exports.insert("interpret-as-image", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<i32, wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let result0 = host.interpret_as_image();
          Ok(_tables.tensor_hint_table.insert(result0) as i32)
        }));
        exports.insert("interpret-as-audio", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<i32, wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let result0 = host.interpret_as_audio();
          Ok(_tables.tensor_hint_table.insert(result0) as i32)
        }));
        exports.insert("supported-shapes", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32,arg2:i32,arg3:i32,arg4:i32| -> Result<i32, wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let mut _bc = unsafe { wit_bindgen_wasmer::BorrowChecker::new(env.memory.get_ref().unwrap().data_unchecked_mut()) };
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let len1 = arg1;
          let base1 = arg0;
          let mut result1 = Vec::with_capacity(len1 as usize);
          for i in 0..len1 {
            let base = base1 + i *1;
            result1.push({
              let load0 = _bc.load::<u8>(base + 0)?;
              match i32::from(load0) {
                0 => ElementType::U8,
                1 => ElementType::I8,
                2 => ElementType::U16,
                3 => ElementType::I16,
                4 => ElementType::U32,
                5 => ElementType::I32,
                6 => ElementType::F32,
                7 => ElementType::U64,
                8 => ElementType::I64,
                9 => ElementType::F64,
                10 => ElementType::Utf8,
                _ => return Err(invalid_variant("ElementType")),
              }
            });
          }
          let param0 = result1;
          let param1 = match arg2 {
            0 => Dimensions::Dynamic,
            1 => Dimensions::Fixed({
              let ptr2 = arg3;
              let len2 = arg4;
              _bc.slice(ptr2, len2)?
            }),
            _ => return Err(invalid_variant("Dimensions")),
          };
          let result3 = host.supported_shapes(param0, param1, );
          Ok(_tables.tensor_hint_table.insert(result3) as i32)
        }));
        exports.insert("interpret-as-number-in-range", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32,arg2:i32,arg3:i32| -> Result<i32, wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let mut _bc = unsafe { wit_bindgen_wasmer::BorrowChecker::new(env.memory.get_ref().unwrap().data_unchecked_mut()) };
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let ptr0 = arg0;
          let len0 = arg1;
          let ptr1 = arg2;
          let len1 = arg3;
          let param0 = _bc.slice_str(ptr0, len0)?;
          let param1 = _bc.slice_str(ptr1, len1)?;
          let result2 = host.interpret_as_number_in_range(param0, param1, );
          Ok(_tables.argument_hint_table.insert(result2) as i32)
        }));
        exports.insert("interpret-as-string-in-enum", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32| -> Result<i32, wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let mut _bc = unsafe { wit_bindgen_wasmer::BorrowChecker::new(env.memory.get_ref().unwrap().data_unchecked_mut()) };
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let len3 = arg1;
          let base3 = arg0;
          let mut result3 = Vec::with_capacity(len3 as usize);
          for i in 0..len3 {
            let base = base3 + i *8;
            result3.push({
              let load0 = _bc.load::<i32>(base + 0)?;
              let load1 = _bc.load::<i32>(base + 4)?;
              let ptr2 = load0;
              let len2 = load1;
              _bc.slice_str(ptr2, len2)?
            });
          }
          let param0 = result3;
          let result4 = host.interpret_as_string_in_enum(param0, );
          Ok(_tables.argument_hint_table.insert(result4) as i32)
        }));
        exports.insert("non-negative-number", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>| -> Result<i32, wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let result0 = host.non_negative_number();
          Ok(_tables.argument_hint_table.insert(result0) as i32)
        }));
        exports.insert("register-node", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32| -> Result<(), wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let param0 = _tables.metadata_table.get((arg0) as u32).ok_or_else(|| {
            wasmer::RuntimeError::new("invalid handle index")
          })?;
          host.register_node(param0, );
          Ok(())
        }));
        exports.insert("graph-context::get-argument", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32,arg2:i32,arg3:i32| -> Result<(), wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let func_canonical_abi_realloc = env.func_canonical_abi_realloc.get_ref().unwrap();
          let mut _bc = unsafe { wit_bindgen_wasmer::BorrowChecker::new(env.memory.get_ref().unwrap().data_unchecked_mut()) };
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let ptr0 = arg1;
          let len0 = arg2;
          let param0 = _tables.graph_context_table.get((arg0) as u32).ok_or_else(|| {
            wasmer::RuntimeError::new("invalid handle index")
          })?;
          let param1 = _bc.slice_str(ptr0, len0)?;
          let result1 = host.graph_context_get_argument(param0, param1, );
          let (result3_0,result3_1,result3_2,) = match result1{
            None => { (0i32, 0i32, 0i32)}
            Some(e) => { {
              let vec2 = e;
              let ptr2 = func_canonical_abi_realloc.call(0, 0, 1, (vec2.len() as i32) * 1)?;
              let caller_memory = unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
              caller_memory.store_many(ptr2, vec2.as_ref())?;
              (1i32, ptr2, vec2.len() as i32)
            }}
          };
          let caller_memory = unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
          caller_memory.store(arg3 + 16, wit_bindgen_wasmer::rt::as_i32(result3_2))?;
          caller_memory.store(arg3 + 8, wit_bindgen_wasmer::rt::as_i32(result3_1))?;
          caller_memory.store(arg3 + 0, wit_bindgen_wasmer::rt::as_i32(result3_0))?;
          Ok(())
        }));
        exports.insert("graph-context::add-input-tensor", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32,arg2:i32,arg3:i32,arg4:i32,arg5:i32,arg6:i32| -> Result<(), wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let mut _bc = unsafe { wit_bindgen_wasmer::BorrowChecker::new(env.memory.get_ref().unwrap().data_unchecked_mut()) };
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let ptr0 = arg1;
          let len0 = arg2;
          let param0 = _tables.graph_context_table.get((arg0) as u32).ok_or_else(|| {
            wasmer::RuntimeError::new("invalid handle index")
          })?;
          let param1 = _bc.slice_str(ptr0, len0)?;
          let param2 = match arg3 {
            0 => ElementType::U8,
            1 => ElementType::I8,
            2 => ElementType::U16,
            3 => ElementType::I16,
            4 => ElementType::U32,
            5 => ElementType::I32,
            6 => ElementType::F32,
            7 => ElementType::U64,
            8 => ElementType::I64,
            9 => ElementType::F64,
            10 => ElementType::Utf8,
            _ => return Err(invalid_variant("ElementType")),
          };
          let param3 = match arg4 {
            0 => Dimensions::Dynamic,
            1 => Dimensions::Fixed({
              let ptr1 = arg5;
              let len1 = arg6;
              _bc.slice(ptr1, len1)?
            }),
            _ => return Err(invalid_variant("Dimensions")),
          };
          host.graph_context_add_input_tensor(param0, param1, param2, param3, );
          Ok(())
        }));
        exports.insert("graph-context::add-output-tensor", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32,arg2:i32,arg3:i32,arg4:i32,arg5:i32,arg6:i32| -> Result<(), wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let mut _bc = unsafe { wit_bindgen_wasmer::BorrowChecker::new(env.memory.get_ref().unwrap().data_unchecked_mut()) };
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let ptr0 = arg1;
          let len0 = arg2;
          let param0 = _tables.graph_context_table.get((arg0) as u32).ok_or_else(|| {
            wasmer::RuntimeError::new("invalid handle index")
          })?;
          let param1 = _bc.slice_str(ptr0, len0)?;
          let param2 = match arg3 {
            0 => ElementType::U8,
            1 => ElementType::I8,
            2 => ElementType::U16,
            3 => ElementType::I16,
            4 => ElementType::U32,
            5 => ElementType::I32,
            6 => ElementType::F32,
            7 => ElementType::U64,
            8 => ElementType::I64,
            9 => ElementType::F64,
            10 => ElementType::Utf8,
            _ => return Err(invalid_variant("ElementType")),
          };
          let param3 = match arg4 {
            0 => Dimensions::Dynamic,
            1 => Dimensions::Fixed({
              let ptr1 = arg5;
              let len1 = arg6;
              _bc.slice(ptr1, len1)?
            }),
            _ => return Err(invalid_variant("Dimensions")),
          };
          host.graph_context_add_output_tensor(param0, param1, param2, param3, );
          Ok(())
        }));
        exports.insert("kernel-context::get-argument", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32,arg2:i32,arg3:i32| -> Result<(), wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let func_canonical_abi_realloc = env.func_canonical_abi_realloc.get_ref().unwrap();
          let mut _bc = unsafe { wit_bindgen_wasmer::BorrowChecker::new(env.memory.get_ref().unwrap().data_unchecked_mut()) };
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let ptr0 = arg1;
          let len0 = arg2;
          let param0 = _tables.kernel_context_table.get((arg0) as u32).ok_or_else(|| {
            wasmer::RuntimeError::new("invalid handle index")
          })?;
          let param1 = _bc.slice_str(ptr0, len0)?;
          let result1 = host.kernel_context_get_argument(param0, param1, );
          let (result3_0,result3_1,result3_2,) = match result1{
            None => { (0i32, 0i32, 0i32)}
            Some(e) => { {
              let vec2 = e;
              let ptr2 = func_canonical_abi_realloc.call(0, 0, 1, (vec2.len() as i32) * 1)?;
              let caller_memory = unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
              caller_memory.store_many(ptr2, vec2.as_ref())?;
              (1i32, ptr2, vec2.len() as i32)
            }}
          };
          let caller_memory = unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
          caller_memory.store(arg3 + 16, wit_bindgen_wasmer::rt::as_i32(result3_2))?;
          caller_memory.store(arg3 + 8, wit_bindgen_wasmer::rt::as_i32(result3_1))?;
          caller_memory.store(arg3 + 0, wit_bindgen_wasmer::rt::as_i32(result3_0))?;
          Ok(())
        }));
        exports.insert("kernel-context::get-input-tensor", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32,arg2:i32,arg3:i32| -> Result<(), wasmer::RuntimeError> {
          let env = &mut *env.lock().unwrap();
          let func_canonical_abi_realloc = env.func_canonical_abi_realloc.get_ref().unwrap();
          let mut _bc = unsafe { wit_bindgen_wasmer::BorrowChecker::new(env.memory.get_ref().unwrap().data_unchecked_mut()) };
          let host = &mut env.data;
          let _tables = &mut env.tables;
          let ptr0 = arg1;
          let len0 = arg2;
          let param0 = _tables.kernel_context_table.get((arg0) as u32).ok_or_else(|| {
            wasmer::RuntimeError::new("invalid handle index")
          })?;
          let param1 = _bc.slice_str(ptr0, len0)?;
          let result1 = host.kernel_context_get_input_tensor(param0, param1, );
          let (result17_0,result17_1,result17_2,result17_3,result17_4,result17_5,) = match result1{
            None => { (0i32, 0i32, 0i32, 0i32, 0i32, 0i32)}
            Some(e) => { {
              let TensorResult{ dimensions:dimensions2, data:data2, } = e;
              let vec3 = dimensions2;
              let ptr3 = func_canonical_abi_realloc.call(0, 0, 4, (vec3.len() as i32) * 4)?;
              let caller_memory = unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
              caller_memory.store_many(ptr3, vec3.as_ref())?;
              let (result16_0,result16_1,result16_2,) = match data2{
                TensorDataResult::U8(e) => { {
                  let vec4 = e;
                  let ptr4 = func_canonical_abi_realloc.call(0, 0, 1, (vec4.len() as i32) * 1)?;
                  let caller_memory = unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                  caller_memory.store_many(ptr4, vec4.as_ref())?;
                  (0i32, ptr4, vec4.len() as i32)
                }}
                TensorDataResult::I8(e) => { {
                  let vec5 = e;
                  let ptr5 = func_canonical_abi_realloc.call(0, 0, 1, (vec5.len() as i32) * 1)?;
                  let caller_memory = unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                  caller_memory.store_many(ptr5, vec5.as_ref())?;
                  (1i32, ptr5, vec5.len() as i32)
                }}
                TensorDataResult::U16(e) => { {
                  let vec6 = e;
                  let ptr6 = func_canonical_abi_realloc.call(0, 0, 2, (vec6.len() as i32) * 2)?;
                  let caller_memory = unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                  caller_memory.store_many(ptr6, vec6.as_ref())?;
                  (2i32, ptr6, vec6.len() as i32)
                }}
                TensorDataResult::I16(e) => { {
                  let vec7 = e;
                  let ptr7 = func_canonical_abi_realloc.call(0, 0, 2, (vec7.len() as i32) * 2)?;
                  let caller_memory = unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                  caller_memory.store_many(ptr7, vec7.as_ref())?;
                  (3i32, ptr7, vec7.len() as i32)
                }}
                TensorDataResult::U32(e) => { {
                  let vec8 = e;
                  let ptr8 = func_canonical_abi_realloc.call(0, 0, 4, (vec8.len() as i32) * 4)?;
                  let caller_memory = unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                  caller_memory.store_many(ptr8, vec8.as_ref())?;
                  (4i32, ptr8, vec8.len() as i32)
                }}
                TensorDataResult::I32(e) => { {
                  let vec9 = e;
                  let ptr9 = func_canonical_abi_realloc.call(0, 0, 4, (vec9.len() as i32) * 4)?;
                  let caller_memory = unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                  caller_memory.store_many(ptr9, vec9.as_ref())?;
                  (5i32, ptr9, vec9.len() as i32)
                }}
                TensorDataResult::F32(e) => { {
                  let vec10 = e;
                  let ptr10 = func_canonical_abi_realloc.call(0, 0, 4, (vec10.len() as i32) * 4)?;
                  let caller_memory = unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                  caller_memory.store_many(ptr10, vec10.as_ref())?;
                  (6i32, ptr10, vec10.len() as i32)
                }}
                TensorDataResult::U64(e) => { {
                  let vec11 = e;
                  let ptr11 = func_canonical_abi_realloc.call(0, 0, 8, (vec11.len() as i32) * 8)?;
                  let caller_memory = unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                  caller_memory.store_many(ptr11, vec11.as_ref())?;
                  (7i32, ptr11, vec11.len() as i32)
                }}
                TensorDataResult::I64(e) => { {
                  let vec12 = e;
                  let ptr12 = func_canonical_abi_realloc.call(0, 0, 8, (vec12.len() as i32) * 8)?;
                  let caller_memory = unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                  caller_memory.store_many(ptr12, vec12.as_ref())?;
                  (8i32, ptr12, vec12.len() as i32)
                }}
                TensorDataResult::F64(e) => { {
                  let vec13 = e;
                  let ptr13 = func_canonical_abi_realloc.call(0, 0, 8, (vec13.len() as i32) * 8)?;
                  let caller_memory = unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                  caller_memory.store_many(ptr13, vec13.as_ref())?;
                  (9i32, ptr13, vec13.len() as i32)
                }}
                TensorDataResult::Utf8(e) => { {
                  let vec15 = e;
                  let len15 = vec15.len() as i32;
                  let result15 = func_canonical_abi_realloc.call(0, 0, 4, len15 * 8)?;
                  for (i, e) in vec15.into_iter().enumerate() {
                    let base = result15 + (i as i32) * 8;
                    {
                      let vec14 = e;
                      let ptr14 = func_canonical_abi_realloc.call(0, 0, 1, (vec14.len() as i32) * 1)?;
                      let caller_memory = unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
                      caller_memory.store_many(ptr14, vec14.as_ref())?;
                      caller_memory.store(base + 4, wit_bindgen_wasmer::rt::as_i32(vec14.len() as i32))?;
                      caller_memory.store(base + 0, wit_bindgen_wasmer::rt::as_i32(ptr14))?;
                    }}(10i32, result15, len15)
                  }}
                };
                (1i32, ptr3, vec3.len() as i32, result16_0, result16_1, result16_2)
              }}
            };
            let caller_memory = unsafe { env.memory.get_ref().unwrap().data_unchecked_mut() };
            caller_memory.store(arg3 + 40, wit_bindgen_wasmer::rt::as_i32(result17_5))?;
            caller_memory.store(arg3 + 32, wit_bindgen_wasmer::rt::as_i32(result17_4))?;
            caller_memory.store(arg3 + 24, wit_bindgen_wasmer::rt::as_i32(result17_3))?;
            caller_memory.store(arg3 + 16, wit_bindgen_wasmer::rt::as_i32(result17_2))?;
            caller_memory.store(arg3 + 8, wit_bindgen_wasmer::rt::as_i32(result17_1))?;
            caller_memory.store(arg3 + 0, wit_bindgen_wasmer::rt::as_i32(result17_0))?;
            Ok(())
          }));
          exports.insert("kernel-context::set-output-tensor", wasmer::Function::new_native_with_env(store, env.clone(), move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>,arg0:i32,arg1:i32,arg2:i32,arg3:i32,arg4:i32,arg5:i32,arg6:i32,arg7:i32| -> Result<(), wasmer::RuntimeError> {
            let env = &mut *env.lock().unwrap();
            let mut _bc = unsafe { wit_bindgen_wasmer::BorrowChecker::new(env.memory.get_ref().unwrap().data_unchecked_mut()) };
            let host = &mut env.data;
            let _tables = &mut env.tables;
            let ptr0 = arg1;
            let len0 = arg2;
            let ptr1 = arg3;
            let len1 = arg4;
            let param0 = _tables.kernel_context_table.get((arg0) as u32).ok_or_else(|| {
              wasmer::RuntimeError::new("invalid handle index")
            })?;
            let param1 = _bc.slice_str(ptr0, len0)?;
            let param2 = TensorParam{dimensions:_bc.slice(ptr1, len1)?, data:match arg5 {
              0 => TensorDataParam::U8({
                let ptr2 = arg6;
                let len2 = arg7;
                _bc.slice(ptr2, len2)?
              }),
              1 => TensorDataParam::I8({
                let ptr3 = arg6;
                let len3 = arg7;
                _bc.slice(ptr3, len3)?
              }),
              2 => TensorDataParam::U16({
                let ptr4 = arg6;
                let len4 = arg7;
                _bc.slice(ptr4, len4)?
              }),
              3 => TensorDataParam::I16({
                let ptr5 = arg6;
                let len5 = arg7;
                _bc.slice(ptr5, len5)?
              }),
              4 => TensorDataParam::U32({
                let ptr6 = arg6;
                let len6 = arg7;
                _bc.slice(ptr6, len6)?
              }),
              5 => TensorDataParam::I32({
                let ptr7 = arg6;
                let len7 = arg7;
                _bc.slice(ptr7, len7)?
              }),
              6 => TensorDataParam::F32({
                let ptr8 = arg6;
                let len8 = arg7;
                _bc.slice(ptr8, len8)?
              }),
              7 => TensorDataParam::U64({
                let ptr9 = arg6;
                let len9 = arg7;
                _bc.slice(ptr9, len9)?
              }),
              8 => TensorDataParam::I64({
                let ptr10 = arg6;
                let len10 = arg7;
                _bc.slice(ptr10, len10)?
              }),
              9 => TensorDataParam::F64({
                let ptr11 = arg6;
                let len11 = arg7;
                _bc.slice(ptr11, len11)?
              }),
              10 => TensorDataParam::Utf8({
                let len15 = arg7;
                let base15 = arg6;
                let mut result15 = Vec::with_capacity(len15 as usize);
                for i in 0..len15 {
                  let base = base15 + i *8;
                  result15.push({
                    let load12 = _bc.load::<i32>(base + 0)?;
                    let load13 = _bc.load::<i32>(base + 4)?;
                    let ptr14 = load12;
                    let len14 = load13;
                    _bc.slice_str(ptr14, len14)?
                  });
                }
                result15
              }),
              _ => return Err(invalid_variant("TensorData")),
            }, };
            host.kernel_context_set_output_tensor(param0, param1, param2, );
            Ok(())
          }));
          imports.register("runtime-v1", exports);
          let mut canonical_abi = imports.get_namespace_exports("canonical_abi").unwrap_or_else(wasmer::Exports::new);
          canonical_abi.insert(
          "resource_drop_argument-hint",
          wasmer::Function::new_native_with_env(store, env.clone(),
          move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>, handle: u32| -> Result<(), wasmer::RuntimeError> {
            let env = &mut *env.lock().unwrap();
            let handle = env
            .tables
            .argument_hint_table
            .remove(handle)
            .map_err(|e| {
              wasmer::RuntimeError::new(format!("failed to remove handle: {}", e))
            })?;
            env.data.drop_argument_hint(handle);
            Ok(())
          }
          )
          );
          canonical_abi.insert(
          "resource_drop_argument-metadata",
          wasmer::Function::new_native_with_env(store, env.clone(),
          move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>, handle: u32| -> Result<(), wasmer::RuntimeError> {
            let env = &mut *env.lock().unwrap();
            let handle = env
            .tables
            .argument_metadata_table
            .remove(handle)
            .map_err(|e| {
              wasmer::RuntimeError::new(format!("failed to remove handle: {}", e))
            })?;
            env.data.drop_argument_metadata(handle);
            Ok(())
          }
          )
          );
          canonical_abi.insert(
          "resource_drop_graph-context",
          wasmer::Function::new_native_with_env(store, env.clone(),
          move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>, handle: u32| -> Result<(), wasmer::RuntimeError> {
            let env = &mut *env.lock().unwrap();
            let handle = env
            .tables
            .graph_context_table
            .remove(handle)
            .map_err(|e| {
              wasmer::RuntimeError::new(format!("failed to remove handle: {}", e))
            })?;
            env.data.drop_graph_context(handle);
            Ok(())
          }
          )
          );
          canonical_abi.insert(
          "resource_drop_kernel-context",
          wasmer::Function::new_native_with_env(store, env.clone(),
          move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>, handle: u32| -> Result<(), wasmer::RuntimeError> {
            let env = &mut *env.lock().unwrap();
            let handle = env
            .tables
            .kernel_context_table
            .remove(handle)
            .map_err(|e| {
              wasmer::RuntimeError::new(format!("failed to remove handle: {}", e))
            })?;
            env.data.drop_kernel_context(handle);
            Ok(())
          }
          )
          );
          canonical_abi.insert(
          "resource_drop_metadata",
          wasmer::Function::new_native_with_env(store, env.clone(),
          move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>, handle: u32| -> Result<(), wasmer::RuntimeError> {
            let env = &mut *env.lock().unwrap();
            let handle = env
            .tables
            .metadata_table
            .remove(handle)
            .map_err(|e| {
              wasmer::RuntimeError::new(format!("failed to remove handle: {}", e))
            })?;
            env.data.drop_metadata(handle);
            Ok(())
          }
          )
          );
          canonical_abi.insert(
          "resource_drop_tensor-hint",
          wasmer::Function::new_native_with_env(store, env.clone(),
          move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>, handle: u32| -> Result<(), wasmer::RuntimeError> {
            let env = &mut *env.lock().unwrap();
            let handle = env
            .tables
            .tensor_hint_table
            .remove(handle)
            .map_err(|e| {
              wasmer::RuntimeError::new(format!("failed to remove handle: {}", e))
            })?;
            env.data.drop_tensor_hint(handle);
            Ok(())
          }
          )
          );
          canonical_abi.insert(
          "resource_drop_tensor-metadata",
          wasmer::Function::new_native_with_env(store, env.clone(),
          move |env: &std::sync::Arc<std::sync::Mutex<EnvWrapper<T>>>, handle: u32| -> Result<(), wasmer::RuntimeError> {
            let env = &mut *env.lock().unwrap();
            let handle = env
            .tables
            .tensor_metadata_table
            .remove(handle)
            .map_err(|e| {
              wasmer::RuntimeError::new(format!("failed to remove handle: {}", e))
            })?;
            env.data.drop_tensor_metadata(handle);
            Ok(())
          }
          )
          );
          imports.register("canonical_abi", canonical_abi);
        }
        use wit_bindgen_wasmer::rt::RawMem;
        use wit_bindgen_wasmer::rt::invalid_variant;
        use wit_bindgen_wasmer::Le;
        use wit_bindgen_wasmer::rt::copy_slice;
      }
