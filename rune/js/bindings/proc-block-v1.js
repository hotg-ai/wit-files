import { data_view, UTF8_DECODER, utf8_encode, UTF8_ENCODED_LEN } from './intrinsics.js';
export class ProcBlockV1 {
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
  registerMetadata() {
    this._exports['register-metadata']();
  }
  graph(arg0) {
    const memory = this._exports.memory;
    const realloc = this._exports["canonical_abi_realloc"];
    const free = this._exports["canonical_abi_free"];
    const ptr0 = utf8_encode(arg0, realloc, memory);
    const len0 = UTF8_ENCODED_LEN;
    const ret = this._exports['graph'](ptr0, len0);
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
    const realloc = this._exports["canonical_abi_realloc"];
    const free = this._exports["canonical_abi_free"];
    const ptr0 = utf8_encode(arg0, realloc, memory);
    const len0 = UTF8_ENCODED_LEN;
    const ret = this._exports['kernel'](ptr0, len0);
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
