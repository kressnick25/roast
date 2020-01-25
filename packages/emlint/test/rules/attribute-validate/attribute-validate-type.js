const t = require("tap");
const { Linter } = require("../../../dist/emlint.cjs");
const { applyFixes } = require("../../../t-util/util");

// 01. validation
// -----------------------------------------------------------------------------

t.test(
  `01.01 - ${`\u001b[${34}m${`validation`}\u001b[${39}m`} - no type, error level 0`,
  t => {
    const str = `<a>`;
    const linter = new Linter();
    const messages = linter.verify(str, {
      rules: {
        "attribute-validate-type": 0
      }
    });
    t.equal(applyFixes(str, messages), str);
    t.same(messages, []);
    t.end();
  }
);

t.test(
  `01.02 - ${`\u001b[${34}m${`validation`}\u001b[${39}m`} - no type, error level 1`,
  t => {
    const str = `<a>`;
    const linter = new Linter();
    const messages = linter.verify(str, {
      rules: {
        "attribute-validate-type": 1
      }
    });
    t.equal(applyFixes(str, messages), str);
    t.same(messages, []);
    t.end();
  }
);

t.test(
  `01.03 - ${`\u001b[${34}m${`validation`}\u001b[${39}m`} - no type, error level 2`,
  t => {
    const str = `<a>`;
    const linter = new Linter();
    const messages = linter.verify(str, {
      rules: {
        "attribute-validate-type": 2
      }
    });
    t.equal(applyFixes(str, messages), str);
    t.same(messages, []);
    t.end();
  }
);

t.test(
  `01.04 - ${`\u001b[${34}m${`validation`}\u001b[${39}m`} - healthy attribute, a`,
  t => {
    const str = `<a type='application/json'>`; // <-- notice single quotes
    const linter = new Linter();
    const messages = linter.verify(str, {
      rules: {
        "attribute-validate-type": 2
      }
    });
    t.equal(applyFixes(str, messages), str);
    t.same(messages, []);
    t.end();
  }
);

t.test(
  `01.05 - ${`\u001b[${34}m${`validation`}\u001b[${39}m`} - fancy MIME from the list`,
  t => {
    const str = `<a type="application/vnd.openxmlformats-officedocument.presentationml.template.main+xml">`; // <-- notice single quotes
    const linter = new Linter();
    const messages = linter.verify(str, {
      rules: {
        "attribute-validate-type": 2
      }
    });
    t.equal(applyFixes(str, messages), str);
    t.same(messages, []);
    t.end();
  }
);

t.test(
  `01.06 - ${`\u001b[${34}m${`validation`}\u001b[${39}m`} - healthy attribute, input`,
  t => {
    const str = `<input type="password">`;
    const linter = new Linter();
    const messages = linter.verify(str, {
      rules: {
        "attribute-validate-type": 2
      }
    });
    t.equal(applyFixes(str, messages), str);
    t.same(messages, []);
    t.end();
  }
);

t.test(
  `01.07 - ${`\u001b[${34}m${`validation`}\u001b[${39}m`} - healthy attribute, li`,
  t => {
    const str = `<li type="disc">`;
    const linter = new Linter();
    const messages = linter.verify(str, {
      rules: {
        "attribute-validate-type": 2
      }
    });
    t.equal(applyFixes(str, messages), str);
    t.same(messages, []);
    t.end();
  }
);

t.test(
  `01.08 - ${`\u001b[${34}m${`validation`}\u001b[${39}m`} - healthy attribute, ol`,
  t => {
    const str = `<ol type="1">`;
    const linter = new Linter();
    const messages = linter.verify(str, {
      rules: {
        "attribute-validate-type": 2
      }
    });
    t.equal(applyFixes(str, messages), str);
    t.same(messages, []);
    t.end();
  }
);

t.test(
  `01.09 - ${`\u001b[${34}m${`validation`}\u001b[${39}m`} - healthy attribute, ul`,
  t => {
    const str = `<ul type="square">`;
    const linter = new Linter();
    const messages = linter.verify(str, {
      rules: {
        "attribute-validate-type": 2
      }
    });
    t.equal(applyFixes(str, messages), str);
    t.same(messages, []);
    t.end();
  }
);

t.test(
  `01.10 - ${`\u001b[${34}m${`validation`}\u001b[${39}m`} - healthy attribute, button`,
  t => {
    const str = `<button type="reset">`;
    const linter = new Linter();
    const messages = linter.verify(str, {
      rules: {
        "attribute-validate-type": 2
      }
    });
    t.equal(applyFixes(str, messages), str);
    t.same(messages, []);
    t.end();
  }
);

// 02. rogue whitespace
// -----------------------------------------------------------------------------

t.test(
  `02.01 - ${`\u001b[${36}m${`whitespace`}\u001b[${39}m`} - space in front`,
  t => {
    const str = `<a type=" application/json">`;
    const linter = new Linter();
    const messages = linter.verify(str, {
      rules: {
        "attribute-validate-type": 2
      }
    });
    t.equal(applyFixes(str, messages), `<a type="application/json">`);
    t.match(messages, [
      {
        ruleId: "attribute-validate-type",
        idxFrom: 9,
        idxTo: 10,
        message: `Remove whitespace.`,
        fix: {
          ranges: [[9, 10]]
        }
      }
    ]);
    t.end();
  }
);

t.test(
  `02.02 - ${`\u001b[${36}m${`whitespace`}\u001b[${39}m`} - space after`,
  t => {
    const str = `<a type="application/json ">`;
    const linter = new Linter();
    const messages = linter.verify(str, {
      rules: {
        "attribute-validate-type": 2
      }
    });
    t.equal(applyFixes(str, messages), `<a type="application/json">`);
    t.match(messages, [
      {
        ruleId: "attribute-validate-type",
        idxFrom: 25,
        idxTo: 26,
        message: `Remove whitespace.`,
        fix: {
          ranges: [[25, 26]]
        }
      }
    ]);
    t.end();
  }
);

t.test(
  `02.03 - ${`\u001b[${36}m${`whitespace`}\u001b[${39}m`} - copious whitespace around`,
  t => {
    const str = `<a type="  application/json \t">`;
    const linter = new Linter();
    const messages = linter.verify(str, {
      rules: {
        "attribute-validate-type": 2
      }
    });
    t.equal(applyFixes(str, messages), `<a type="application/json">`);
    t.match(messages, [
      {
        ruleId: "attribute-validate-type",
        idxFrom: 9,
        idxTo: 29,
        message: `Remove whitespace.`,
        fix: {
          ranges: [
            [9, 11],
            [27, 29]
          ]
        }
      }
    ]);
    t.end();
  }
);

t.test(
  `02.04 - ${`\u001b[${36}m${`whitespace`}\u001b[${39}m`} - only trimmable whitespace as a value`,
  t => {
    const str = `<a type="  \t">`;
    const linter = new Linter();
    const messages = linter.verify(str, {
      rules: {
        "attribute-validate-type": 2
      }
    });
    // can't fix:
    t.equal(applyFixes(str, messages), str);
    t.match(messages, [
      {
        ruleId: "attribute-validate-type",
        idxFrom: 9,
        idxTo: 12,
        message: `Missing value.`,
        fix: null
      }
    ]);
    t.end();
  }
);

// 03. wrong value
// -----------------------------------------------------------------------------

t.test(
  `03.01 - ${`\u001b[${35}m${`value`}\u001b[${39}m`} - an out-of-whack value, a`,
  t => {
    const str = `<a type="tralala">`;
    const linter = new Linter();
    const messages = linter.verify(str, {
      rules: {
        "attribute-validate-type": 2
      }
    });
    // can't fix:
    t.equal(applyFixes(str, messages), str);
    t.match(messages, [
      {
        ruleId: "attribute-validate-type",
        idxFrom: 9,
        idxTo: 16,
        message: `Unrecognised value: "tralala".`,
        fix: null
      }
    ]);
    t.end();
  }
);

t.test(
  `03.02 - ${`\u001b[${35}m${`value`}\u001b[${39}m`} - an out-of-whack value, input`,
  t => {
    const str = `<input type="circle">`;
    const linter = new Linter();
    const messages = linter.verify(str, {
      rules: {
        "attribute-validate-type": 2
      }
    });
    // can't fix:
    t.equal(applyFixes(str, messages), str);
    t.match(messages, [
      {
        ruleId: "attribute-validate-type",
        idxFrom: 13,
        idxTo: 19,
        message: `Unrecognised value: "circle".`,
        fix: null
      }
    ]);
    t.end();
  }
);

t.test(
  `03.03 - ${`\u001b[${35}m${`value`}\u001b[${39}m`} - an out-of-whack value, li`,
  t => {
    const str = `<li type="text">`;
    const linter = new Linter();
    const messages = linter.verify(str, {
      rules: {
        "attribute-validate-type": 2
      }
    });
    // can't fix:
    t.equal(applyFixes(str, messages), str);
    t.match(messages, [
      {
        ruleId: "attribute-validate-type",
        idxFrom: 10,
        idxTo: 14,
        message: `Unrecognised value: "text".`,
        fix: null
      }
    ]);
    t.end();
  }
);

t.test(
  `03.04 - ${`\u001b[${35}m${`value`}\u001b[${39}m`} - an out-of-whack value, ol`,
  t => {
    const str = `<ol type="text">`;
    const linter = new Linter();
    const messages = linter.verify(str, {
      rules: {
        "attribute-validate-type": 2
      }
    });
    // can't fix:
    t.equal(applyFixes(str, messages), str);
    t.match(messages, [
      {
        ruleId: "attribute-validate-type",
        idxFrom: 10,
        idxTo: 14,
        message: `Should be "1|a|A|i|I".`,
        fix: null
      }
    ]);
    t.end();
  }
);

t.test(
  `03.05 - ${`\u001b[${35}m${`value`}\u001b[${39}m`} - an out-of-whack value, ul`,
  t => {
    const str = `<ul type="text">`;
    const linter = new Linter();
    const messages = linter.verify(str, {
      rules: {
        "attribute-validate-type": 2
      }
    });
    // can't fix:
    t.equal(applyFixes(str, messages), str);
    t.match(messages, [
      {
        ruleId: "attribute-validate-type",
        idxFrom: 10,
        idxTo: 14,
        message: `Should be "disc|square|circle".`,
        fix: null
      }
    ]);
    t.end();
  }
);

t.test(
  `03.06 - ${`\u001b[${35}m${`value`}\u001b[${39}m`} - an out-of-whack value, button`,
  t => {
    const str = `<button type="circle">`;
    const linter = new Linter();
    const messages = linter.verify(str, {
      rules: {
        "attribute-validate-type": 2
      }
    });
    // can't fix:
    t.equal(applyFixes(str, messages), str);
    t.match(messages, [
      {
        ruleId: "attribute-validate-type",
        idxFrom: 14,
        idxTo: 20,
        message: `Should be "button|submit|reset".`,
        fix: null
      }
    ]);
    t.end();
  }
);

// 04. wrong parent tag
// -----------------------------------------------------------------------------

t.test(
  `04.01 - ${`\u001b[${35}m${`parent`}\u001b[${39}m`} - recognised tag`,
  t => {
    const str = `<div type="application/json">`;
    const linter = new Linter();
    const messages = linter.verify(str, {
      rules: {
        "attribute-validate-type": 2
      }
    });
    // can't fix:
    t.equal(applyFixes(str, messages), str);
    t.match(messages, [
      {
        ruleId: "attribute-validate-type",
        idxFrom: 5,
        idxTo: 28,
        message: `Tag "div" can't have this attribute.`,
        fix: null
      }
    ]);
    t.end();
  }
);

t.test(
  `04.02 - ${`\u001b[${35}m${`parent`}\u001b[${39}m`} - unrecognised tag`,
  t => {
    const str = `<zzz type="application/json" yyy>`;
    const linter = new Linter();
    const messages = linter.verify(str, {
      rules: {
        "attribute-validate-type": 2
      }
    });
    // can't fix:
    t.equal(applyFixes(str, messages), str);
    t.match(messages, [
      {
        ruleId: "attribute-validate-type",
        idxFrom: 5,
        idxTo: 28,
        message: `Tag "zzz" can't have this attribute.`,
        fix: null
      }
    ]);
    t.end();
  }
);