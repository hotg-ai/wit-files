mod runtime_v1 {
    /// The various types of values a tensor may contain.
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum ElementType {
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
                ElementType::U8 => f.debug_tuple("ElementType::U8").finish(),
                ElementType::I8 => f.debug_tuple("ElementType::I8").finish(),
                ElementType::U16 => f.debug_tuple("ElementType::U16").finish(),
                ElementType::I16 => f.debug_tuple("ElementType::I16").finish(),
                ElementType::U32 => f.debug_tuple("ElementType::U32").finish(),
                ElementType::I32 => f.debug_tuple("ElementType::I32").finish(),
                ElementType::F32 => f.debug_tuple("ElementType::F32").finish(),
                ElementType::U64 => f.debug_tuple("ElementType::U64").finish(),
                ElementType::I64 => f.debug_tuple("ElementType::I64").finish(),
                ElementType::F64 => f.debug_tuple("ElementType::F64").finish(),
                ElementType::Utf8 => f.debug_tuple("ElementType::Utf8").finish(),
            }
        }
    }
    /// The dimensions that a tensor may have.
    pub enum Dimensions<'a> {
        /// There can be an arbitrary number of dimensions with arbitrary sizes.
        Dynamic,
        /// The tensor has a fixed rank with the provided dimension sizes.
        ///
        /// If a particular dimension's length is zero, that is interpreted as the
        /// dimension being allowed to have any arbitrary length.
        Fixed(&'a [u32]),
    }
    impl<'a> std::fmt::Debug for Dimensions<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Dimensions::Dynamic => f.debug_tuple("Dimensions::Dynamic").finish(),
                Dimensions::Fixed(e) => f.debug_tuple("Dimensions::Fixed").field(e).finish(),
            }
        }
    }
    /// How will an argument be interpreted?
    ///
    /// All nodes receive arguments as strings. This enum lets the node hint to the
    /// runtime that an argument may be formatted in a particular way.
    #[repr(u8)]
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum ArgumentType {
        /// An unsigned integer.
        UnsignedInteger,
        /// An integer, possibly signed.
        Integer,
        /// A decimal number.
        Float,
        /// A short string.
        String,
        /// A multi-line string.
        LongString,
    }
    impl std::fmt::Debug for ArgumentType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ArgumentType::UnsignedInteger => {
                    f.debug_tuple("ArgumentType::UnsignedInteger").finish()
                }
                ArgumentType::Integer => f.debug_tuple("ArgumentType::Integer").finish(),
                ArgumentType::Float => f.debug_tuple("ArgumentType::Float").finish(),
                ArgumentType::String => f.debug_tuple("ArgumentType::String").finish(),
                ArgumentType::LongString => f.debug_tuple("ArgumentType::LongString").finish(),
            }
        }
    }
    /// A tensor with fixed dimensions.
    #[derive(Clone)]
    pub struct TensorParam<'a> {
        /// The type of data this tensor contains.
        pub element_type: ElementType,
        /// The tensor's dimensions.
        pub dimensions: &'a [u32],
        /// The raw bytes used by this tensor, as little-endian values.
        ///
        /// Note: because string tensors are represented using a variable-length
        /// encoding where each string is prefixed by its length as a little-endian
        /// `u32`.
        pub buffer: &'a [u8],
    }
    impl<'a> std::fmt::Debug for TensorParam<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TensorParam")
                .field("element-type", &self.element_type)
                .field("dimensions", &self.dimensions)
                .field("buffer", &self.buffer)
                .finish()
        }
    }
    /// A tensor with fixed dimensions.
    #[derive(Clone)]
    pub struct TensorResult {
        /// The type of data this tensor contains.
        pub element_type: ElementType,
        /// The tensor's dimensions.
        pub dimensions: Vec<u32>,
        /// The raw bytes used by this tensor, as little-endian values.
        ///
        /// Note: because string tensors are represented using a variable-length
        /// encoding where each string is prefixed by its length as a little-endian
        /// `u32`.
        pub buffer: Vec<u8>,
    }
    impl std::fmt::Debug for TensorResult {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("TensorResult")
                .field("element-type", &self.element_type)
                .field("dimensions", &self.dimensions)
                .field("buffer", &self.buffer)
                .finish()
        }
    }
    /// Metadata describing a single node in the Machine Learning pipeline.
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct Metadata(i32);
    impl Metadata {
        pub unsafe fn from_raw(raw: i32) -> Self {
            Self(raw)
        }

        pub fn into_raw(self) -> i32 {
            let ret = self.0;
            core::mem::forget(self);
            return ret;
        }

        pub fn as_raw(&self) -> i32 {
            self.0
        }
    }
    impl Drop for Metadata {
        fn drop(&mut self) {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_drop_metadata"]
                fn close(fd: i32);
            }
            unsafe {
                close(self.0);
            }
        }
    }
    /// Information about a node's argument.
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct ArgumentMetadata(i32);
    impl ArgumentMetadata {
        pub unsafe fn from_raw(raw: i32) -> Self {
            Self(raw)
        }

        pub fn into_raw(self) -> i32 {
            let ret = self.0;
            core::mem::forget(self);
            return ret;
        }

        pub fn as_raw(&self) -> i32 {
            self.0
        }
    }
    impl Drop for ArgumentMetadata {
        fn drop(&mut self) {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_drop_argument-metadata"]
                fn close(fd: i32);
            }
            unsafe {
                close(self.0);
            }
        }
    }
    /// Information about a tensor.
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct TensorMetadata(i32);
    impl TensorMetadata {
        pub unsafe fn from_raw(raw: i32) -> Self {
            Self(raw)
        }

        pub fn into_raw(self) -> i32 {
            let ret = self.0;
            core::mem::forget(self);
            return ret;
        }

        pub fn as_raw(&self) -> i32 {
            self.0
        }
    }
    impl Drop for TensorMetadata {
        fn drop(&mut self) {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_drop_tensor-metadata"]
                fn close(fd: i32);
            }
            unsafe {
                close(self.0);
            }
        }
    }
    /// Hints that can be used by the runtime when inspecting a tensor.
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct TensorHint(i32);
    impl TensorHint {
        pub unsafe fn from_raw(raw: i32) -> Self {
            Self(raw)
        }

        pub fn into_raw(self) -> i32 {
            let ret = self.0;
            core::mem::forget(self);
            return ret;
        }

        pub fn as_raw(&self) -> i32 {
            self.0
        }
    }
    impl Drop for TensorHint {
        fn drop(&mut self) {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_drop_tensor-hint"]
                fn close(fd: i32);
            }
            unsafe {
                close(self.0);
            }
        }
    }
    /// Hints that can be used by the runtime when inspecting an argument
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct ArgumentHint(i32);
    impl ArgumentHint {
        pub unsafe fn from_raw(raw: i32) -> Self {
            Self(raw)
        }

        pub fn into_raw(self) -> i32 {
            let ret = self.0;
            core::mem::forget(self);
            return ret;
        }

        pub fn as_raw(&self) -> i32 {
            self.0
        }
    }
    impl Drop for ArgumentHint {
        fn drop(&mut self) {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_drop_argument-hint"]
                fn close(fd: i32);
            }
            unsafe {
                close(self.0);
            }
        }
    }
    /// Contextual information used when determining the ML / Data Processing pipeline.
    /// This is defined by the runtime but available for logic within the container (rune)
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct GraphContext(i32);
    impl GraphContext {
        pub unsafe fn from_raw(raw: i32) -> Self {
            Self(raw)
        }

        pub fn into_raw(self) -> i32 {
            let ret = self.0;
            core::mem::forget(self);
            return ret;
        }

        pub fn as_raw(&self) -> i32 {
            self.0
        }
    }
    impl Drop for GraphContext {
        fn drop(&mut self) {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_drop_graph-context"]
                fn close(fd: i32);
            }
            unsafe {
                close(self.0);
            }
        }
    }
    /// Contextual information provided to the guest when evaluating a node.
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct KernelContext(i32);
    impl KernelContext {
        pub unsafe fn from_raw(raw: i32) -> Self {
            Self(raw)
        }

        pub fn into_raw(self) -> i32 {
            let ret = self.0;
            core::mem::forget(self);
            return ret;
        }

        pub fn as_raw(&self) -> i32 {
            self.0
        }
    }
    impl Drop for KernelContext {
        fn drop(&mut self) {
            #[link(wasm_import_module = "canonical_abi")]
            extern "C" {
                #[link_name = "resource_drop_kernel-context"]
                fn close(fd: i32);
            }
            unsafe {
                close(self.0);
            }
        }
    }
    impl Metadata {
        /// Create a new metadata object with the provided name and version number.
        ///
        /// The name should typically be one or two words that concisely describe
        /// the node and will be used as the human-friendly label shown to users
        /// when referring to it.
        pub fn new(name: &str, version: &str) -> Metadata {
            unsafe {
                let vec0 = name;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                let vec1 = version;
                let ptr1 = vec1.as_ptr() as i32;
                let len1 = vec1.len() as i32;
                #[link(wasm_import_module = "runtime-v1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "metadata::new")]
                    #[cfg_attr(not(target_arch = "wasm32"), link_name = "runtime-v1_metadata::new")]
                    fn wit_import(_: i32, _: i32, _: i32, _: i32) -> i32;
                }
                let ret = wit_import(ptr0, len0, ptr1, len1);
                Metadata(ret)
            }
        }
    }
    impl Metadata {
        /// A human-friendly description of the node.
        ///
        /// The text may include markdown.
        pub fn set_description(&self, description: &str) {
            unsafe {
                let vec0 = description;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                #[link(wasm_import_module = "runtime-v1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "metadata::set-description")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "runtime-v1_metadata::set-description"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, ptr0, len0);
            }
        }
    }
    impl Metadata {
        /// A repository containing this node's source code.
        pub fn set_repository(&self, url: &str) {
            unsafe {
                let vec0 = url;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                #[link(wasm_import_module = "runtime-v1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "metadata::set-repository")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "runtime-v1_metadata::set-repository"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, ptr0, len0);
            }
        }
    }
    impl Metadata {
        /// The node's home page.
        ///
        /// This will typically point to a `README` file or a page on the internet
        /// that users can go to when they want to find out more about the node.
        pub fn set_homepage(&self, url: &str) {
            unsafe {
                let vec0 = url;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                #[link(wasm_import_module = "runtime-v1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "metadata::set-homepage")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "runtime-v1_metadata::set-homepage"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, ptr0, len0);
            }
        }
    }
    impl Metadata {
        /// Associate this node with a particular tag.
        ///
        /// Tags are typically used to assist in search and filtering.
        pub fn add_tag(&self, tag: &str) {
            unsafe {
                let vec0 = tag;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                #[link(wasm_import_module = "runtime-v1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "metadata::add-tag")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "runtime-v1_metadata::add-tag"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, ptr0, len0);
            }
        }
    }
    impl Metadata {
        /// Arguments this node accepts.
        pub fn add_argument(&self, arg: &ArgumentMetadata) {
            unsafe {
                #[link(wasm_import_module = "runtime-v1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "metadata::add-argument")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "runtime-v1_metadata::add-argument"
                    )]
                    fn wit_import(_: i32, _: i32);
                }
                wit_import(self.0, arg.0);
            }
        }
    }
    impl Metadata {
        /// Information about this node's input tensors.
        pub fn add_input(&self, metadata: &TensorMetadata) {
            unsafe {
                #[link(wasm_import_module = "runtime-v1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "metadata::add-input")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "runtime-v1_metadata::add-input"
                    )]
                    fn wit_import(_: i32, _: i32);
                }
                wit_import(self.0, metadata.0);
            }
        }
    }
    impl Metadata {
        /// Information about this node's output tensors.
        pub fn add_output(&self, metadata: &TensorMetadata) {
            unsafe {
                #[link(wasm_import_module = "runtime-v1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "metadata::add-output")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "runtime-v1_metadata::add-output"
                    )]
                    fn wit_import(_: i32, _: i32);
                }
                wit_import(self.0, metadata.0);
            }
        }
    }
    impl ArgumentMetadata {
        /// Create a new named argument.
        pub fn new(name: &str) -> ArgumentMetadata {
            unsafe {
                let vec0 = name;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                #[link(wasm_import_module = "runtime-v1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "argument-metadata::new")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "runtime-v1_argument-metadata::new"
                    )]
                    fn wit_import(_: i32, _: i32) -> i32;
                }
                let ret = wit_import(ptr0, len0);
                ArgumentMetadata(ret)
            }
        }
    }
    impl ArgumentMetadata {
        /// A human-friendly description of the argument.
        ///
        /// The text may include markdown.
        pub fn set_description(&self, description: &str) {
            unsafe {
                let vec0 = description;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                #[link(wasm_import_module = "runtime-v1")]
                extern "C" {
                    #[cfg_attr(
                        target_arch = "wasm32",
                        link_name = "argument-metadata::set-description"
                    )]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "runtime-v1_argument-metadata::set-description"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, ptr0, len0);
            }
        }
    }
    impl ArgumentMetadata {
        /// A useful default value for this argument.
        pub fn set_default_value(&self, default_value: &str) {
            unsafe {
                let vec0 = default_value;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                #[link(wasm_import_module = "runtime-v1")]
                extern "C" {
                    #[cfg_attr(
                        target_arch = "wasm32",
                        link_name = "argument-metadata::set-default-value"
                    )]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "runtime-v1_argument-metadata::set-default-value"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, ptr0, len0);
            }
        }
    }
    impl ArgumentMetadata {
        pub fn add_hint(&self, hint: &ArgumentHint) {
            unsafe {
                #[link(wasm_import_module = "runtime-v1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "argument-metadata::add-hint")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "runtime-v1_argument-metadata::add-hint"
                    )]
                    fn wit_import(_: i32, _: i32);
                }
                wit_import(self.0, hint.0);
            }
        }
    }
    impl TensorMetadata {
        /// Create a new named tensor.
        pub fn new(name: &str) -> TensorMetadata {
            unsafe {
                let vec0 = name;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                #[link(wasm_import_module = "runtime-v1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "tensor-metadata::new")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "runtime-v1_tensor-metadata::new"
                    )]
                    fn wit_import(_: i32, _: i32) -> i32;
                }
                let ret = wit_import(ptr0, len0);
                TensorMetadata(ret)
            }
        }
    }
    impl TensorMetadata {
        /// A human-friendly description of the tensor.
        ///
        /// The text may include markdown.
        pub fn set_description(&self, description: &str) {
            unsafe {
                let vec0 = description;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                #[link(wasm_import_module = "runtime-v1")]
                extern "C" {
                    #[cfg_attr(
                        target_arch = "wasm32",
                        link_name = "tensor-metadata::set-description"
                    )]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "runtime-v1_tensor-metadata::set-description"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32);
                }
                wit_import(self.0, ptr0, len0);
            }
        }
    }
    impl TensorMetadata {
        /// Add a hint that provides the runtime with contextual information about
        /// this node.
        pub fn add_hint(&self, hint: &TensorHint) {
            unsafe {
                #[link(wasm_import_module = "runtime-v1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "tensor-metadata::add-hint")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "runtime-v1_tensor-metadata::add-hint"
                    )]
                    fn wit_import(_: i32, _: i32);
                }
                wit_import(self.0, hint.0);
            }
        }
    }
    /// Hint to the runtime that a tensor may be displayed as an image.
    pub fn interpret_as_image() -> TensorHint {
        unsafe {
            #[link(wasm_import_module = "runtime-v1")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "interpret-as-image")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "runtime-v1_interpret-as-image"
                )]
                fn wit_import() -> i32;
            }
            let ret = wit_import();
            TensorHint(ret)
        }
    }
    /// Hint to the runtime that a tensor may be interpreted as an audio clip.
    pub fn interpret_as_audio() -> TensorHint {
        unsafe {
            #[link(wasm_import_module = "runtime-v1")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "interpret-as-audio")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "runtime-v1_interpret-as-audio"
                )]
                fn wit_import() -> i32;
            }
            let ret = wit_import();
            TensorHint(ret)
        }
    }
    /// Hint that a tensor may have a particular shape and the element types it
    /// supports.
    ///
    /// Note: This hint will be removed in the future in favour of a more flexible
    /// mechanism.
    pub fn supported_shapes(
        supported_element_types: &[ElementType],
        dimensions: Dimensions<'_>,
    ) -> TensorHint {
        unsafe {
            let vec0 = supported_element_types;
            let len0 = vec0.len() as i32;
            let layout0 = core::alloc::Layout::from_size_align_unchecked(vec0.len() * 1, 1);
            let result0 = std::alloc::alloc(layout0);
            if result0.is_null() {
                std::alloc::handle_alloc_error(layout0);
            }
            for (i, e) in vec0.into_iter().enumerate() {
                let base = result0 as i32 + (i as i32) * 1;
                {
                    match e {
                        ElementType::U8 => {
                            *((base + 0) as *mut u8) = (0i32) as u8;
                        }
                        ElementType::I8 => {
                            *((base + 0) as *mut u8) = (1i32) as u8;
                        }
                        ElementType::U16 => {
                            *((base + 0) as *mut u8) = (2i32) as u8;
                        }
                        ElementType::I16 => {
                            *((base + 0) as *mut u8) = (3i32) as u8;
                        }
                        ElementType::U32 => {
                            *((base + 0) as *mut u8) = (4i32) as u8;
                        }
                        ElementType::I32 => {
                            *((base + 0) as *mut u8) = (5i32) as u8;
                        }
                        ElementType::F32 => {
                            *((base + 0) as *mut u8) = (6i32) as u8;
                        }
                        ElementType::U64 => {
                            *((base + 0) as *mut u8) = (7i32) as u8;
                        }
                        ElementType::I64 => {
                            *((base + 0) as *mut u8) = (8i32) as u8;
                        }
                        ElementType::F64 => {
                            *((base + 0) as *mut u8) = (9i32) as u8;
                        }
                        ElementType::Utf8 => {
                            *((base + 0) as *mut u8) = (10i32) as u8;
                        }
                    };
                }
            }
            let (result2_0, result2_1, result2_2) = match dimensions {
                Dimensions::Dynamic => (0i32, 0i32, 0i32),
                Dimensions::Fixed(e) => {
                    let vec1 = e;
                    let ptr1 = vec1.as_ptr() as i32;
                    let len1 = vec1.len() as i32;

                    (1i32, ptr1, len1)
                }
            };
            #[link(wasm_import_module = "runtime-v1")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "supported-shapes")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "runtime-v1_supported-shapes")]
                fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32) -> i32;
            }
            let ret = wit_import(result0 as i32, len0, result2_0, result2_1, result2_2);
            std::alloc::dealloc(result0, layout0);
            TensorHint(ret)
        }
    }
    /// Hint to the runtime that an argument may be interpreted as a number in `[min, max]`
    pub fn interpret_as_number_in_range(min: &str, max: &str) -> ArgumentHint {
        unsafe {
            let vec0 = min;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            let vec1 = max;
            let ptr1 = vec1.as_ptr() as i32;
            let len1 = vec1.len() as i32;
            #[link(wasm_import_module = "runtime-v1")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "interpret-as-number-in-range")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "runtime-v1_interpret-as-number-in-range"
                )]
                fn wit_import(_: i32, _: i32, _: i32, _: i32) -> i32;
            }
            let ret = wit_import(ptr0, len0, ptr1, len1);
            ArgumentHint(ret)
        }
    }
    /// Hint to the runtime that an argument may be interpreted as a string in a defined list
    pub fn interpret_as_string_in_enum(string_enum: &[&str]) -> ArgumentHint {
        unsafe {
            let vec1 = string_enum;
            let len1 = vec1.len() as i32;
            let layout1 = core::alloc::Layout::from_size_align_unchecked(vec1.len() * 8, 4);
            let result1 = std::alloc::alloc(layout1);
            if result1.is_null() {
                std::alloc::handle_alloc_error(layout1);
            }
            for (i, e) in vec1.into_iter().enumerate() {
                let base = result1 as i32 + (i as i32) * 8;
                {
                    let vec0 = e;
                    let ptr0 = vec0.as_ptr() as i32;
                    let len0 = vec0.len() as i32;
                    *((base + 4) as *mut i32) = len0;
                    *((base + 0) as *mut i32) = ptr0;
                }
            }
            #[link(wasm_import_module = "runtime-v1")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "interpret-as-string-in-enum")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "runtime-v1_interpret-as-string-in-enum"
                )]
                fn wit_import(_: i32, _: i32) -> i32;
            }
            let ret = wit_import(result1 as i32, len1);
            std::alloc::dealloc(result1, layout1);
            ArgumentHint(ret)
        }
    }
    /// Hint to the runtime that an argument may be interpreted as a non-negative number
    pub fn non_negative_number() -> ArgumentHint {
        unsafe {
            #[link(wasm_import_module = "runtime-v1")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "non-negative-number")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "runtime-v1_non-negative-number"
                )]
                fn wit_import() -> i32;
            }
            let ret = wit_import();
            ArgumentHint(ret)
        }
    }
    /// Tell the runtime that this argument may have a certain type.
    pub fn supported_argument_type(hint: ArgumentType) -> ArgumentHint {
        unsafe {
            #[link(wasm_import_module = "runtime-v1")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "supported-argument-type")]
                #[cfg_attr(
                    not(target_arch = "wasm32"),
                    link_name = "runtime-v1_supported-argument-type"
                )]
                fn wit_import(_: i32) -> i32;
            }
            let ret = wit_import(hint as i32);
            ArgumentHint(ret)
        }
    }
    /// Register a node type with the runtime.
    pub fn register_node(metadata: &Metadata) {
        unsafe {
            #[link(wasm_import_module = "runtime-v1")]
            extern "C" {
                #[cfg_attr(target_arch = "wasm32", link_name = "register-node")]
                #[cfg_attr(not(target_arch = "wasm32"), link_name = "runtime-v1_register-node")]
                fn wit_import(_: i32);
            }
            wit_import(metadata.0);
        }
    }
    impl GraphContext {
        /// Get the current graph context.
        ///
        /// Note: this can only be used from within the `graph()` function.
        pub fn current() -> Option<GraphContext> {
            unsafe {
                let ptr0 = RET_AREA.as_mut_ptr() as i32;
                #[link(wasm_import_module = "runtime-v1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "graph-context::current")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "runtime-v1_graph-context::current"
                    )]
                    fn wit_import(_: i32);
                }
                wit_import(ptr0);
                match *((ptr0 + 0) as *const i32) {
                    0 => None,
                    1 => Some(GraphContext(*((ptr0 + 8) as *const i32))),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl GraphContext {
        /// Name of the argument and returns the value of the argument
        /// Analogy: Getting an environment variable docker container
        pub fn get_argument(&self, name: &str) -> Option<String> {
            unsafe {
                let vec0 = name;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                let ptr1 = RET_AREA.as_mut_ptr() as i32;
                #[link(wasm_import_module = "runtime-v1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "graph-context::get-argument")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "runtime-v1_graph-context::get-argument"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32, _: i32);
                }
                wit_import(self.0, ptr0, len0, ptr1);
                match *((ptr1 + 0) as *const i32) {
                    0 => None,
                    1 => Some({
                        let len2 = *((ptr1 + 16) as *const i32) as usize;

                        String::from_utf8(Vec::from_raw_parts(
                            *((ptr1 + 8) as *const i32) as *mut _,
                            len2,
                            len2,
                        ))
                        .unwrap()
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl GraphContext {
        pub fn add_input_tensor(
            &self,
            name: &str,
            element_type: ElementType,
            dimensions: Dimensions<'_>,
        ) {
            unsafe {
                let vec0 = name;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                let (result2_0, result2_1, result2_2) = match dimensions {
                    Dimensions::Dynamic => (0i32, 0i32, 0i32),
                    Dimensions::Fixed(e) => {
                        let vec1 = e;
                        let ptr1 = vec1.as_ptr() as i32;
                        let len1 = vec1.len() as i32;

                        (1i32, ptr1, len1)
                    }
                };
                #[link(wasm_import_module = "runtime-v1")]
                extern "C" {
                    #[cfg_attr(
                        target_arch = "wasm32",
                        link_name = "graph-context::add-input-tensor"
                    )]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "runtime-v1_graph-context::add-input-tensor"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32);
                }
                wit_import(
                    self.0,
                    ptr0,
                    len0,
                    element_type as i32,
                    result2_0,
                    result2_1,
                    result2_2,
                );
            }
        }
    }
    impl GraphContext {
        pub fn add_output_tensor(
            &self,
            name: &str,
            element_type: ElementType,
            dimensions: Dimensions<'_>,
        ) {
            unsafe {
                let vec0 = name;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                let (result2_0, result2_1, result2_2) = match dimensions {
                    Dimensions::Dynamic => (0i32, 0i32, 0i32),
                    Dimensions::Fixed(e) => {
                        let vec1 = e;
                        let ptr1 = vec1.as_ptr() as i32;
                        let len1 = vec1.len() as i32;

                        (1i32, ptr1, len1)
                    }
                };
                #[link(wasm_import_module = "runtime-v1")]
                extern "C" {
                    #[cfg_attr(
                        target_arch = "wasm32",
                        link_name = "graph-context::add-output-tensor"
                    )]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "runtime-v1_graph-context::add-output-tensor"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32);
                }
                wit_import(
                    self.0,
                    ptr0,
                    len0,
                    element_type as i32,
                    result2_0,
                    result2_1,
                    result2_2,
                );
            }
        }
    }
    impl KernelContext {
        /// Get the current kernel context.
        ///
        /// Note: this can only be used from within the `kernel()` function.
        pub fn current() -> Option<KernelContext> {
            unsafe {
                let ptr0 = RET_AREA.as_mut_ptr() as i32;
                #[link(wasm_import_module = "runtime-v1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "kernel-context::current")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "runtime-v1_kernel-context::current"
                    )]
                    fn wit_import(_: i32);
                }
                wit_import(ptr0);
                match *((ptr0 + 0) as *const i32) {
                    0 => None,
                    1 => Some(KernelContext(*((ptr0 + 8) as *const i32))),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl KernelContext {
        /// Get a named argument.
        pub fn get_argument(&self, name: &str) -> Option<String> {
            unsafe {
                let vec0 = name;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                let ptr1 = RET_AREA.as_mut_ptr() as i32;
                #[link(wasm_import_module = "runtime-v1")]
                extern "C" {
                    #[cfg_attr(target_arch = "wasm32", link_name = "kernel-context::get-argument")]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "runtime-v1_kernel-context::get-argument"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32, _: i32);
                }
                wit_import(self.0, ptr0, len0, ptr1);
                match *((ptr1 + 0) as *const i32) {
                    0 => None,
                    1 => Some({
                        let len2 = *((ptr1 + 16) as *const i32) as usize;

                        String::from_utf8(Vec::from_raw_parts(
                            *((ptr1 + 8) as *const i32) as *mut _,
                            len2,
                            len2,
                        ))
                        .unwrap()
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl KernelContext {
        pub fn get_input_tensor(&self, name: &str) -> Option<TensorResult> {
            unsafe {
                let vec0 = name;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                let ptr1 = RET_AREA.as_mut_ptr() as i32;
                #[link(wasm_import_module = "runtime-v1")]
                extern "C" {
                    #[cfg_attr(
                        target_arch = "wasm32",
                        link_name = "kernel-context::get-input-tensor"
                    )]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "runtime-v1_kernel-context::get-input-tensor"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32, _: i32);
                }
                wit_import(self.0, ptr0, len0, ptr1);
                match *((ptr1 + 0) as *const i32) {
                    0 => None,
                    1 => Some({
                        let len2 = *((ptr1 + 24) as *const i32) as usize;
                        let len3 = *((ptr1 + 40) as *const i32) as usize;

                        TensorResult {
                            element_type: match *((ptr1 + 8) as *const i32) {
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
                                _ => panic!("invalid enum discriminant"),
                            },
                            dimensions: Vec::from_raw_parts(
                                *((ptr1 + 16) as *const i32) as *mut _,
                                len2,
                                len2,
                            ),
                            buffer: Vec::from_raw_parts(
                                *((ptr1 + 32) as *const i32) as *mut _,
                                len3,
                                len3,
                            ),
                        }
                    }),
                    _ => panic!("invalid enum discriminant"),
                }
            }
        }
    }
    impl KernelContext {
        pub fn set_output_tensor(&self, name: &str, tensor: TensorParam<'_>) {
            unsafe {
                let vec0 = name;
                let ptr0 = vec0.as_ptr() as i32;
                let len0 = vec0.len() as i32;
                let TensorParam {
                    element_type: element_type1,
                    dimensions: dimensions1,
                    buffer: buffer1,
                } = tensor;
                let vec2 = dimensions1;
                let ptr2 = vec2.as_ptr() as i32;
                let len2 = vec2.len() as i32;
                let vec3 = buffer1;
                let ptr3 = vec3.as_ptr() as i32;
                let len3 = vec3.len() as i32;
                #[link(wasm_import_module = "runtime-v1")]
                extern "C" {
                    #[cfg_attr(
                        target_arch = "wasm32",
                        link_name = "kernel-context::set-output-tensor"
                    )]
                    #[cfg_attr(
                        not(target_arch = "wasm32"),
                        link_name = "runtime-v1_kernel-context::set-output-tensor"
                    )]
                    fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32, _: i32);
                }
                wit_import(
                    self.0,
                    ptr0,
                    len0,
                    element_type1 as i32,
                    ptr2,
                    len2,
                    ptr3,
                    len3,
                );
            }
        }
    }
    static mut RET_AREA: [i64; 6] = [0; 6];
}
mod rune_v1 {
    pub enum GraphError {
        InvalidArgument(InvalidArgument),
        Other(String),
    }
    impl std::fmt::Debug for GraphError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                GraphError::InvalidArgument(e) => f
                    .debug_tuple("GraphError::InvalidArgument")
                    .field(e)
                    .finish(),
                GraphError::Other(e) => f.debug_tuple("GraphError::Other").field(e).finish(),
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
            f.debug_struct("InvalidArgument")
                .field("name", &self.name)
                .field("reason", &self.reason)
                .finish()
        }
    }
    /// The reason error is of type string that is thrown by the
    /// for example modulo(n: 0) => graph-error(invalid-argument(name: 'n', reason: invalid-value("N must be positive")))
    #[derive(Clone)]
    pub enum BadArgumentReason {
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
                BadArgumentReason::InvalidValue(e) => f
                    .debug_tuple("BadArgumentReason::InvalidValue")
                    .field(e)
                    .finish(),
                BadArgumentReason::Other(e) => {
                    f.debug_tuple("BadArgumentReason::Other").field(e).finish()
                }
            }
        }
    }
    pub enum KernelError {
        InvalidArgument(InvalidArgument),
        MissingInput(String),
        Other(String),
    }
    impl std::fmt::Debug for KernelError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                KernelError::InvalidArgument(e) => f
                    .debug_tuple("KernelError::InvalidArgument")
                    .field(e)
                    .finish(),
                KernelError::MissingInput(e) => {
                    f.debug_tuple("KernelError::MissingInput").field(e).finish()
                }
                KernelError::Other(e) => f.debug_tuple("KernelError::Other").field(e).finish(),
            }
        }
    }
    #[export_name = "start"]
    unsafe extern "C" fn __wit_bindgen_start() {
        <super::RuneV1 as RuneV1>::start();
    }
    #[export_name = "graph"]
    unsafe extern "C" fn __wit_bindgen_graph() -> i32 {
        let result0 = <super::RuneV1 as RuneV1>::graph();
        let (result8_0, result8_1, result8_2, result8_3, result8_4, result8_5, result8_6) =
            match result0 {
                Ok(()) => (0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32),
                Err(e) => {
                    let (result7_0, result7_1, result7_2, result7_3, result7_4, result7_5) = match e
                    {
                        GraphError::InvalidArgument(e) => {
                            let InvalidArgument {
                                name: name1,
                                reason: reason1,
                            } = e;
                            let vec2 = (name1.into_bytes()).into_boxed_slice();
                            let ptr2 = vec2.as_ptr() as i32;
                            let len2 = vec2.len() as i32;
                            core::mem::forget(vec2);
                            let (result5_0, result5_1, result5_2) = match reason1 {
                                BadArgumentReason::NotFound => (0i32, 0i32, 0i32),
                                BadArgumentReason::InvalidValue(e) => {
                                    let vec3 = (e.into_bytes()).into_boxed_slice();
                                    let ptr3 = vec3.as_ptr() as i32;
                                    let len3 = vec3.len() as i32;
                                    core::mem::forget(vec3);

                                    (1i32, ptr3, len3)
                                }
                                BadArgumentReason::Other(e) => {
                                    let vec4 = (e.into_bytes()).into_boxed_slice();
                                    let ptr4 = vec4.as_ptr() as i32;
                                    let len4 = vec4.len() as i32;
                                    core::mem::forget(vec4);

                                    (2i32, ptr4, len4)
                                }
                            };

                            (0i32, ptr2, len2, result5_0, result5_1, result5_2)
                        }
                        GraphError::Other(e) => {
                            let vec6 = (e.into_bytes()).into_boxed_slice();
                            let ptr6 = vec6.as_ptr() as i32;
                            let len6 = vec6.len() as i32;
                            core::mem::forget(vec6);

                            (1i32, ptr6, len6, 0i32, 0i32, 0i32)
                        }
                    };

                    (
                        1i32, result7_0, result7_1, result7_2, result7_3, result7_4, result7_5,
                    )
                }
            };
        let ptr9 = RET_AREA.as_mut_ptr() as i32;
        *((ptr9 + 48) as *mut i32) = result8_6;
        *((ptr9 + 40) as *mut i32) = result8_5;
        *((ptr9 + 32) as *mut i32) = result8_4;
        *((ptr9 + 24) as *mut i32) = result8_3;
        *((ptr9 + 16) as *mut i32) = result8_2;
        *((ptr9 + 8) as *mut i32) = result8_1;
        *((ptr9 + 0) as *mut i32) = result8_0;
        ptr9
    }
    #[export_name = "kernel"]
    unsafe extern "C" fn __wit_bindgen_kernel() -> i32 {
        let result0 = <super::RuneV1 as RuneV1>::kernel();
        let (result9_0, result9_1, result9_2, result9_3, result9_4, result9_5, result9_6) =
            match result0 {
                Ok(()) => (0i32, 0i32, 0i32, 0i32, 0i32, 0i32, 0i32),
                Err(e) => {
                    let (result8_0, result8_1, result8_2, result8_3, result8_4, result8_5) = match e
                    {
                        KernelError::InvalidArgument(e) => {
                            let InvalidArgument {
                                name: name1,
                                reason: reason1,
                            } = e;
                            let vec2 = (name1.into_bytes()).into_boxed_slice();
                            let ptr2 = vec2.as_ptr() as i32;
                            let len2 = vec2.len() as i32;
                            core::mem::forget(vec2);
                            let (result5_0, result5_1, result5_2) = match reason1 {
                                BadArgumentReason::NotFound => (0i32, 0i32, 0i32),
                                BadArgumentReason::InvalidValue(e) => {
                                    let vec3 = (e.into_bytes()).into_boxed_slice();
                                    let ptr3 = vec3.as_ptr() as i32;
                                    let len3 = vec3.len() as i32;
                                    core::mem::forget(vec3);

                                    (1i32, ptr3, len3)
                                }
                                BadArgumentReason::Other(e) => {
                                    let vec4 = (e.into_bytes()).into_boxed_slice();
                                    let ptr4 = vec4.as_ptr() as i32;
                                    let len4 = vec4.len() as i32;
                                    core::mem::forget(vec4);

                                    (2i32, ptr4, len4)
                                }
                            };

                            (0i32, ptr2, len2, result5_0, result5_1, result5_2)
                        }
                        KernelError::MissingInput(e) => {
                            let vec6 = (e.into_bytes()).into_boxed_slice();
                            let ptr6 = vec6.as_ptr() as i32;
                            let len6 = vec6.len() as i32;
                            core::mem::forget(vec6);

                            (1i32, ptr6, len6, 0i32, 0i32, 0i32)
                        }
                        KernelError::Other(e) => {
                            let vec7 = (e.into_bytes()).into_boxed_slice();
                            let ptr7 = vec7.as_ptr() as i32;
                            let len7 = vec7.len() as i32;
                            core::mem::forget(vec7);

                            (2i32, ptr7, len7, 0i32, 0i32, 0i32)
                        }
                    };

                    (
                        1i32, result8_0, result8_1, result8_2, result8_3, result8_4, result8_5,
                    )
                }
            };
        let ptr10 = RET_AREA.as_mut_ptr() as i32;
        *((ptr10 + 48) as *mut i32) = result9_6;
        *((ptr10 + 40) as *mut i32) = result9_5;
        *((ptr10 + 32) as *mut i32) = result9_4;
        *((ptr10 + 24) as *mut i32) = result9_3;
        *((ptr10 + 16) as *mut i32) = result9_2;
        *((ptr10 + 8) as *mut i32) = result9_1;
        *((ptr10 + 0) as *mut i32) = result9_0;
        ptr10
    }
    pub trait RuneV1 {
        /// ! Functions and types exposed by the Rune.
        /// A function called when the module is first loaded.
        fn start();
        /// A function that is called by the compiler/Forge while constructing the ML
        /// pipeline to find out this node's inputs and outputs.
        fn graph() -> Result<(), GraphError>;
        /// The function called when doing inference.
        fn kernel() -> Result<(), KernelError>;
    }
    static mut RET_AREA: [i64; 7] = [0; 7];
}
