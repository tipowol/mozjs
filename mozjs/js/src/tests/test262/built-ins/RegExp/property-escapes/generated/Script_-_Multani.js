// |reftest| skip -- regexp-unicode-property-escapes is not supported
// Copyright 2018 Mathias Bynens. All rights reserved.
// This code is governed by the BSD license found in the LICENSE file.

/*---
author: Mathias Bynens
description: >
  Unicode property escapes for `Script=Multani`
info: |
  Generated by https://github.com/mathiasbynens/unicode-property-escapes-tests
  Unicode v11.0.0
esid: sec-static-semantics-unicodematchproperty-p
features: [regexp-unicode-property-escapes]
includes: [regExpUtils.js]
---*/

const matchSymbols = buildString({
  loneCodePoints: [
    0x011288
  ],
  ranges: [
    [0x011280, 0x011286],
    [0x01128A, 0x01128D],
    [0x01128F, 0x01129D],
    [0x01129F, 0x0112A9]
  ]
});
testPropertyEscapes(
  /^\p{Script=Multani}+$/u,
  matchSymbols,
  "\\p{Script=Multani}"
);
testPropertyEscapes(
  /^\p{Script=Mult}+$/u,
  matchSymbols,
  "\\p{Script=Mult}"
);
testPropertyEscapes(
  /^\p{sc=Multani}+$/u,
  matchSymbols,
  "\\p{sc=Multani}"
);
testPropertyEscapes(
  /^\p{sc=Mult}+$/u,
  matchSymbols,
  "\\p{sc=Mult}"
);

const nonMatchSymbols = buildString({
  loneCodePoints: [
    0x011287,
    0x011289,
    0x01128E,
    0x01129E
  ],
  ranges: [
    [0x00DC00, 0x00DFFF],
    [0x000000, 0x00DBFF],
    [0x00E000, 0x01127F],
    [0x0112AA, 0x10FFFF]
  ]
});
testPropertyEscapes(
  /^\P{Script=Multani}+$/u,
  nonMatchSymbols,
  "\\P{Script=Multani}"
);
testPropertyEscapes(
  /^\P{Script=Mult}+$/u,
  nonMatchSymbols,
  "\\P{Script=Mult}"
);
testPropertyEscapes(
  /^\P{sc=Multani}+$/u,
  nonMatchSymbols,
  "\\P{sc=Multani}"
);
testPropertyEscapes(
  /^\P{sc=Mult}+$/u,
  nonMatchSymbols,
  "\\P{sc=Mult}"
);

reportCompare(0, 0);
