import { data_view, UTF8_DECODER, Slab } from './intrinsics.js';
export class RuneV1 {
  constructor() {
    this._resource0_slab = new Slab();
    this._resource1_slab = new Slab();
  }
  addToImports(imports) {
    if (!("canonical_abi" in imports)) imports["canonical_abi"] = {};
    
    imports.canonical_abi['resource_drop_graph-context'] = i => {
      this._resource0_slab.remove(i).drop();
    };
    imports.canonical_abi['resource_clone_graph-context'] = i => {
      const obj = this._resource0_slab.get(i);
      return this._resource0_slab.insert(obj.clone())
    };
    imports.canonical_abi['resource_get_graph-context'] = i => {
      return this._resource0_slab.get(i)._wasm_val;
    };
    imports.canonical_abi['resource_new_graph-context'] = i => {
      const registry = this._registry0;
      return this._resource0_slab.insert(new GraphContext(i, this));
    };
    
    imports.canonical_abi['resource_drop_kernel-context'] = i => {
      this._resource1_slab.remove(i).drop();
    };
    imports.canonical_abi['resource_clone_kernel-context'] = i => {
      const obj = this._resource1_slab.get(i);
      return this._resource1_slab.insert(obj.clone())
    };
    imports.canonical_abi['resource_get_kernel-context'] = i => {
      return this._resource1_slab.get(i)._wasm_val;
    };
    imports.canonical_abi['resource_new_kernel-context'] = i => {
      const registry = this._registry1;
      return this._resource1_slab.insert(new KernelContext(i, this));
    };
  }
  
