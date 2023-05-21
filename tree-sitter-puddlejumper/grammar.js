/// <reference types="tree-sitter-cli/dsl" />
// @ts-check
module.exports = grammar({
  name: "puddlejumper",
  externals: ($) => [$.indent, $.dedent, $.newline],
  conflicts: ($) => [[$._node_line], [$.block]],
  rules: {
    document: ($) => optional(field("children", $._body)),
    _body: ($) =>
      choice(
        seq(repeat1($._node_line), repeat($._block_node_section)),
        seq(repeat1($._block_node_section))
      ),
    _node_line: ($) => seq($.node, optional($.newline)),

    _block_node_section: ($) => seq($.block, optional($.newline)),
    block: ($) =>
      seq(
        optional(seq(field("binding", $.binding), $.newline)),
        field("header", $.block_header),
        optional(seq(optional($.newline), field("children", $._block_body)))
      ),
    _block_body: ($) => repeat1($._node_line),
    block_header: ($) => seq($._block_begin, /[\s]+/, $.node),
    _block_begin: ($) => token(prec(1, "#")),

    children: ($) => seq($.indent, $._body, $.dedent),

    node: ($) =>
      choice(
        seq(
          field("binding", $.binding),
          $.newline,
          field("content", $._node_content),
          optional(field("children", $.children))
        ),
        seq(
          field("binding", $.binding),
          /\s*/,
          field("content", $._node_content),
          optional(field("children", $.children))
        ),
        seq(
          field("content", $._node_content),
          optional(field("children", $.children))
        ),
        seq(field("binding", $.binding), /\s*/, field("children", $.children))
      ),
    _node_content: ($) => choice($.content, $.ref),
    ref: ($) => $._binder,
    identifier: ($) => token(prec(-1, /[a-zA-Z0-9_]+/)),
    _binder: ($) => seq(token("@"), field("identifier", $.identifier)),
    _anonymous_binder: ($) => token("@"),
    binding: ($) =>
      choice(
        seq($._binder, $._assignment),
        seq($._anonymous_binder, $._assignment)
      ),
    // This needs to bind higher than content, otherwise the colon in a binding (e.g. `@foo: bar`)
    // will be parsed as the start of a content token.
    _assignment: ($) => token(prec(1, ":")),
    content: ($) => token(/[^@# \n][^\n]*/),
  },
});
