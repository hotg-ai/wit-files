import { data_view, UTF8_DECODER } from './intrinsics.js';
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
export function addRuntimeV1ToImports(imports, obj, get_export) {
  if (!("runtime-v1" in imports)) imports["runtime-v1"] = {};
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
    const len19 = arg15;
    const base19 = arg14;
    const result19 = [];
    for (let i = 0; i < len19; i++) {
      const base = base19 + i * 20;
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
      result19.push({
        name: UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr16, len16)),
        description: variant18,
      });
    }
    const len23 = arg17;
    const base23 = arg16;
    const result23 = [];
    for (let i = 0; i < len23; i++) {
      const base = base23 + i * 20;
      const ptr20 = data_view(memory).getInt32(base + 0, true);
      const len20 = data_view(memory).getInt32(base + 4, true);
      let variant22;
      switch (data_view(memory).getUint8(base + 8, true)) {
        case 0: {
          variant22 = null;
          break;
        }
        case 1: {
          const ptr21 = data_view(memory).getInt32(base + 12, true);
          const len21 = data_view(memory).getInt32(base + 16, true);
          variant22 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr21, len21));
          break;
        }
        default:
        throw new RangeError("invalid variant discriminant for option");
      }
      result23.push({
        name: UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr20, len20)),
        description: variant22,
      });
    }
    obj.registerNode({
      name: UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)),
      version: UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr1, len1)),
      description: variant3,
      repository: variant5,
      tags: result7,
      arguments: result15,
      inputs: result19,
      outputs: result23,
    });
  };
}