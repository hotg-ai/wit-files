import { data_view, UTF8_DECODER } from './intrinsics.js';
export class RuneV1 {
  addToImports(imports) {
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
  }
  start() {
    this._exports['start']();
  }
  graph() {
    const memory = this._exports.memory;
    const free = this._exports["canonical_abi_free"];
    const ret = this._exports['graph']();
    let variant6;
    switch (data_view(memory).getInt32(ret + 0, true)) {
      case 0: {
        variant6 = {
          tag: "ok",
        };
        break;
      }
      case 1: {
        let variant5;
        switch (data_view(memory).getInt32(ret + 8, true)) {
          case 0: {
            const ptr0 = data_view(memory).getInt32(ret + 16, true);
            const len0 = data_view(memory).getInt32(ret + 24, true);
            const list0 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0));
            free(ptr0, len0, 1);
            let variant3;
            switch (data_view(memory).getInt32(ret + 32, true)) {
              case 0: {
                variant3 = {
                  tag: "not-found",
                };
                break;
              }
              case 1: {
                const ptr1 = data_view(memory).getInt32(ret + 40, true);
                const len1 = data_view(memory).getInt32(ret + 48, true);
                const list1 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr1, len1));
                free(ptr1, len1, 1);
                variant3 = {
                  tag: "invalid-value",
                  val: list1,
                };
                break;
              }
              case 2: {
                const ptr2 = data_view(memory).getInt32(ret + 40, true);
                const len2 = data_view(memory).getInt32(ret + 48, true);
                const list2 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr2, len2));
                free(ptr2, len2, 1);
                variant3 = {
                  tag: "other",
                  val: list2,
                };
                break;
              }
              default:
              throw new RangeError("invalid variant discriminant for BadArgumentReason");
            }
            variant5 = {
              tag: "invalid-argument",
              val: {
                name: list0,
                reason: variant3,
              },
            };
            break;
          }
          case 1: {
            const ptr4 = data_view(memory).getInt32(ret + 16, true);
            const len4 = data_view(memory).getInt32(ret + 24, true);
            const list4 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr4, len4));
            free(ptr4, len4, 1);
            variant5 = {
              tag: "other",
              val: list4,
            };
            break;
          }
          default:
          throw new RangeError("invalid variant discriminant for GraphError");
        }
        variant6 = {
          tag: "err",
          val: variant5,
        };
        break;
      }
      default:
      throw new RangeError("invalid variant discriminant for expected");
    }
    return variant6;
  }
  kernel() {
    const memory = this._exports.memory;
    const free = this._exports["canonical_abi_free"];
    const ret = this._exports['kernel']();
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
            const ptr0 = data_view(memory).getInt32(ret + 16, true);
            const len0 = data_view(memory).getInt32(ret + 24, true);
            const list0 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0));
            free(ptr0, len0, 1);
            let variant3;
            switch (data_view(memory).getInt32(ret + 32, true)) {
              case 0: {
                variant3 = {
                  tag: "not-found",
                };
                break;
              }
              case 1: {
                const ptr1 = data_view(memory).getInt32(ret + 40, true);
                const len1 = data_view(memory).getInt32(ret + 48, true);
                const list1 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr1, len1));
                free(ptr1, len1, 1);
                variant3 = {
                  tag: "invalid-value",
                  val: list1,
                };
                break;
              }
              case 2: {
                const ptr2 = data_view(memory).getInt32(ret + 40, true);
                const len2 = data_view(memory).getInt32(ret + 48, true);
                const list2 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr2, len2));
                free(ptr2, len2, 1);
                variant3 = {
                  tag: "other",
                  val: list2,
                };
                break;
              }
              default:
              throw new RangeError("invalid variant discriminant for BadArgumentReason");
            }
            variant6 = {
              tag: "invalid-argument",
              val: {
                name: list0,
                reason: variant3,
              },
            };
            break;
          }
          case 1: {
            const ptr4 = data_view(memory).getInt32(ret + 16, true);
            const len4 = data_view(memory).getInt32(ret + 24, true);
            const list4 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr4, len4));
            free(ptr4, len4, 1);
            variant6 = {
              tag: "missing-input",
              val: list4,
            };
            break;
          }
          case 2: {
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
          throw new RangeError("invalid variant discriminant for KernelError");
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
}
