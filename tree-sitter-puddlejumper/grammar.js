module.exports = grammar({
  name: "puddlejumper",
  externals: $ => [
    $.indent,
    $.dedent,
    $.newline,
  ],
  word: $ => $.identifier,
  rules: {
    document: $ => $._block,
    _block: $ => repeat1(seq($.node, optional($.newline))),
    identifier: $ => /[a-zA-Z0-9_]+/,
    node: $ => prec(2, choice(
      seq($.binding, $.children),
      seq(optional(seq($.binding, $.newline)), $._bindable),
      $.ref,
    )),
    _bindable: $ => prec(1, choice(
      seq($.content, optional($.children)),
      $.ref,
    )),
    binding: $ => seq("@", $.identifier, ":"),
    ref: $ => seq("@", $.identifier),
    children: $ => seq(
      $.indent,
      $._block,
      $.dedent,
    ),
    content: $ => /[^@ \n][^\n]*/,
  }
});
