/// <reference types="tree-sitter-cli/dsl" />
// @ts-check
module.exports = grammar({
  name: "puddlejumper",
  externals: $ => [
    $.indent, 
    $.dedent, 
    $.newline, 
  ],
  conflicts: $ => [
    [$._primary_node_line],
    [$.block],
  ],
  rules: {
    document: $ => optional($._stanza),
    _stanza: $ => choice(
      seq(repeat1($._primary_node_line), repeat($._block_node_section)),
      seq(repeat1($._block_node_section)),
    ),
    _primary_node_line: $ => seq($._primary_node, optional($.newline)),
    _primary_node: $ => $.node, 

    _block_node_section: $ => seq($.block, optional($.newline)),
    block: $ => seq(
      optional(seq($.binding, $.newline)),
      $.block_header, 
      optional(seq(optional($.newline), $._block_body)),
    ),
    _block_body: $ => repeat1($._primary_node_line),
    block_header: $ => seq($._block_begin, /[\s]+/, $._primary_node),
    _block_begin: $ => token(prec(1, "#")),

    children: $ => seq($.indent, $._stanza, $.dedent),

    node: $ => choice(
      seq($.binding, $.newline, $._node_content, optional($.children)),
      seq($.binding, /\s*/, $._node_content, optional($.children)),
      seq($._node_content, optional($.children)),
      seq($.binding, /\s*/, $.children),
    ),
    _node_content: $ => choice(
      $.content,
      $.ref
    ),
    ref: $ => $._binder,
    identifier: $ => token(prec(-1, /[a-zA-Z0-9_]+/)),
    _binder: $ => seq(token("@"), $.identifier),
    _anonymous_binder: $ => token("@"),
    binding: $ => choice(
      seq($._binder,  $._assignment),
      seq($._anonymous_binder,  $._assignment),
    ),
    // This needs to bind higher than content, otherwise the colon in a binding (e.g. `@foo: bar`)
    // will be parsed as the start of a content token.
    _assignment: $ => token(prec(1, ":")),
    content: $ => token(/[^@# \n][^\n]*/),
  },
});