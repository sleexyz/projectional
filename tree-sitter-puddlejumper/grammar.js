module.exports = grammar({
  name: "puddlejumper",
  externals: $ => [
    $.indent,
    $.dedent,
    $.newline,
  ],
  rules: {
    document: $ => $._block,
    _block: $ => repeat1(seq($.node, optional($.newline))),
    node: $ => seq(
      $.content,
      optional($.children),
    ),
    children: $ => seq(
      $.indent,
      $._block,
      $.dedent,
    ),
    content: $ => /[^\n]+/,
  }
});
