module.exports = grammar({
  name: "puddlejumper",
  externals: $ => [
    $._indent,
    $._dedent,
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
      $._indent,
      $._block,
      $._dedent,
    ),
    content: $ => /[^\n]+/,
  }
});
