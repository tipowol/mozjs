// |reftest| skip-if(!xulRuntime.shell||!this.hasOwnProperty('Atomics')||!this.hasOwnProperty('SharedArrayBuffer')) -- browser cannot block main thread, Atomics,SharedArrayBuffer is not enabled unconditionally
// Copyright (C) 2018 Amal Hussein. All rights reserved.
// This code is governed by the BSD license found in the LICENSE file.

/*---
esid: sec-atomics.wait
description: >
  Throws a TypeError if index arg can not be converted to an Integer
info: |
  Atomics.wait( typedArray, index, value, timeout )

  4. Let q be ? ToNumber(timeout).

    Null -> Return +0.

features: [Atomics, SharedArrayBuffer, Symbol, Symbol.toPrimitive, TypedArray]
flags: [CanBlockIsTrue]
---*/

const i32a = new Int32Array(
  new SharedArrayBuffer(Int32Array.BYTES_PER_ELEMENT * 4)
);

const valueOf = {
  valueOf: function() {
    return null;
  }
};

const toPrimitive = {
  [Symbol.toPrimitive]: function() {
    return null;
  }
};

assert.sameValue(
  Atomics.wait(i32a, 0, 0, null),
  "timed-out",
  'Atomics.wait(i32a, 0, 0, null) returns "timed-out"'
);
assert.sameValue(
  Atomics.wait(i32a, 0, 0, valueOf),
  "timed-out",
  'Atomics.wait(i32a, 0, 0, valueOf) returns "timed-out"'
);
assert.sameValue(
  Atomics.wait(i32a, 0, 0, toPrimitive),
  "timed-out",
  'Atomics.wait(i32a, 0, 0, toPrimitive) returns "timed-out"'
);

reportCompare(0, 0);
