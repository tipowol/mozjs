// |reftest| skip -- regexp-unicode-property-escapes is not supported
// Copyright 2018 Mathias Bynens. All rights reserved.
// This code is governed by the BSD license found in the LICENSE file.

/*---
author: Mathias Bynens
description: >
  Unicode property escapes for `Script_Extensions=Buginese`
info: |
  Generated by https://github.com/mathiasbynens/unicode-property-escapes-tests
  Unicode v11.0.0
esid: sec-static-semantics-unicodematchproperty-p
features: [regexp-unicode-property-escapes]
includes: [regExpUtils.js]
---*/

const matchSymbols = buildString({
  loneCodePoints: [
    0x00A9CF
  ],
  ranges: [
    [0x001A00, 0x001A1B],
    [0x001A1E, 0x001A1F]
  ]
});
testPropertyEscapes(
  /^\p{Script_Extensions=Buginese}+$/u,
  matchSymbols,
  "\\p{Script_Extensions=Buginese}"
);
testPropertyEscapes(
  /^\p{Script_Extensions=Bugi}+$/u,
  matchSymbols,
  "\\p{Script_Extensions=Bugi}"
);
testPropertyEscapes(
  /^\p{scx=Buginese}+$/u,
  matchSymbols,
  "\\p{scx=Buginese}"
);
testPropertyEscapes(
  /^\p{scx=Bugi}+$/u,
  matchSymbols,
  "\\p{scx=Bugi}"
);

const nonMatchSymbols = buildString({
  loneCodePoints: [],
  ranges: [
    [0x00DC00, 0x00DFFF],
    [0x000000, 0x0019FF],
    [0x001A1C, 0x001A1D],
    [0x001A20, 0x00A9CE],
    [0x00A9D0, 0x00DBFF],
    [0x00E000, 0x10FFFF]
  ]
});
testPropertyEscapes(
  /^\P{Script_Extensions=Buginese}+$/u,
  nonMatchSymbols,
  "\\P{Script_Extensions=Buginese}"
);
testPropertyEscapes(
  /^\P{Script_Extensions=Bugi}+$/u,
  nonMatchSymbols,
  "\\P{Script_Extensions=Bugi}"
);
testPropertyEscapes(
  /^\P{scx=Buginese}+$/u,
  nonMatchSymbols,
  "\\P{scx=Buginese}"
);
testPropertyEscapes(
  /^\P{scx=Bugi}+$/u,
  nonMatchSymbols,
  "\\P{scx=Bugi}"
);

reportCompare(0, 0);
