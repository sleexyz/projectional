module.exports = grammar({
  name: "puddlejumper",
  rules: {
    document: $ => repeat(seq($.block, optional("\n"))),
    block: $ => /[^\n]+/,
  }
});
