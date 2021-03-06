function get(k) {
  // Different typed array types to ensure we emit a GetProp IC.
  var xs = [
    new Int32Array(10),
    new Int16Array(10),
  ];

  // Doesn't read values from the prototype chain.
  Object.prototype[k] = 1;

  for (var i = 0; i < 100; ++i) {
    var x = xs[i & 1];
    assertEq(x[k], undefined);
  }
}

// Make sure we use a distinct function for each test.
function test(fn) {
  return Function(`return ${fn};`)();
}

// Negative numbers values aren't int32 indices.
test(get)(-1);
test(get)(-2147483648);
test(get)(-2147483649);

// Int32 indices must be less-or-equal to max-int32.
test(get)(2147483648);

// Int32 indices must not have fractional parts.
test(get)(1.5);
test(get)(-1.5);

// Non-finite numbers aren't int32 indices.
test(get)(NaN);
test(get)(-Infinity);
test(get)(Infinity);
