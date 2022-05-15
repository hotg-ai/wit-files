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
export const LogLevel = Object.freeze({
  0: "Trace",
  "Trace": 0,
  1: "Debug",
  "Debug": 1,
  2: "Info",
  "Info": 2,
  3: "Warn",
  "Warn": 3,
  4: "Error",
  "Error": 4,
  5: "Fatal",
  "Fatal": 5,
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
  imports["runtime-v1"]["graph-context::for-node"] = function(arg0, arg1, arg2) {
    const memory = get_export("memory");
    const ptr0 = arg0;
    const len0 = arg1;
    const ret = obj.graphContextForNode(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)));
    const variant1 = ret;
    let variant1_0;
    let variant1_1;
    switch (variant1) {
      case null: {
        variant1_0 = 0;
        variant1_1 = 0;
        break;
      }
      default: {
        const e = variant1;
        variant1_0 = 1;
        variant1_1 = resources5.insert(e);
        break;
      }
    }
    data_view(memory).setInt32(arg2 + 8, variant1_1, true);
    data_view(memory).setInt32(arg2 + 0, variant1_0, true);
  };
  imports["runtime-v1"]["kernel-context::for-node"] = function(arg0, arg1, arg2) {
    const memory = get_export("memory");
    const ptr0 = arg0;
    const len0 = arg1;
    const ret = obj.kernelContextForNode(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)));
    const variant1 = ret;
    let variant1_0;
    let variant1_1;
    switch (variant1) {
      case null: {
        variant1_0 = 0;
        variant1_1 = 0;
        break;
      }
      default: {
        const e = variant1;
        variant1_0 = 1;
        variant1_1 = resources6.insert(e);
        break;
      }
    }
    data_view(memory).setInt32(arg2 + 8, variant1_1, true);
    data_view(memory).setInt32(arg2 + 0, variant1_0, true);
  };
  imports["runtime-v1"]["is-enabled"] = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12) {
    const memory = get_export("memory");
    const ptr0 = arg0;
    const len0 = arg1;
    const ptr1 = arg2;
    const len1 = arg3;
    const tag2 = arg4;
    if (!(tag2 in LogLevel))
    throw new RangeError("invalid discriminant specified for LogLevel");
    let variant4;
    switch (arg5) {
      case 0: {
        variant4 = null;
        break;
      }
      case 1: {
        const ptr3 = arg6;
        const len3 = arg7;
        variant4 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr3, len3));
        break;
      }
      default:
      throw new RangeError("invalid variant discriminant for option");
    }
    let variant5;
    switch (arg8) {
      case 0: {
        variant5 = null;
        break;
      }
      case 1: {
        variant5 = arg9 >>> 0;
        break;
      }
      default:
      throw new RangeError("invalid variant discriminant for option");
    }
    let variant7;
    switch (arg10) {
      case 0: {
        variant7 = null;
        break;
      }
      case 1: {
        const ptr6 = arg11;
        const len6 = arg12;
        variant7 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr6, len6));
        break;
      }
      default:
      throw new RangeError("invalid variant discriminant for option");
    }
    const ret = obj.isEnabled({
      name: UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)),
      target: UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr1, len1)),
      level: tag2,
      file: variant4,
      line: variant5,
      module: variant7,
    });
    const variant8 = ret;
    let variant8_0;
    switch (variant8) {
      case false: {
        variant8_0 = 0;
        break;
      }
      case true: {
        variant8_0 = 1;
        break;
      }
      default:
      throw new RangeError("invalid variant specified for bool");
    }
    return variant8_0;
  };
  imports["runtime-v1"]["log"] = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15, arg16) {
    const memory = get_export("memory");
    const ptr0 = arg0;
    const len0 = arg1;
    const ptr1 = arg2;
    const len1 = arg3;
    const tag2 = arg4;
    if (!(tag2 in LogLevel))
    throw new RangeError("invalid discriminant specified for LogLevel");
    let variant4;
    switch (arg5) {
      case 0: {
        variant4 = null;
        break;
      }
      case 1: {
        const ptr3 = arg6;
        const len3 = arg7;
        variant4 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr3, len3));
        break;
      }
      default:
      throw new RangeError("invalid variant discriminant for option");
    }
    let variant5;
    switch (arg8) {
      case 0: {
        variant5 = null;
        break;
      }
      case 1: {
        variant5 = arg9 >>> 0;
        break;
      }
      default:
      throw new RangeError("invalid variant discriminant for option");
    }
    let variant7;
    switch (arg10) {
      case 0: {
        variant7 = null;
        break;
      }
      case 1: {
        const ptr6 = arg11;
        const len6 = arg12;
        variant7 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr6, len6));
        break;
      }
      default:
      throw new RangeError("invalid variant discriminant for option");
    }
    const ptr8 = arg13;
    const len8 = arg14;
    const len13 = arg16;
    const base13 = arg15;
    const result13 = [];
    for (let i = 0; i < len13; i++) {
      const base = base13 + i * 24;
      const ptr9 = data_view(memory).getInt32(base + 0, true);
      const len9 = data_view(memory).getInt32(base + 4, true);
      let variant12;
      switch (data_view(memory).getUint8(base + 8, true)) {
        case 0: {
          variant12 = {
            tag: "null",
          };
          break;
        }
        case 1: {
          let variant10;
          switch (data_view(memory).getUint8(base + 16, true)) {
            case 0: {
              variant10 = false;
              break;
            }
            case 1: {
              variant10 = true;
              break;
            }
            default:
            throw new RangeError("invalid variant discriminant for bool");
          }
          variant12 = {
            tag: "boolean",
            val: variant10,
          };
          break;
        }
        case 2: {
          variant12 = {
            tag: "integer",
            val: data_view(memory).getBigInt64(base + 16, true),
          };
          break;
        }
        case 3: {
          variant12 = {
            tag: "float",
            val: data_view(memory).getFloat64(base + 16, true),
          };
          break;
        }
        case 4: {
          const ptr11 = data_view(memory).getInt32(base + 16, true);
          const len11 = data_view(memory).getInt32(base + 20, true);
          variant12 = {
            tag: "string",
            val: UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr11, len11)),
          };
          break;
        }
        default:
        throw new RangeError("invalid variant discriminant for LogValue");
      }
      result13.push([UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr9, len9)), variant12]);
    }
    obj.log({
      name: UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)),
      target: UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr1, len1)),
      level: tag2,
      file: variant4,
      line: variant5,
      module: variant7,
    }, UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr8, len8)), result13);
  };
  imports["runtime-v1"]["model::load"] = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    const memory = get_export("memory");
    const realloc = get_export("canonical_abi_realloc");
    const ptr0 = arg0;
    const len0 = arg1;
    const ptr1 = arg2;
    const len1 = arg3;
    const len4 = arg5;
    const base4 = arg4;
    const result4 = [];
    for (let i = 0; i < len4; i++) {
      const base = base4 + i * 16;
      const ptr2 = data_view(memory).getInt32(base + 0, true);
      const len2 = data_view(memory).getInt32(base + 4, true);
      const ptr3 = data_view(memory).getInt32(base + 8, true);
      const len3 = data_view(memory).getInt32(base + 12, true);
      result4.push([UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr2, len2)), UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr3, len3))]);
    }
    const ret = obj.modelLoad(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)), new Uint8Array(memory.buffer.slice(ptr1, ptr1 + len1 * 1)), result4);
    const variant7 = ret;
    let variant7_0;
    let variant7_1;
    let variant7_2;
    let variant7_3;
    switch (variant7.tag) {
      case "ok": {
        const e = variant7.val;
        variant7_0 = 0;
        variant7_1 = resources7.insert(e);
        variant7_2 = 0;
        variant7_3 = 0;
        break;
      }
      case "err": {
        const e = variant7.val;
        const variant6 = e;
        let variant6_0;
        let variant6_1;
        let variant6_2;
        switch (variant6.tag) {
          case "other": {
            const e = variant6.val;
            const ptr5 = utf8_encode(e, realloc, memory);
            const len5 = UTF8_ENCODED_LEN;
            variant6_0 = 0;
            variant6_1 = ptr5;
            variant6_2 = len5;
            break;
          }
          default:
          throw new RangeError("invalid variant specified for ModelLoadError");
        }
        variant7_0 = 1;
        variant7_1 = variant6_0;
        variant7_2 = variant6_1;
        variant7_3 = variant6_2;
        break;
      }
      default:
      throw new RangeError("invalid variant specified for expected");
    }
    data_view(memory).setInt32(arg6 + 24, variant7_3, true);
    data_view(memory).setInt32(arg6 + 16, variant7_2, true);
    data_view(memory).setInt32(arg6 + 8, variant7_1, true);
    data_view(memory).setInt32(arg6 + 0, variant7_0, true);
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
        (new Uint8Array(memory.buffer, ptr3, len3 * 4)).set(new Uint8Array(val3.buffer, val3.byteOffset, len3 * 4));
        const val4 = v1_2;
        const len4 = val4.length;
        const ptr4 = realloc(0, 0, 1, len4 * 1);
        (new Uint8Array(memory.buffer, ptr4, len4 * 1)).set(new Uint8Array(val4.buffer, val4.byteOffset, len4 * 1));
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
  imports["runtime-v1"]["kernel-context::get-global-input"] = function(arg0, arg1, arg2, arg3) {
    const memory = get_export("memory");
    const realloc = get_export("canonical_abi_realloc");
    const ptr0 = arg1;
    const len0 = arg2;
    const ret = resources6.get(arg0).getGlobalInput(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)));
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
        (new Uint8Array(memory.buffer, ptr3, len3 * 4)).set(new Uint8Array(val3.buffer, val3.byteOffset, len3 * 4));
        const val4 = v1_2;
        const len4 = val4.length;
        const ptr4 = realloc(0, 0, 1, len4 * 1);
        (new Uint8Array(memory.buffer, ptr4, len4 * 1)).set(new Uint8Array(val4.buffer, val4.byteOffset, len4 * 1));
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
  imports["runtime-v1"]["kernel-context::set-global-output"] = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
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
    resources6.get(arg0).setGlobalOutput(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)), {
      elementType: tag1,
      dimensions: new Uint32Array(memory.buffer.slice(ptr2, ptr2 + len2 * 4)),
      buffer: new Uint8Array(memory.buffer.slice(ptr3, ptr3 + len3 * 1)),
    });
  };
  imports["runtime-v1"]["model::infer"] = function(arg0, arg1, arg2, arg3) {
    const memory = get_export("memory");
    const realloc = get_export("canonical_abi_realloc");
    const len3 = arg2;
    const base3 = arg1;
    const result3 = [];
    for (let i = 0; i < len3; i++) {
      const base = base3 + i * 20;
      const tag0 = data_view(memory).getUint8(base + 0, true);
      if (!(tag0 in ElementType))
      throw new RangeError("invalid discriminant specified for ElementType");
      const ptr1 = data_view(memory).getInt32(base + 4, true);
      const len1 = data_view(memory).getInt32(base + 8, true);
      const ptr2 = data_view(memory).getInt32(base + 12, true);
      const len2 = data_view(memory).getInt32(base + 16, true);
      result3.push({
        elementType: tag0,
        dimensions: new Uint32Array(memory.buffer.slice(ptr1, ptr1 + len1 * 4)),
        buffer: new Uint8Array(memory.buffer.slice(ptr2, ptr2 + len2 * 1)),
      });
    }
    const ret = resources7.get(arg0).infer(result3);
    const variant11 = ret;
    let variant11_0;
    let variant11_1;
    let variant11_2;
    let variant11_3;
    switch (variant11.tag) {
      case "ok": {
        const e = variant11.val;
        const vec8 = e;
        const len8 = vec8.length;
        const result8 = realloc(0, 0, 4, len8 * 20);
        for (let i = 0; i < vec8.length; i++) {
          const e = vec8[i];
          const base = result8 + i * 20;
          const {elementType: v4_0, dimensions: v4_1, buffer: v4_2 } = e;
          const variant5 = v4_0;
          switch (variant5) {
            case 0: {
              const e = variant5;
              data_view(memory).setInt8(base + 0, 0, true);
              break;
            }
            case 1: {
              const e = variant5;
              data_view(memory).setInt8(base + 0, 1, true);
              break;
            }
            case 2: {
              const e = variant5;
              data_view(memory).setInt8(base + 0, 2, true);
              break;
            }
            case 3: {
              const e = variant5;
              data_view(memory).setInt8(base + 0, 3, true);
              break;
            }
            case 4: {
              const e = variant5;
              data_view(memory).setInt8(base + 0, 4, true);
              break;
            }
            case 5: {
              const e = variant5;
              data_view(memory).setInt8(base + 0, 5, true);
              break;
            }
            case 6: {
              const e = variant5;
              data_view(memory).setInt8(base + 0, 6, true);
              break;
            }
            case 7: {
              const e = variant5;
              data_view(memory).setInt8(base + 0, 7, true);
              break;
            }
            case 8: {
              const e = variant5;
              data_view(memory).setInt8(base + 0, 8, true);
              break;
            }
            case 9: {
              const e = variant5;
              data_view(memory).setInt8(base + 0, 9, true);
              break;
            }
            case 10: {
              const e = variant5;
              data_view(memory).setInt8(base + 0, 10, true);
              break;
            }
            default:
            throw new RangeError("invalid variant specified for ElementType");
          }
          const val6 = v4_1;
          const len6 = val6.length;
          const ptr6 = realloc(0, 0, 4, len6 * 4);
          (new Uint8Array(memory.buffer, ptr6, len6 * 4)).set(new Uint8Array(val6.buffer, val6.byteOffset, len6 * 4));
          data_view(memory).setInt32(base + 8, len6, true);
          data_view(memory).setInt32(base + 4, ptr6, true);
          const val7 = v4_2;
          const len7 = val7.length;
          const ptr7 = realloc(0, 0, 1, len7 * 1);
          (new Uint8Array(memory.buffer, ptr7, len7 * 1)).set(new Uint8Array(val7.buffer, val7.byteOffset, len7 * 1));
          data_view(memory).setInt32(base + 16, len7, true);
          data_view(memory).setInt32(base + 12, ptr7, true);
        }
        variant11_0 = 0;
        variant11_1 = result8;
        variant11_2 = len8;
        variant11_3 = 0;
        break;
      }
      case "err": {
        const e = variant11.val;
        const variant10 = e;
        let variant10_0;
        let variant10_1;
        let variant10_2;
        switch (variant10.tag) {
          case "other": {
            const e = variant10.val;
            const ptr9 = utf8_encode(e, realloc, memory);
            const len9 = UTF8_ENCODED_LEN;
            variant10_0 = 0;
            variant10_1 = ptr9;
            variant10_2 = len9;
            break;
          }
          default:
          throw new RangeError("invalid variant specified for ModelInferError");
        }
        variant11_0 = 1;
        variant11_1 = variant10_0;
        variant11_2 = variant10_1;
        variant11_3 = variant10_2;
        break;
      }
      default:
      throw new RangeError("invalid variant specified for expected");
    }
    data_view(memory).setInt32(arg3 + 24, variant11_3, true);
    data_view(memory).setInt32(arg3 + 16, variant11_2, true);
    data_view(memory).setInt32(arg3 + 8, variant11_1, true);
    data_view(memory).setInt32(arg3 + 0, variant11_0, true);
  };
  imports["runtime-v1"]["model::inputs"] = function(arg0, arg1) {
    const memory = get_export("memory");
    const realloc = get_export("canonical_abi_realloc");
    const ret = resources7.get(arg0).inputs();
    const vec4 = ret;
    const len4 = vec4.length;
    const result4 = realloc(0, 0, 4, len4 * 16);
    for (let i = 0; i < vec4.length; i++) {
      const e = vec4[i];
      const base = result4 + i * 16;
      const {elementType: v0_0, dimensions: v0_1 } = e;
      const variant1 = v0_0;
      switch (variant1) {
        case 0: {
          const e = variant1;
          data_view(memory).setInt8(base + 0, 0, true);
          break;
        }
        case 1: {
          const e = variant1;
          data_view(memory).setInt8(base + 0, 1, true);
          break;
        }
        case 2: {
          const e = variant1;
          data_view(memory).setInt8(base + 0, 2, true);
          break;
        }
        case 3: {
          const e = variant1;
          data_view(memory).setInt8(base + 0, 3, true);
          break;
        }
        case 4: {
          const e = variant1;
          data_view(memory).setInt8(base + 0, 4, true);
          break;
        }
        case 5: {
          const e = variant1;
          data_view(memory).setInt8(base + 0, 5, true);
          break;
        }
        case 6: {
          const e = variant1;
          data_view(memory).setInt8(base + 0, 6, true);
          break;
        }
        case 7: {
          const e = variant1;
          data_view(memory).setInt8(base + 0, 7, true);
          break;
        }
        case 8: {
          const e = variant1;
          data_view(memory).setInt8(base + 0, 8, true);
          break;
        }
        case 9: {
          const e = variant1;
          data_view(memory).setInt8(base + 0, 9, true);
          break;
        }
        case 10: {
          const e = variant1;
          data_view(memory).setInt8(base + 0, 10, true);
          break;
        }
        default:
        throw new RangeError("invalid variant specified for ElementType");
      }
      const variant3 = v0_1;
      switch (variant3.tag) {
        case "dynamic": {
          data_view(memory).setInt8(base + 4, 0, true);
          break;
        }
        case "fixed": {
          const e = variant3.val;
          data_view(memory).setInt8(base + 4, 1, true);
          const val2 = e;
          const len2 = val2.length;
          const ptr2 = realloc(0, 0, 4, len2 * 4);
          (new Uint8Array(memory.buffer, ptr2, len2 * 4)).set(new Uint8Array(val2.buffer, val2.byteOffset, len2 * 4));
          data_view(memory).setInt32(base + 12, len2, true);
          data_view(memory).setInt32(base + 8, ptr2, true);
          break;
        }
        default:
        throw new RangeError("invalid variant specified for Dimensions");
      }
    }
    data_view(memory).setInt32(arg1 + 8, len4, true);
    data_view(memory).setInt32(arg1 + 0, result4, true);
  };
  imports["runtime-v1"]["model::outputs"] = function(arg0, arg1) {
    const memory = get_export("memory");
    const realloc = get_export("canonical_abi_realloc");
    const ret = resources7.get(arg0).outputs();
    const vec4 = ret;
    const len4 = vec4.length;
    const result4 = realloc(0, 0, 4, len4 * 16);
    for (let i = 0; i < vec4.length; i++) {
      const e = vec4[i];
      const base = result4 + i * 16;
      const {elementType: v0_0, dimensions: v0_1 } = e;
      const variant1 = v0_0;
      switch (variant1) {
        case 0: {
          const e = variant1;
          data_view(memory).setInt8(base + 0, 0, true);
          break;
        }
        case 1: {
          const e = variant1;
          data_view(memory).setInt8(base + 0, 1, true);
          break;
        }
        case 2: {
          const e = variant1;
          data_view(memory).setInt8(base + 0, 2, true);
          break;
        }
        case 3: {
          const e = variant1;
          data_view(memory).setInt8(base + 0, 3, true);
          break;
        }
        case 4: {
          const e = variant1;
          data_view(memory).setInt8(base + 0, 4, true);
          break;
        }
        case 5: {
          const e = variant1;
          data_view(memory).setInt8(base + 0, 5, true);
          break;
        }
        case 6: {
          const e = variant1;
          data_view(memory).setInt8(base + 0, 6, true);
          break;
        }
        case 7: {
          const e = variant1;
          data_view(memory).setInt8(base + 0, 7, true);
          break;
        }
        case 8: {
          const e = variant1;
          data_view(memory).setInt8(base + 0, 8, true);
          break;
        }
        case 9: {
          const e = variant1;
          data_view(memory).setInt8(base + 0, 9, true);
          break;
        }
        case 10: {
          const e = variant1;
          data_view(memory).setInt8(base + 0, 10, true);
          break;
        }
        default:
        throw new RangeError("invalid variant specified for ElementType");
      }
      const variant3 = v0_1;
      switch (variant3.tag) {
        case "dynamic": {
          data_view(memory).setInt8(base + 4, 0, true);
          break;
        }
        case "fixed": {
          const e = variant3.val;
          data_view(memory).setInt8(base + 4, 1, true);
          const val2 = e;
          const len2 = val2.length;
          const ptr2 = realloc(0, 0, 4, len2 * 4);
          (new Uint8Array(memory.buffer, ptr2, len2 * 4)).set(new Uint8Array(val2.buffer, val2.byteOffset, len2 * 4));
          data_view(memory).setInt32(base + 12, len2, true);
          data_view(memory).setInt32(base + 8, ptr2, true);
          break;
        }
        default:
        throw new RangeError("invalid variant specified for Dimensions");
      }
    }
    data_view(memory).setInt32(arg1 + 8, len4, true);
    data_view(memory).setInt32(arg1 + 0, result4, true);
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
  
  const resources7 = new Slab();
  imports.canonical_abi["resource_drop_model"] = (i) => {
    const val = resources7.remove(i);
    if (obj.dropModel)
    obj.dropModel(val);
  };
}