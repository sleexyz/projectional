/// <reference types="tree-sitter-cli/dsl" />
// @ts-check
module.exports = grammar({
  name: "puddlejumper",
  externals: ($) => [$.indent, $.dedent, $.newline, $.mega_newline],
  precedences: ($) => [["binding", "ref"]],
  extras: ($) => [],
  word: ($) => $.identifier,
  rules: {
    document: ($) => optional($._stanza),
    _stanza: ($) =>
      repeat1(
        choice(
          seq($._primary_node, optional($.newline)),
          seq($.block_node, optional($.mega_newline))
        )
      ),
    _primary_node: ($) => choice($.node, $.ref_node),

    block_node: ($) => seq($.binding, /\s*/, repeat1(seq($.newline, $.node))),
    node: ($) =>
      seq(optional(seq($.binding, /\s*/)), $.content, optional($.children)),
    ref_node: ($) => seq($.ref, optional($.children)),
    children: ($) => seq($.indent, $._stanza, $.dedent),

    identifier: ($) => /[a-zA-Z0-9_]+/,
    binding: ($) =>
      seq(token(prec("binding", "@")), optional($.identifier), ":"),
    ref: ($) => seq(token(prec("ref", "@")), $.identifier),
    _node_content: ($) => choice($.content, $.ref),
    content: ($) => /[^@ \n][^\n]*/,
  },
});
