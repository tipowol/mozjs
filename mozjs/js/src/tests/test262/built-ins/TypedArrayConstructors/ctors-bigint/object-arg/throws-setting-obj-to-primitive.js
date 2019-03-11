// |reftest| skip-if(!this.hasOwnProperty('BigInt')) -- BigInt is not enabled unconditionally
// Copyright (C) 2016 the V8 project authors. All rights reserved.
// This code is governed by the BSD license found in the LICENSE file.
/*---
esid: sec-typedarray-object
description: >
  Return abrupt from @@toPrimitive when setting a property
info: |
  22.2.4.4 TypedArray ( object )

  This description applies only if the TypedArray function is called with at
  least one argument and the Type of the first argument is Object and that
  object does not have either a [[TypedArrayName]] or an [[ArrayBufferData]]
  internal slot.

  ...
  8. Repeat, while k < len
    ...
    b. Let kValue be ? Get(arrayLike, Pk).
    c. Perform ? Set(O, Pk, kValue, true).
  ...

  9.4.5.5 [[Set]] ( P, V, Receiver)

  ...
  2. If Type(P) is String and if SameValue(O, Receiver) is true, then
    a. Let numericIndex be ! CanonicalNumericIndexString(P).
    b. If numericIndex is not undefined, then
      i. Return ? IntegerIndexedElementSet(O, numericIndex, V).
  ...

  9.4.5.9 IntegerIndexedElementSet ( O, index, value )

  ...
  5. If arrayTypeName is "BigUint64Array" or "BigInt64Array",
     let numValue be ? ToBigInt(value).
  ...

  ToBigInt ( argument )

  Object, Apply the following steps:
    1. Let prim be ? ToPrimitive(argument, hint Number).
    2. Return the value that prim corresponds to in Table 10.

  7.1.1 ToPrimitive ( input [ , PreferredType ] )

  ...
  4. Let exoticToPrim be ? GetMethod(input, @@toPrimitive).
  5. If exoticToPrim is not undefined, then
    a. Let result be ? Call(exoticToPrim, input, « hint »).
  ...
includes: [testBigIntTypedArray.js]
features: [BigInt, Symbol.toPrimitive, TypedArray]
---*/

testWithBigIntTypedArrayConstructors(function(TA) {
  var sample = new Int8Array(1);
  var toPrimitive = 0;
  var valueOf = 0;

  sample[Symbol.toPrimitive] = function() {
    toPrimitive++;
    throw new Test262Error();
  };

  sample.valueOf = function() {
    valueOf++;
  };

  assert.throws(Test262Error, function() {
    new TA([8n, sample]);
  }, "abrupt completion from sample @@toPrimitive");

  assert.sameValue(toPrimitive, 1, "toPrimitive was called once");
  assert.sameValue(valueOf, 0, "it does not call sample.valueOf");
});

reportCompare(0, 0);
