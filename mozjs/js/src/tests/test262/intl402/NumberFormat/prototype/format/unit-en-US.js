// |reftest| skip-if(release_or_beta) -- Intl.NumberFormat-unified is not released yet
// Copyright 2019 Igalia, S.L. All rights reserved.
// This code is governed by the BSD license found in the LICENSE file.

/*---
esid: sec-intl.numberformat.prototype.format
description: Checks handling of the unit style.
locale: [en-US]
features: [Intl.NumberFormat-unified]
---*/


const tests = [
  [
    -987,
    {
      "short": "-987 m",
      "narrow": "-987m",
      "long": "-987 meters",
    }
  ],
  [
    -0.001,
    {
      "short": "-0.001 m",
      "narrow": "-0.001m",
      "long": "-0.001 meters",
    }
  ],
  [
    -0,
    {
      "short": "-0 m",
      "narrow": "-0m",
      "long": "-0 meters",
    }
  ],
  [
    0,
    {
      "short": "0 m",
      "narrow": "0m",
      "long": "0 meters",
    }
  ],
  [
    0.001,
    {
      "short": "0.001 m",
      "narrow": "0.001m",
      "long": "0.001 meters",
    }
  ],
  [
    987,
    {
      "short": "987 m",
      "narrow": "987m",
      "long": "987 meters",
    }
  ],
];

for (const [number, expectedData] of tests) {
  for (const [unitDisplay, expected] of Object.entries(expectedData)) {
    const nf = new Intl.NumberFormat("en-US", { style: "unit", unit: "meter", unitDisplay });
    assert.sameValue(nf.format(number), expected);
  }
}


reportCompare(0, 0);
