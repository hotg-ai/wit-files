import { data_view, UTF8_DECODER, Slab } from './intrinsics.js';
export const TypeHint = Object.freeze({
  0: "Integer",
  "Integer": 0,
  1: "Float",
  "Float": 1,
  2: "OnelineString",
  "OnelineString": 2,
  3: "MultilineString",
  "MultilineString": 3,
});
export const ElementType = Object.freeze({
  0: "Uint8",
  "Uint8": 0,
  1: "Uintint8",
  "Uintint8": 1,
  2: "Uint16",
  "Uint16": 2,
  3: "Uintint16",
  "Uintint16": 3,
  4: "Uint32",
  "Uint32": 4,
  5: "Uintint32",
  "Uintint32": 5,
  6: "Float32",
  "Float32": 6,
  7: "Uint64",
  "Uint64": 7,
  8: "Uintint64",
  "Uintint64": 8,
  9: "Float64",
  "Float64": 9,
});
export function addRuntimeV1ToImports(imports, obj, get_export) {
  if (!("runtime-v1" in imports)) imports["runtime-v1"] = {};
  imports["runtime-v1"]["interpret-as-image"] = function() {
    const ret = obj.interpretAsImage();
    return resources0.insert(ret);
  };
  imports["runtime-v1"]["interpret-as-audio"] = function() {
    const ret = obj.interpretAsAudio();
    return resources0.insert(ret);
  };
  imports["runtime-v1"]["example-shape"] = function(arg0, arg1, arg2, arg3) {
    const memory = get_export("memory");
    const tag0 = arg0;
    if (!(tag0 in ElementType))
    throw new RangeError("invalid discriminant specified for ElementType");
    let variant2;
    switch (arg1) {
      case 0: {
        variant2 = {
          tag: "dynamic",
        };
        break;
      }
      case 1: {
        const ptr1 = arg2;
        const len1 = arg3;
        variant2 = {
          tag: "fixed",
          val: new Uint32Array(memory.buffer.slice(ptr1, ptr1 + len1 * 4)),
        };
        break;
      }
      default:
      throw new RangeError("invalid variant discriminant for Dimensions");
    }
    const ret = obj.exampleShape(tag0, variant2);
    return resources0.insert(ret);
  };
  imports["runtime-v1"]["register-node"] = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14, arg15, arg16, arg17) {
    const memory = get_export("memory");
    const ptr0 = arg0;
    const len0 = arg1;
    const ptr1 = arg2;
    const len1 = arg3;
    let variant3;
    switch (arg4) {
      case 0: {
        variant3 = null;
        break;
      }
      case 1: {
        const ptr2 = arg5;
        const len2 = arg6;
        variant3 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr2, len2));
        break;
      }
      default:
      throw new RangeError("invalid variant discriminant for option");
    }
    let variant5;
    switch (arg7) {
      case 0: {
        variant5 = null;
        break;
      }
      case 1: {
        const ptr4 = arg8;
        const len4 = arg9;
        variant5 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr4, len4));
        break;
      }
      default:
      throw new RangeError("invalid variant discriminant for option");
    }
    const len7 = arg11;
    const base7 = arg10;
    const result7 = [];
    for (let i = 0; i < len7; i++) {
      const base = base7 + i * 8;
      const ptr6 = data_view(memory).getInt32(base + 0, true);
      const len6 = data_view(memory).getInt32(base + 4, true);
      result7.push(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr6, len6)));
    }
    const len15 = arg13;
    const base15 = arg12;
    const result15 = [];
    for (let i = 0; i < len15; i++) {
      const base = base15 + i * 36;
      const ptr8 = data_view(memory).getInt32(base + 0, true);
      const len8 = data_view(memory).getInt32(base + 4, true);
      let variant10;
      switch (data_view(memory).getUint8(base + 8, true)) {
        case 0: {
          variant10 = null;
          break;
        }
        case 1: {
          const ptr9 = data_view(memory).getInt32(base + 12, true);
          const len9 = data_view(memory).getInt32(base + 16, true);
          variant10 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr9, len9));
          break;
        }
        default:
        throw new RangeError("invalid variant discriminant for option");
      }
      let variant12;
      switch (data_view(memory).getUint8(base + 20, true)) {
        case 0: {
          variant12 = null;
          break;
        }
        case 1: {
          const ptr11 = data_view(memory).getInt32(base + 24, true);
          const len11 = data_view(memory).getInt32(base + 28, true);
          variant12 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr11, len11));
          break;
        }
        default:
        throw new RangeError("invalid variant discriminant for option");
      }
      let variant14;
      switch (data_view(memory).getUint8(base + 32, true)) {
        case 0: {
          variant14 = null;
          break;
        }
        case 1: {
          const tag13 = data_view(memory).getUint8(base + 33, true);
          if (!(tag13 in TypeHint))
          throw new RangeError("invalid discriminant specified for TypeHint");
          variant14 = tag13;
          break;
        }
        default:
        throw new RangeError("invalid variant discriminant for option");
      }
      result15.push({
        name: UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr8, len8)),
        description: variant10,
        defaultValue: variant12,
        typeHint: variant14,
      });
    }
    const len20 = arg15;
    const base20 = arg14;
    const result20 = [];
    for (let i = 0; i < len20; i++) {
      const base = base20 + i * 28;
      const ptr16 = data_view(memory).getInt32(base + 0, true);
      const len16 = data_view(memory).getInt32(base + 4, true);
      let variant18;
      switch (data_view(memory).getUint8(base + 8, true)) {
        case 0: {
          variant18 = null;
          break;
        }
        case 1: {
          const ptr17 = data_view(memory).getInt32(base + 12, true);
          const len17 = data_view(memory).getInt32(base + 16, true);
          variant18 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr17, len17));
          break;
        }
        default:
        throw new RangeError("invalid variant discriminant for option");
      }
      const len19 = data_view(memory).getInt32(base + 24, true);
      const base19 = data_view(memory).getInt32(base + 20, true);
      const result19 = [];
      for (let i = 0; i < len19; i++) {
        const base = base19 + i * 4;
        result19.push(resources0.get(data_view(memory).getInt32(base + 0, true)));
      }
      result20.push({
        name: UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr16, len16)),
        description: variant18,
        hints: result19,
      });
    }
    const len25 = arg17;
    const base25 = arg16;
    const result25 = [];
    for (let i = 0; i < len25; i++) {
      const base = base25 + i * 28;
      const ptr21 = data_view(memory).getInt32(base + 0, true);
      const len21 = data_view(memory).getInt32(base + 4, true);
      let variant23;
      switch (data_view(memory).getUint8(base + 8, true)) {
        case 0: {
          variant23 = null;
          break;
        }
        case 1: {
          const ptr22 = data_view(memory).getInt32(base + 12, true);
          const len22 = data_view(memory).getInt32(base + 16, true);
          variant23 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr22, len22));
          break;
        }
        default:
        throw new RangeError("invalid variant discriminant for option");
      }
      const len24 = data_view(memory).getInt32(base + 24, true);
      const base24 = data_view(memory).getInt32(base + 20, true);
      const result24 = [];
      for (let i = 0; i < len24; i++) {
        const base = base24 + i * 4;
        result24.push(resources0.get(data_view(memory).getInt32(base + 0, true)));
      }
      result25.push({
        name: UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr21, len21)),
        description: variant23,
        hints: result24,
      });
    }
    obj.registerNode({
      name: UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)),
      version: UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr1, len1)),
      description: variant3,
      repository: variant5,
      tags: result7,
      arguments: result15,
      inputs: result20,
      outputs: result25,
    });
  };
  if (!("canonical_abi" in imports)) imports["canonical_abi"] = {};
  
  const resources0 = new Slab();
  imports.canonical_abi["resource_drop_tensor-hint"] = (i) => {
    const val = resources0.remove(i);
    if (obj.dropTensorHint)
    obj.dropTensorHint(val);
  };
}