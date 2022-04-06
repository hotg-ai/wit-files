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
  getPipeline() {
    const memory = this._exports.memory;
    const free = this._exports["canonical_abi_free"];
    const ret = this._exports['get-pipeline']();
    const len6 = data_view(memory).getInt32(ret + 8, true);
    const base6 = data_view(memory).getInt32(ret + 0, true);
    const result6 = [];
    for (let i = 0; i < len6; i++) {
      const base = base6 + i * 24;
      const ptr0 = data_view(memory).getInt32(base + 0, true);
      const len0 = data_view(memory).getInt32(base + 4, true);
      const list0 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0));
      free(ptr0, len0, 1);
      const len2 = data_view(memory).getInt32(base + 12, true);
      const base2 = data_view(memory).getInt32(base + 8, true);
      const result2 = [];
      for (let i = 0; i < len2; i++) {
        const base = base2 + i * 8;
        const ptr1 = data_view(memory).getInt32(base + 0, true);
        const len1 = data_view(memory).getInt32(base + 4, true);
        const list1 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr1, len1));
        free(ptr1, len1, 1);
        result2.push(list1);
      }
      free(base2, len2 * 8, 4);
      const len5 = data_view(memory).getInt32(base + 20, true);
      const base5 = data_view(memory).getInt32(base + 16, true);
      const result5 = [];
      for (let i = 0; i < len5; i++) {
        const base = base5 + i * 16;
        const ptr3 = data_view(memory).getInt32(base + 0, true);
        const len3 = data_view(memory).getInt32(base + 4, true);
        const list3 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr3, len3));
        free(ptr3, len3, 1);
        const ptr4 = data_view(memory).getInt32(base + 8, true);
        const len4 = data_view(memory).getInt32(base + 12, true);
        const list4 = UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr4, len4));
        free(ptr4, len4, 1);
        result5.push([list3, list4]);
      }
      free(base5, len5 * 16, 4);
      result6.push({
        id: list0,
        dependsOn: result2,
        arguments: result5,
      });
    }
    free(base6, len6 * 24, 4);
    return {
      nodes: result6,
    };
  }
}
