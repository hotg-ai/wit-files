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
  1: "Int8",
  "Int8": 1,
  2: "Uint16",
  "Uint16": 2,
  3: "Int16",
  "Int16": 3,
  4: "Uint32",
  "Uint32": 4,
  5: "Int32",
  "Int32": 5,
  6: "Float32",
  "Float32": 6,
  7: "Uint64",
  "Uint64": 7,
  8: "Int64",
  "Int64": 8,
  9: "Float64",
  "Float64": 9,
  10: "Utf8",
  "Utf8": 10,
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
  imports["runtime-v1"]["interpret-as-number-in-range"] = function(arg0, arg1) {
    const ret = obj.interpretAsNumberInRange(arg0, arg1);
    return resources4.insert(ret);
  };
  imports["runtime-v1"]["register-node"] = function(arg0) {
    obj.registerNode(resources0.get(arg0));
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
  imports["runtime-v1"]["argument-metadata::set-type-hint"] = function(arg0, arg1) {
    const tag0 = arg1;
    if (!(tag0 in TypeHint))
    throw new RangeError("invalid discriminant specified for TypeHint");
    resources1.get(arg0).setTypeHint(tag0);
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
}