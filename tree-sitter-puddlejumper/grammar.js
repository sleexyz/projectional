module.exports = grammar({
  name: "puddlejumper",
  externals: $ => [
    $._indent,
    $._dedent,
    $._newline,
  ],
  rules: {
    document: $ => $._block,
    _block: $ => repeat1(seq($.node, optional($._newline))),
    node: $ => seq(
      $.content,
      optional($._children),
    ),
    _children: $ => seq(
      $._indent,
      $._block,
      $._dedent,
    ),
    content: $ => /[^\n]+/,
  }
});
