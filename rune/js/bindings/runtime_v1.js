import { data_view, UTF8_DECODER, utf8_encode, UTF8_ENCODED_LEN, Slab } from './intrinsics.js';
export const ElementType = Object.freeze({
  0: "U8",
  "U8": 0,
  1: "I8",
  "I8": 1,
  2: "U16",
  "U16": 2,
  3: "I16",
  "I16": 3,
  4: "U32",
  "U32": 4,
  5: "I32",
  "I32": 5,
  6: "F32",
  "F32": 6,
  7: "U64",
  "U64": 7,
  8: "I64",
  "I64": 8,
  9: "F64",
  "F64": 9,
  10: "Utf8",
  "Utf8": 10,
});
export const ArgumentType = Object.freeze({
  0: "UnsignedInteger",
  "UnsignedInteger": 0,
  1: "Integer",
  "Integer": 1,
  2: "Float",
  "Float": 2,
  3: "String",
  "String": 3,
  4: "LongString",
  "LongString": 4,
});
export function addRuntimeV1ToImports(imports, obj, get_export) {
  if (!("runtime-v1" in imports)) imports["runtime-v1"] = {};
  imports["runtime-v1"]["metadata::new"] = function(arg0, arg1, arg2, arg3) {
    const memory = get_export("memory");
    const ptr0 = arg0;
    const len0 = arg1;
    const ptr1 = arg2;
    const len1 = arg3;
    const ret = obj.metadataNew(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)), UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr1, len1)));
    return resources0.insert(ret);
  };
  imports["runtime-v1"]["argument-metadata::new"] = function(arg0, arg1) {
    const memory = get_export("memory");
    const ptr0 = arg0;
    const len0 = arg1;
    const ret = obj.argumentMetadataNew(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)));
    return resources1.insert(ret);
  };
  imports["runtime-v1"]["tensor-metadata::new"] = function(arg0, arg1) {
    const memory = get_export("memory");
    const ptr0 = arg0;
    const len0 = arg1;
    const ret = obj.tensorMetadataNew(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)));
    return resources2.insert(ret);
  };
  imports["runtime-v1"]["interpret-as-image"] = function() {
    const ret = obj.interpretAsImage();
    return resources3.insert(ret);
  };
  imports["runtime-v1"]["interpret-as-audio"] = function() {
    const ret = obj.interpretAsAudio();
    return resources3.insert(ret);
  };
  imports["runtime-v1"]["supported-shapes"] = function(arg0, arg1, arg2, arg3, arg4) {
    const memory = get_export("memory");
    const len1 = arg1;
    const base1 = arg0;
    const result1 = [];
    for (let i = 0; i < len1; i++) {
      const base = base1 + i * 1;
      const tag0 = data_view(memory).getUint8(base + 0, true);
      if (!(tag0 in ElementType))
      throw new RangeError("invalid discriminant specified for ElementType");
      result1.push(tag0);
    }
    let variant3;
    switch (arg2) {
      case 0: {
        variant3 = {
          tag: "dynamic",
        };
        break;
      }
      case 1: {
        const ptr2 = arg3;
        const len2 = arg4;
        variant3 = {
          tag: "fixed",
          val: new Uint32Array(memory.buffer.slice(ptr2, ptr2 + len2 * 4)),
        };
        break;
      }
      default:
      throw new RangeError("invalid variant discriminant for Dimensions");
    }
    const ret = obj.supportedShapes(result1, variant3);
    return resources3.insert(ret);
  };
  imports["runtime-v1"]["interpret-as-number-in-range"] = function(arg0, arg1, arg2, arg3) {
    const memory = get_export("memory");
    const ptr0 = arg0;
    const len0 = arg1;
    const ptr1 = arg2;
    const len1 = arg3;
    const ret = obj.interpretAsNumberInRange(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)), UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr1, len1)));
    return resources4.insert(ret);
  };
  imports["runtime-v1"]["interpret-as-string-in-enum"] = function(arg0, arg1) {
    const memory = get_export("memory");
    const len1 = arg1;
    const base1 = arg0;
    const result1 = [];
    for (let i = 0; i < len1; i++) {
      const base = base1 + i * 8;
      const ptr0 = data_view(memory).getInt32(base + 0, true);
      const len0 = data_view(memory).getInt32(base + 4, true);
      result1.push(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)));
    }
    const ret = obj.interpretAsStringInEnum(result1);
    return resources4.insert(ret);
  };
  imports["runtime-v1"]["non-negative-number"] = function() {
    const ret = obj.nonNegativeNumber();
    return resources4.insert(ret);
  };
  imports["runtime-v1"]["supported-argument-type"] = function(arg0) {
    const tag0 = arg0;
    if (!(tag0 in ArgumentType))
    throw new RangeError("invalid discriminant specified for ArgumentType");
    const ret = obj.supportedArgumentType(tag0);
    return resources4.insert(ret);
  };
  imports["runtime-v1"]["register-node"] = function(arg0) {
    obj.registerNode(resources0.get(arg0));
  };
  imports["runtime-v1"]["graph-context::current"] = function(arg0) {
    const memory = get_export("memory");
    const ret = obj.graphContextCurrent();
    const variant0 = ret;
    let variant0_0;
    let variant0_1;
    switch (variant0) {
      case null: {
        variant0_0 = 0;
        variant0_1 = 0;
        break;
      }
      default: {
        const e = variant0;
        variant0_0 = 1;
        variant0_1 = resources5.insert(e);
        break;
      }
    }
    data_view(memory).setInt32(arg0 + 8, variant0_1, true);
    data_view(memory).setInt32(arg0 + 0, variant0_0, true);
  };
  imports["runtime-v1"]["kernel-context::current"] = function(arg0) {
    const memory = get_export("memory");
    const ret = obj.kernelContextCurrent();
    const variant0 = ret;
    let variant0_0;
    let variant0_1;
    switch (variant0) {
      case null: {
        variant0_0 = 0;
        variant0_1 = 0;
        break;
      }
      default: {
        const e = variant0;
        variant0_0 = 1;
        variant0_1 = resources6.insert(e);
        break;
      }
    }
    data_view(memory).setInt32(arg0 + 8, variant0_1, true);
    data_view(memory).setInt32(arg0 + 0, variant0_0, true);
  };
  imports["runtime-v1"]["metadata::set-description"] = function(arg0, arg1, arg2) {
    const memory = get_export("memory");
    const ptr0 = arg1;
    const len0 = arg2;
    resources0.get(arg0).setDescription(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)));
  };
  imports["runtime-v1"]["metadata::set-repository"] = function(arg0, arg1, arg2) {
    const memory = get_export("memory");
    const ptr0 = arg1;
    const len0 = arg2;
    resources0.get(arg0).setRepository(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)));
  };
  imports["runtime-v1"]["metadata::set-homepage"] = function(arg0, arg1, arg2) {
    const memory = get_export("memory");
    const ptr0 = arg1;
    const len0 = arg2;
    resources0.get(arg0).setHomepage(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)));
  };
  imports["runtime-v1"]["metadata::add-tag"] = function(arg0, arg1, arg2) {
    const memory = get_export("memory");
    const ptr0 = arg1;
    const len0 = arg2;
    resources0.get(arg0).addTag(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)));
  };
  imports["runtime-v1"]["metadata::add-argument"] = function(arg0, arg1) {
    resources0.get(arg0).addArgument(resources1.get(arg1));
  };
  imports["runtime-v1"]["metadata::add-input"] = function(arg0, arg1) {
    resources0.get(arg0).addInput(resources2.get(arg1));
  };
  imports["runtime-v1"]["metadata::add-output"] = function(arg0, arg1) {
    resources0.get(arg0).addOutput(resources2.get(arg1));
  };
  imports["runtime-v1"]["argument-metadata::set-description"] = function(arg0, arg1, arg2) {
    const memory = get_export("memory");
    const ptr0 = arg1;
    const len0 = arg2;
    resources1.get(arg0).setDescription(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)));
  };
  imports["runtime-v1"]["argument-metadata::set-default-value"] = function(arg0, arg1, arg2) {
    const memory = get_export("memory");
    const ptr0 = arg1;
    const len0 = arg2;
    resources1.get(arg0).setDefaultValue(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)));
  };
  imports["runtime-v1"]["argument-metadata::add-hint"] = function(arg0, arg1) {
    resources1.get(arg0).addHint(resources4.get(arg1));
  };
  imports["runtime-v1"]["tensor-metadata::set-description"] = function(arg0, arg1, arg2) {
    const memory = get_export("memory");
    const ptr0 = arg1;
    const len0 = arg2;
    resources2.get(arg0).setDescription(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)));
  };
  imports["runtime-v1"]["tensor-metadata::add-hint"] = function(arg0, arg1) {
    resources2.get(arg0).addHint(resources3.get(arg1));
  };
  imports["runtime-v1"]["graph-context::get-argument"] = function(arg0, arg1, arg2, arg3) {
    const memory = get_export("memory");
    const realloc = get_export("canonical_abi_realloc");
    const ptr0 = arg1;
    const len0 = arg2;
    const ret = resources5.get(arg0).getArgument(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)));
    const variant2 = ret;
    let variant2_0;
    let variant2_1;
    let variant2_2;
    switch (variant2) {
      case null: {
        variant2_0 = 0;
        variant2_1 = 0;
        variant2_2 = 0;
        break;
      }
      default: {
        const e = variant2;
        const ptr1 = utf8_encode(e, realloc, memory);
        const len1 = UTF8_ENCODED_LEN;
        variant2_0 = 1;
        variant2_1 = ptr1;
        variant2_2 = len1;
        break;
      }
    }
    data_view(memory).setInt32(arg3 + 16, variant2_2, true);
    data_view(memory).setInt32(arg3 + 8, variant2_1, true);
    data_view(memory).setInt32(arg3 + 0, variant2_0, true);
  };
  imports["runtime-v1"]["graph-context::add-input-tensor"] = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    const memory = get_export("memory");
    const ptr0 = arg1;
    const len0 = arg2;
    const tag1 = arg3;
    if (!(tag1 in ElementType))
    throw new RangeError("invalid discriminant specified for ElementType");
    let variant3;
    switch (arg4) {
      case 0: {
        variant3 = {
          tag: "dynamic",
        };
        break;
      }
      case 1: {
        const ptr2 = arg5;
        const len2 = arg6;
        variant3 = {
          tag: "fixed",
          val: new Uint32Array(memory.buffer.slice(ptr2, ptr2 + len2 * 4)),
        };
        break;
      }
      default:
      throw new RangeError("invalid variant discriminant for Dimensions");
    }
    resources5.get(arg0).addInputTensor(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)), tag1, variant3);
  };
  imports["runtime-v1"]["graph-context::add-output-tensor"] = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    const memory = get_export("memory");
    const ptr0 = arg1;
    const len0 = arg2;
    const tag1 = arg3;
    if (!(tag1 in ElementType))
    throw new RangeError("invalid discriminant specified for ElementType");
    let variant3;
    switch (arg4) {
      case 0: {
        variant3 = {
          tag: "dynamic",
        };
        break;
      }
      case 1: {
        const ptr2 = arg5;
        const len2 = arg6;
        variant3 = {
          tag: "fixed",
          val: new Uint32Array(memory.buffer.slice(ptr2, ptr2 + len2 * 4)),
        };
        break;
      }
      default:
      throw new RangeError("invalid variant discriminant for Dimensions");
    }
    resources5.get(arg0).addOutputTensor(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)), tag1, variant3);
  };
  imports["runtime-v1"]["kernel-context::get-argument"] = function(arg0, arg1, arg2, arg3) {
    const memory = get_export("memory");
    const realloc = get_export("canonical_abi_realloc");
    const ptr0 = arg1;
    const len0 = arg2;
    const ret = resources6.get(arg0).getArgument(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)));
    const variant2 = ret;
    let variant2_0;
    let variant2_1;
    let variant2_2;
    switch (variant2) {
      case null: {
        variant2_0 = 0;
        variant2_1 = 0;
        variant2_2 = 0;
        break;
      }
      default: {
        const e = variant2;
        const ptr1 = utf8_encode(e, realloc, memory);
        const len1 = UTF8_ENCODED_LEN;
        variant2_0 = 1;
        variant2_1 = ptr1;
        variant2_2 = len1;
        break;
      }
    }
    data_view(memory).setInt32(arg3 + 16, variant2_2, true);
    data_view(memory).setInt32(arg3 + 8, variant2_1, true);
    data_view(memory).setInt32(arg3 + 0, variant2_0, true);
  };
  imports["runtime-v1"]["kernel-context::get-input-tensor"] = function(arg0, arg1, arg2, arg3) {
    const memory = get_export("memory");
    const realloc = get_export("canonical_abi_realloc");
    const ptr0 = arg1;
    const len0 = arg2;
    const ret = resources6.get(arg0).getInputTensor(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)));
    const variant5 = ret;
    let variant5_0;
    let variant5_1;
    let variant5_2;
    let variant5_3;
    let variant5_4;
    let variant5_5;
    switch (variant5) {
      case null: {
        variant5_0 = 0;
        variant5_1 = 0;
        variant5_2 = 0;
        variant5_3 = 0;
        variant5_4 = 0;
        variant5_5 = 0;
        break;
      }
      default: {
        const e = variant5;
        const {elementType: v1_0, dimensions: v1_1, buffer: v1_2 } = e;
        const variant2 = v1_0;
        if (!(variant2 in ElementType))
        throw new RangeError("invalid variant specified for ElementType");
        const val3 = v1_1;
        const len3 = val3.length;
        const ptr3 = realloc(0, 0, 4, len3 * 4);
        (new Uint8Array(memory.buffer, ptr3, len3 * 4)).set(new Uint8Array(val3.buffer));
        const val4 = v1_2;
        const len4 = val4.length;
        const ptr4 = realloc(0, 0, 1, len4 * 1);
        (new Uint8Array(memory.buffer, ptr4, len4 * 1)).set(new Uint8Array(val4.buffer));
        variant5_0 = 1;
        variant5_1 = Number.isInteger(variant2) ? variant2 : ElementType[variant2];
        variant5_2 = ptr3;
        variant5_3 = len3;
        variant5_4 = ptr4;
        variant5_5 = len4;
        break;
      }
    }
    data_view(memory).setInt32(arg3 + 40, variant5_5, true);
    data_view(memory).setInt32(arg3 + 32, variant5_4, true);
    data_view(memory).setInt32(arg3 + 24, variant5_3, true);
    data_view(memory).setInt32(arg3 + 16, variant5_2, true);
    data_view(memory).setInt32(arg3 + 8, variant5_1, true);
    data_view(memory).setInt32(arg3 + 0, variant5_0, true);
  };
  imports["runtime-v1"]["kernel-context::set-output-tensor"] = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
    const memory = get_export("memory");
    const ptr0 = arg1;
    const len0 = arg2;
    const tag1 = arg3;
    if (!(tag1 in ElementType))
    throw new RangeError("invalid discriminant specified for ElementType");
    const ptr2 = arg4;
    const len2 = arg5;
    const ptr3 = arg6;
    const len3 = arg7;
    resources6.get(arg0).setOutputTensor(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)), {
      elementType: tag1,
      dimensions: new Uint32Array(memory.buffer.slice(ptr2, ptr2 + len2 * 4)),
      buffer: new Uint8Array(memory.buffer.slice(ptr3, ptr3 + len3 * 1)),
    });
  };
  if (!("canonical_abi" in imports)) imports["canonical_abi"] = {};
  
  const resources0 = new Slab();
  imports.canonical_abi["resource_drop_metadata"] = (i) => {
    const val = resources0.remove(i);
    if (obj.dropMetadata)
    obj.dropMetadata(val);
  };
  
  const resources1 = new Slab();
  imports.canonical_abi["resource_drop_argument-metadata"] = (i) => {
    const val = resources1.remove(i);
    if (obj.dropArgumentMetadata)
    obj.dropArgumentMetadata(val);
  };
  
  const resources2 = new Slab();
  imports.canonical_abi["resource_drop_tensor-metadata"] = (i) => {
    const val = resources2.remove(i);
    if (obj.dropTensorMetadata)
    obj.dropTensorMetadata(val);
  };
  
  const resources3 = new Slab();
  imports.canonical_abi["resource_drop_tensor-hint"] = (i) => {
    const val = resources3.remove(i);
    if (obj.dropTensorHint)
    obj.dropTensorHint(val);
  };
  
  const resources4 = new Slab();
  imports.canonical_abi["resource_drop_argument-hint"] = (i) => {
    const val = resources4.remove(i);
    if (obj.dropArgumentHint)
    obj.dropArgumentHint(val);
  };
  
  const resources5 = new Slab();
  imports.canonical_abi["resource_drop_graph-context"] = (i) => {
    const val = resources5.remove(i);
    if (obj.dropGraphContext)
    obj.dropGraphContext(val);
  };
  
  const resources6 = new Slab();
  imports.canonical_abi["resource_drop_kernel-context"] = (i) => {
    const val = resources6.remove(i);
    if (obj.dropKernelContext)
    obj.dropKernelContext(val);
  };
}