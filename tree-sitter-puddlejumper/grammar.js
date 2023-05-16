module.exports = grammar({
  name: "puddlejumper",
  externals: $ => [
    $.indent,
    $.dedent,
    $.newline,
    $.mega_newline,
  ],
  rules: {
    document: $ => optional($._stanza),
    _stanza: $ => repeat1(choice(
      $.node,
      $.block_node,
    )),

    block_node: $ => seq($.binding, repeat1(seq($.newline, $.node)), $.mega_newline),

    node: $ => prec.left(seq(optional(seq($.binding, /\s*/)), $.content, optional(choice($.children, $.newline)))),
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
