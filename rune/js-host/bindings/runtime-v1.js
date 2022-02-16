import { UTF8_DECODER, Slab } from './intrinsics.js';
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
    return resources3.insert(ret);
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
}