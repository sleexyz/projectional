module.exports = grammar({
  name: "puddlejumper",
  externals: $ => [
    $.indent,
    $.dedent,
    $.newline,
  ],
  word: $ => $.identifier,
  rules: {
    document: $ => optional($._stanza),
    _stanza: $ => choice(
      seq(repeat1($.node), repeat($.block_node)),
      repeat1($.block_node),
    ),
    // block_node: $ => prec.left(seq($.binding, $.newline, repeat1(seq($.node, $.newline)))),
    // block_node: $ => prec.left(seq($.binding, $.newline, $.content, repeat(seq($.newline, $.node)), optional($.newline))),

    // block_node: $ => prec.left(seq($.binding, repeat1(seq($.newline, $.node)), optional($.newline))),

    block_node: $ => prec(1, choice(
      (seq($.binding, $.newline, $.content, repeat(seq($.newline, $.node)), optional($.newline))),
      (seq($.binding, $.newline, $.content, $.children)),
    )),

    node: $ => seq($.content, optional($.children) ),
    children: $ => seq(
      $.indent,
      $._stanza,
      $.dedent,
    ),

    identifier: $ => /[a-zA-Z0-9_]+/,
    binding: $ => seq("@", optional($.identifier), ":"),
    ref: $ => seq("@", $.identifier),
    content: $ => /[^@ \n][^\n]*/,
  }
});