  async instantiate(module, imports) {
    imports = imports || {};
    this.addToImports(imports);
    
    if (module instanceof WebAssembly.Instance) {
      this.instance = module;
    } else if (module instanceof WebAssembly.Module) {
      this.instance = await WebAssembly.instantiate(module, imports);
    } else if (module instanceof ArrayBuffer || module instanceof Uint8Array) {
      const { instance } = await WebAssembly.instantiate(module, imports);
      this.instance = instance;
    } else {
      const { instance } = await WebAssembly.instantiateStreaming(module, imports);
      this.instance = instance;
    }
    this._exports = this.instance.exports;
    this._registry0 = new FinalizationRegistry(this._exports['canonical_abi_drop_graph-context']);
    this._registry1 = new FinalizationRegistry(this._exports['canonical_abi_drop_kernel-context']);
  }
  start() {
    this._exports['start']();
  }
  graph(arg0) {
    const memory = this._exports.memory;
    const free = this._exports["canonical_abi_free"];
    const obj0 = arg0;
    if (!(obj0 instanceof GraphContext)) throw new TypeError('expected instance of GraphContext');
    const ret = this._exports['graph'](this._resource0_slab.insert(obj0.clone()));
    let variant7;
    switch (data_view(memory).getInt32(ret + 0, true)) {
      case 0: {
        variant7 = {
          tag: "ok",
        };
        break;
      }
      case 1: {
        let variant6;
        switch (data_view(memory).getInt32(ret + 8, true)) {
          case 0: {
            const ptr1 = data_view(memory).getInt32(ret + 16, true);
            const len1 = data_view(memory).getInt32(ret + 24, true);
            const list1 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr1, len1));
            free(ptr1, len1, 1);
            let variant4;
            switch (data_view(memory).getInt32(ret + 32, true)) {
              case 0: {
                variant4 = {
                  tag: "not-found",
                };
                break;
              }
              case 1: {
                const ptr2 = data_view(memory).getInt32(ret + 40, true);
                const len2 = data_view(memory).getInt32(ret + 48, true);
                const list2 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr2, len2));
                free(ptr2, len2, 1);
                variant4 = {
                  tag: "invalid-value",
                  val: list2,
                };
                break;
              }
              case 2: {
                const ptr3 = data_view(memory).getInt32(ret + 40, true);
                const len3 = data_view(memory).getInt32(ret + 48, true);
                const list3 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr3, len3));
                free(ptr3, len3, 1);
                variant4 = {
                  tag: "other",
                  val: list3,
                };
                break;
              }
              default:
              throw new RangeError("invalid variant discriminant for BadArgumentReason");
            }
            variant6 = {
              tag: "invalid-argument",
              val: {
                name: list1,
                reason: variant4,
              },
            };
            break;
          }
          case 1: {
            const ptr5 = data_view(memory).getInt32(ret + 16, true);
            const len5 = data_view(memory).getInt32(ret + 24, true);
            const list5 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr5, len5));
            free(ptr5, len5, 1);
            variant6 = {
              tag: "other",
              val: list5,
            };
            break;
          }
          default:
          throw new RangeError("invalid variant discriminant for GraphError");
        }
        variant7 = {
          tag: "err",
          val: variant6,
        };
        break;
      }
      default:
      throw new RangeError("invalid variant discriminant for expected");
    }
    return variant7;
  }
  kernel(arg0) {
    const memory = this._exports.memory;
    const free = this._exports["canonical_abi_free"];
    const obj0 = arg0;
    if (!(obj0 instanceof KernelContext)) throw new TypeError('expected instance of KernelContext');
    const ret = this._exports['kernel'](this._resource1_slab.insert(obj0.clone()));
    let variant8;
    switch (data_view(memory).getInt32(ret + 0, true)) {
      case 0: {
        variant8 = {
          tag: "ok",
        };
        break;
      }
      case 1: {
        let variant7;
        switch (data_view(memory).getInt32(ret + 8, true)) {
          case 0: {
            const ptr1 = data_view(memory).getInt32(ret + 16, true);
            const len1 = data_view(memory).getInt32(ret + 24, true);
            const list1 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr1, len1));
            free(ptr1, len1, 1);
            let variant4;
            switch (data_view(memory).getInt32(ret + 32, true)) {
              case 0: {
                variant4 = {
                  tag: "not-found",
                };
                break;
              }
              case 1: {
                const ptr2 = data_view(memory).getInt32(ret + 40, true);
                const len2 = data_view(memory).getInt32(ret + 48, true);
                const list2 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr2, len2));
                free(ptr2, len2, 1);
                variant4 = {
                  tag: "invalid-value",
                  val: list2,
                };
                break;
              }
              case 2: {
                const ptr3 = data_view(memory).getInt32(ret + 40, true);
                const len3 = data_view(memory).getInt32(ret + 48, true);
                const list3 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr3, len3));
                free(ptr3, len3, 1);
                variant4 = {
                  tag: "other",
                  val: list3,
                };
                break;
              }
              default:
              throw new RangeError("invalid variant discriminant for BadArgumentReason");
            }
            variant7 = {
              tag: "invalid-argument",
              val: {
                name: list1,
                reason: variant4,
              },
            };
            break;
          }
          case 1: {
            const ptr5 = data_view(memory).getInt32(ret + 16, true);
            const len5 = data_view(memory).getInt32(ret + 24, true);
            const list5 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr5, len5));
            free(ptr5, len5, 1);
            variant7 = {
              tag: "missing-input",
              val: list5,
            };
            break;
          }
          case 2: {
            const ptr6 = data_view(memory).getInt32(ret + 16, true);
            const len6 = data_view(memory).getInt32(ret + 24, true);
            const list6 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr6, len6));
            free(ptr6, len6, 1);
            variant7 = {
              tag: "other",
              val: list6,
            };
            break;
          }
          default:
          throw new RangeError("invalid variant discriminant for KernelError");
        }
        variant8 = {
          tag: "err",
          val: variant7,
        };
        break;
      }
      default:
      throw new RangeError("invalid variant discriminant for expected");
    }
    return variant8;
  }
}

export class GraphContext {
  constructor(wasm_val, obj) {
    this._wasm_val = wasm_val;
    this._obj = obj;
    this._refcnt = 1;
    obj._registry0.register(this, wasm_val, this);
  }
  
  clone() {
    this._refcnt += 1;
    return this;
  }
  
  drop() {
    this._refcnt -= 1;
    if (this._refcnt !== 0)
    return;
    this._obj._registry0.unregister(this);
    const dtor = this._obj._exports['canonical_abi_drop_graph-context'];
    const wasm_val = this._wasm_val;
    delete this._obj;
    delete this._refcnt;
    delete this._wasm_val;
    dtor(wasm_val);
  }
}

export class KernelContext {
  constructor(wasm_val, obj) {
    this._wasm_val = wasm_val;
    this._obj = obj;
    this._refcnt = 1;
    obj._registry1.register(this, wasm_val, this);
  }
  
  clone() {
    this._refcnt += 1;
    return this;
  }
  
  drop() {
    this._refcnt -= 1;
    if (this._refcnt !== 0)
    return;
    this._obj._registry1.unregister(this);
    const dtor = this._obj._exports['canonical_abi_drop_kernel-context'];
    const wasm_val = this._wasm_val;
    delete this._obj;
    delete this._refcnt;
    delete this._wasm_val;
    dtor(wasm_val);
  }
}
