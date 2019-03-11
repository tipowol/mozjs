// Copyright (c) 2012 Ecma International.  All rights reserved.
// This code is governed by the BSD license found in the LICENSE file.

/*---
es5id: 15.2.3.7-5-b-257
description: >
    Object.defineProperties - value of 'set' property of 'descObj' is
    an interesting object other than a function (8.10.5 step 8.b)
---*/

var obj = {};
assert.throws(TypeError, function() {
  Object.defineProperties(obj, {
    prop: {
      set: []
    }
  });
});

reportCompare(0, 0);
