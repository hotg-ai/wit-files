export const UTF8_DECODER = new TextDecoder('utf-8');

export class Slab {
  constructor() {
    this.list = [];
    this.head = 0;
  }
  
  insert(val) {
    if (this.head >= this.list.length) {
      this.list.push({
        next: this.list.length + 1,
        val: undefined,
      });
    }
    const ret = this.head;
    const slot = this.list[ret];
    this.head = slot.next;
    slot.next = -1;
    slot.val = val;
    return ret;
  }
  
  get(idx) {
    if (idx >= this.list.length)
    throw new RangeError('handle index not valid');
    const slot = this.list[idx];
    if (slot.next === -1)
    return slot.val;
    throw new RangeError('handle index not valid');
  }
  
  remove(idx) {
    const ret = this.get(idx); // validate the slot
    const slot = this.list[idx];
    slot.val = undefined;
    slot.next = this.head;
    this.head = idx;
    return ret;
  }
}
