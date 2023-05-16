#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 41
#define LARGE_STATE_COUNT 7
#define SYMBOL_COUNT 15
#define ALIAS_COUNT 0
#define TOKEN_COUNT 8
#define EXTERNAL_TOKEN_COUNT 3
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 3
#define PRODUCTION_ID_COUNT 1

enum {
  sym_identifier = 1,
  anon_sym_AT = 2,
  anon_sym_COLON = 3,
  sym_content = 4,
  sym_indent = 5,
  sym_dedent = 6,
  sym_newline = 7,
  sym_document = 8,
  aux_sym__block = 9,
  sym_node = 10,
  sym__bindable = 11,
  sym_binding = 12,
  sym_ref = 13,
  sym_children = 14,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_identifier] = "identifier",
  [anon_sym_AT] = "@",
  [anon_sym_COLON] = ":",
  [sym_content] = "content",
  [sym_indent] = "indent",
  [sym_dedent] = "dedent",
  [sym_newline] = "newline",
  [sym_document] = "document",
  [aux_sym__block] = "_block",
  [sym_node] = "node",
  [sym__bindable] = "_bindable",
  [sym_binding] = "binding",
  [sym_ref] = "ref",
  [sym_children] = "children",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_identifier] = sym_identifier,
  [anon_sym_AT] = anon_sym_AT,
  [anon_sym_COLON] = anon_sym_COLON,
  [sym_content] = sym_content,
  [sym_indent] = sym_indent,
  [sym_dedent] = sym_dedent,
  [sym_newline] = sym_newline,
  [sym_document] = sym_document,
  [aux_sym__block] = aux_sym__block,
  [sym_node] = sym_node,
  [sym__bindable] = sym__bindable,
  [sym_binding] = sym_binding,
  [sym_ref] = sym_ref,
  [sym_children] = sym_children,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_AT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [sym_content] = {
    .visible = true,
    .named = true,
  },
  [sym_indent] = {
    .visible = true,
    .named = true,
  },
  [sym_dedent] = {
    .visible = true,
    .named = true,
  },
  [sym_newline] = {
    .visible = true,
    .named = true,
  },
  [sym_document] = {
    .visible = true,
    .named = true,
  },
  [aux_sym__block] = {
    .visible = false,
    .named = false,
  },
  [sym_node] = {
    .visible = true,
    .named = true,
  },
  [sym__bindable] = {
    .visible = false,
    .named = true,
  },
  [sym_binding] = {
    .visible = true,
    .named = true,
  },
  [sym_ref] = {
    .visible = true,
    .named = true,
  },
  [sym_children] = {
    .visible = true,
    .named = true,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 2,
  [6] = 3,
  [7] = 7,
  [8] = 7,
  [9] = 9,
  [10] = 9,
  [11] = 11,
  [12] = 11,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 14,
  [19] = 19,
  [20] = 20,
  [21] = 20,
  [22] = 19,
  [23] = 13,
  [24] = 16,
  [25] = 25,
  [26] = 26,
  [27] = 26,
  [28] = 15,
  [29] = 25,
  [30] = 17,
  [31] = 31,
  [32] = 32,
  [33] = 32,
  [34] = 31,
  [35] = 35,
  [36] = 36,
  [37] = 37,
  [38] = 36,
  [39] = 37,
  [40] = 40,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(3);
      if (lookahead == ':') ADVANCE(7);
      if (lookahead == '@') ADVANCE(5);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(4);
      END_STATE();
    case 1:
      if (eof) ADVANCE(3);
      if (lookahead == ':') ADVANCE(8);
      if (lookahead == '@') ADVANCE(5);
      if (lookahead == '\t' ||
          lookahead == '\r') ADVANCE(10);
      if (lookahead == '\n' ||
          lookahead == ' ') SKIP(1)
      if (lookahead != 0) ADVANCE(11);
      END_STATE();
    case 2:
      if (eof) ADVANCE(3);
      if (lookahead == '@') ADVANCE(5);
      if (lookahead == '\t' ||
          lookahead == '\r') ADVANCE(9);
      if (lookahead == '\n' ||
          lookahead == ' ') SKIP(2)
      if (lookahead != 0) ADVANCE(11);
      END_STATE();
    case 3:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 4:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(4);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(anon_sym_AT);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(anon_sym_AT);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(11);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(anon_sym_COLON);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(11);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(sym_content);
      if (lookahead == ' ') ADVANCE(9);
      if (lookahead == '@') ADVANCE(6);
      if (lookahead == '\t' ||
          lookahead == '\r') ADVANCE(9);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(11);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(sym_content);
      if (lookahead == ' ') ADVANCE(10);
      if (lookahead == ':') ADVANCE(8);
      if (lookahead == '@') ADVANCE(6);
      if (lookahead == '\t' ||
          lookahead == '\r') ADVANCE(10);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(11);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(sym_content);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(11);
      END_STATE();
    default:
      return false;
  }
}

static bool ts_lex_keywords(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0, .external_lex_state = 1},
  [1] = {.lex_state = 2},
  [2] = {.lex_state = 2},
  [3] = {.lex_state = 2, .external_lex_state = 2},
  [4] = {.lex_state = 2},
  [5] = {.lex_state = 2, .external_lex_state = 2},
  [6] = {.lex_state = 2, .external_lex_state = 2},
  [7] = {.lex_state = 2},
  [8] = {.lex_state = 2},
  [9] = {.lex_state = 2, .external_lex_state = 3},
  [10] = {.lex_state = 2, .external_lex_state = 1},
  [11] = {.lex_state = 1, .external_lex_state = 4},
  [12] = {.lex_state = 1, .external_lex_state = 5},
  [13] = {.lex_state = 2, .external_lex_state = 5},
  [14] = {.lex_state = 2, .external_lex_state = 4},
  [15] = {.lex_state = 2, .external_lex_state = 5},
  [16] = {.lex_state = 2, .external_lex_state = 5},
  [17] = {.lex_state = 2},
  [18] = {.lex_state = 2, .external_lex_state = 5},
  [19] = {.lex_state = 2, .external_lex_state = 4},
  [20] = {.lex_state = 2, .external_lex_state = 4},
  [21] = {.lex_state = 2, .external_lex_state = 5},
  [22] = {.lex_state = 2, .external_lex_state = 5},
  [23] = {.lex_state = 2, .external_lex_state = 4},
  [24] = {.lex_state = 2, .external_lex_state = 4},
  [25] = {.lex_state = 2, .external_lex_state = 4},
  [26] = {.lex_state = 2, .external_lex_state = 4},
  [27] = {.lex_state = 2, .external_lex_state = 5},
  [28] = {.lex_state = 2, .external_lex_state = 4},
  [29] = {.lex_state = 2, .external_lex_state = 5},
  [30] = {.lex_state = 2},
  [31] = {.lex_state = 2, .external_lex_state = 2},
  [32] = {.lex_state = 0, .external_lex_state = 3},
  [33] = {.lex_state = 0, .external_lex_state = 3},
  [34] = {.lex_state = 2},
  [35] = {.lex_state = 0, .external_lex_state = 3},
  [36] = {.lex_state = 0},
  [37] = {.lex_state = 0},
  [38] = {.lex_state = 0},
  [39] = {.lex_state = 0},
  [40] = {.lex_state = 0},
};

enum {
  ts_external_token_indent = 0,
  ts_external_token_dedent = 1,
  ts_external_token_newline = 2,
};

static const TSSymbol ts_external_scanner_symbol_map[EXTERNAL_TOKEN_COUNT] = {
  [ts_external_token_indent] = sym_indent,
  [ts_external_token_dedent] = sym_dedent,
  [ts_external_token_newline] = sym_newline,
};

static const bool ts_external_scanner_states[6][EXTERNAL_TOKEN_COUNT] = {
  [1] = {
    [ts_external_token_indent] = true,
    [ts_external_token_dedent] = true,
    [ts_external_token_newline] = true,
  },
  [2] = {
    [ts_external_token_dedent] = true,
  },
  [3] = {
    [ts_external_token_indent] = true,
    [ts_external_token_newline] = true,
  },
  [4] = {
    [ts_external_token_dedent] = true,
    [ts_external_token_newline] = true,
  },
  [5] = {
    [ts_external_token_newline] = true,
  },
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [anon_sym_AT] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [sym_indent] = ACTIONS(1),
    [sym_dedent] = ACTIONS(1),
    [sym_newline] = ACTIONS(1),
  },
  [1] = {
    [sym_document] = STATE(40),
    [aux_sym__block] = STATE(4),
    [sym_node] = STATE(16),
    [sym__bindable] = STATE(29),
    [sym_binding] = STATE(32),
    [sym_ref] = STATE(27),
    [anon_sym_AT] = ACTIONS(3),
    [sym_content] = ACTIONS(5),
  },
  [2] = {
    [aux_sym__block] = STATE(2),
    [sym_node] = STATE(16),
    [sym__bindable] = STATE(29),
    [sym_binding] = STATE(32),
    [sym_ref] = STATE(27),
    [ts_builtin_sym_end] = ACTIONS(7),
    [anon_sym_AT] = ACTIONS(9),
    [sym_content] = ACTIONS(12),
  },
  [3] = {
    [aux_sym__block] = STATE(5),
    [sym_node] = STATE(24),
    [sym__bindable] = STATE(25),
    [sym_binding] = STATE(33),
    [sym_ref] = STATE(26),
    [anon_sym_AT] = ACTIONS(15),
    [sym_content] = ACTIONS(17),
    [sym_dedent] = ACTIONS(19),
  },
  [4] = {
    [aux_sym__block] = STATE(2),
    [sym_node] = STATE(16),
    [sym__bindable] = STATE(29),
    [sym_binding] = STATE(32),
    [sym_ref] = STATE(27),
    [ts_builtin_sym_end] = ACTIONS(21),
    [anon_sym_AT] = ACTIONS(3),
    [sym_content] = ACTIONS(5),
  },
  [5] = {
    [aux_sym__block] = STATE(5),
    [sym_node] = STATE(24),
    [sym__bindable] = STATE(25),
    [sym_binding] = STATE(33),
    [sym_ref] = STATE(26),
    [anon_sym_AT] = ACTIONS(23),
    [sym_content] = ACTIONS(26),
    [sym_dedent] = ACTIONS(7),
  },
  [6] = {
    [aux_sym__block] = STATE(5),
    [sym_node] = STATE(24),
    [sym__bindable] = STATE(25),
    [sym_binding] = STATE(33),
    [sym_ref] = STATE(26),
    [anon_sym_AT] = ACTIONS(15),
    [sym_content] = ACTIONS(17),
    [sym_dedent] = ACTIONS(29),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 7,
    ACTIONS(15), 1,
      anon_sym_AT,
    ACTIONS(17), 1,
      sym_content,
    STATE(3), 1,
      aux_sym__block,
    STATE(24), 1,
      sym_node,
    STATE(25), 1,
      sym__bindable,
    STATE(26), 1,
      sym_ref,
    STATE(33), 1,
      sym_binding,
  [22] = 7,
    ACTIONS(15), 1,
      anon_sym_AT,
    ACTIONS(17), 1,
      sym_content,
    STATE(6), 1,
      aux_sym__block,
    STATE(24), 1,
      sym_node,
    STATE(25), 1,
      sym__bindable,
    STATE(26), 1,
      sym_ref,
    STATE(33), 1,
      sym_binding,
  [44] = 4,
    ACTIONS(35), 1,
      sym_indent,
    STATE(15), 1,
      sym_children,
    ACTIONS(31), 2,
      sym_newline,
      ts_builtin_sym_end,
    ACTIONS(33), 2,
      anon_sym_AT,
      sym_content,
  [59] = 4,
    ACTIONS(37), 1,
      sym_indent,
    STATE(28), 1,
      sym_children,
    ACTIONS(31), 2,
      sym_dedent,
      sym_newline,
    ACTIONS(33), 2,
      anon_sym_AT,
      sym_content,
  [74] = 3,
    ACTIONS(41), 1,
      anon_sym_COLON,
    ACTIONS(39), 2,
      anon_sym_AT,
      sym_content,
    ACTIONS(43), 2,
      sym_dedent,
      sym_newline,
  [86] = 3,
    ACTIONS(41), 1,
      anon_sym_COLON,
    ACTIONS(39), 2,
      anon_sym_AT,
      sym_content,
    ACTIONS(43), 2,
      sym_newline,
      ts_builtin_sym_end,
  [98] = 2,
    ACTIONS(45), 2,
      sym_newline,
      ts_builtin_sym_end,
    ACTIONS(47), 2,
      anon_sym_AT,
      sym_content,
  [107] = 2,
    ACTIONS(49), 2,
      anon_sym_AT,
      sym_content,
    ACTIONS(51), 2,
      sym_dedent,
      sym_newline,
  [116] = 2,
    ACTIONS(53), 2,
      sym_newline,
      ts_builtin_sym_end,
    ACTIONS(55), 2,
      anon_sym_AT,
      sym_content,
  [125] = 3,
    ACTIONS(57), 1,
      ts_builtin_sym_end,
    ACTIONS(61), 1,
      sym_newline,
    ACTIONS(59), 2,
      anon_sym_AT,
      sym_content,
  [136] = 3,
    ACTIONS(5), 1,
      sym_content,
    ACTIONS(63), 1,
      anon_sym_AT,
    STATE(13), 2,
      sym__bindable,
      sym_ref,
  [147] = 2,
    ACTIONS(49), 2,
      anon_sym_AT,
      sym_content,
    ACTIONS(51), 2,
      sym_newline,
      ts_builtin_sym_end,
  [156] = 2,
    ACTIONS(39), 2,
      anon_sym_AT,
      sym_content,
    ACTIONS(43), 2,
      sym_dedent,
      sym_newline,
  [165] = 2,
    ACTIONS(65), 2,
      anon_sym_AT,
      sym_content,
    ACTIONS(67), 2,
      sym_dedent,
      sym_newline,
  [174] = 2,
    ACTIONS(65), 2,
      anon_sym_AT,
      sym_content,
    ACTIONS(67), 2,
      sym_newline,
      ts_builtin_sym_end,
  [183] = 2,
    ACTIONS(39), 2,
      anon_sym_AT,
      sym_content,
    ACTIONS(43), 2,
      sym_newline,
      ts_builtin_sym_end,
  [192] = 2,
    ACTIONS(45), 2,
      sym_dedent,
      sym_newline,
    ACTIONS(47), 2,
      anon_sym_AT,
      sym_content,
  [201] = 3,
    ACTIONS(57), 1,
      sym_dedent,
    ACTIONS(69), 1,
      sym_newline,
    ACTIONS(59), 2,
      anon_sym_AT,
      sym_content,
  [212] = 2,
    ACTIONS(71), 2,
      anon_sym_AT,
      sym_content,
    ACTIONS(73), 2,
      sym_dedent,
      sym_newline,
  [221] = 2,
    ACTIONS(71), 2,
      anon_sym_AT,
      sym_content,
    ACTIONS(73), 2,
      sym_dedent,
      sym_newline,
  [230] = 2,
    ACTIONS(71), 2,
      anon_sym_AT,
      sym_content,
    ACTIONS(73), 2,
      sym_newline,
      ts_builtin_sym_end,
  [239] = 2,
    ACTIONS(53), 2,
      sym_dedent,
      sym_newline,
    ACTIONS(55), 2,
      anon_sym_AT,
      sym_content,
  [248] = 2,
    ACTIONS(71), 2,
      anon_sym_AT,
      sym_content,
    ACTIONS(73), 2,
      sym_newline,
      ts_builtin_sym_end,
  [257] = 3,
    ACTIONS(17), 1,
      sym_content,
    ACTIONS(75), 1,
      anon_sym_AT,
    STATE(23), 2,
      sym__bindable,
      sym_ref,
  [268] = 2,
    ACTIONS(7), 1,
      sym_dedent,
    ACTIONS(77), 2,
      anon_sym_AT,
      sym_content,
  [276] = 3,
    ACTIONS(35), 1,
      sym_indent,
    ACTIONS(79), 1,
      sym_newline,
    STATE(18), 1,
      sym_children,
  [286] = 3,
    ACTIONS(37), 1,
      sym_indent,
    ACTIONS(81), 1,
      sym_newline,
    STATE(14), 1,
      sym_children,
  [296] = 2,
    ACTIONS(7), 1,
      ts_builtin_sym_end,
    ACTIONS(77), 2,
      anon_sym_AT,
      sym_content,
  [304] = 1,
    ACTIONS(83), 2,
      sym_indent,
      sym_newline,
  [309] = 1,
    ACTIONS(85), 1,
      sym_identifier,
  [313] = 1,
    ACTIONS(87), 1,
      sym_identifier,
  [317] = 1,
    ACTIONS(89), 1,
      sym_identifier,
  [321] = 1,
    ACTIONS(91), 1,
      sym_identifier,
  [325] = 1,
    ACTIONS(93), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(7)] = 0,
  [SMALL_STATE(8)] = 22,
  [SMALL_STATE(9)] = 44,
  [SMALL_STATE(10)] = 59,
  [SMALL_STATE(11)] = 74,
  [SMALL_STATE(12)] = 86,
  [SMALL_STATE(13)] = 98,
  [SMALL_STATE(14)] = 107,
  [SMALL_STATE(15)] = 116,
  [SMALL_STATE(16)] = 125,
  [SMALL_STATE(17)] = 136,
  [SMALL_STATE(18)] = 147,
  [SMALL_STATE(19)] = 156,
  [SMALL_STATE(20)] = 165,
  [SMALL_STATE(21)] = 174,
  [SMALL_STATE(22)] = 183,
  [SMALL_STATE(23)] = 192,
  [SMALL_STATE(24)] = 201,
  [SMALL_STATE(25)] = 212,
  [SMALL_STATE(26)] = 221,
  [SMALL_STATE(27)] = 230,
  [SMALL_STATE(28)] = 239,
  [SMALL_STATE(29)] = 248,
  [SMALL_STATE(30)] = 257,
  [SMALL_STATE(31)] = 268,
  [SMALL_STATE(32)] = 276,
  [SMALL_STATE(33)] = 286,
  [SMALL_STATE(34)] = 296,
  [SMALL_STATE(35)] = 304,
  [SMALL_STATE(36)] = 309,
  [SMALL_STATE(37)] = 313,
  [SMALL_STATE(38)] = 317,
  [SMALL_STATE(39)] = 321,
  [SMALL_STATE(40)] = 325,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = false}}, SHIFT(36),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(9),
  [7] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__block, 2),
  [9] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__block, 2), SHIFT_REPEAT(36),
  [12] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__block, 2), SHIFT_REPEAT(9),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(38),
  [17] = {.entry = {.count = 1, .reusable = false}}, SHIFT(10),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [21] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_document, 1),
  [23] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__block, 2), SHIFT_REPEAT(38),
  [26] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__block, 2), SHIFT_REPEAT(10),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [31] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__bindable, 1),
  [33] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__bindable, 1),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [39] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_ref, 2),
  [41] = {.entry = {.count = 1, .reusable = false}}, SHIFT(35),
  [43] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ref, 2),
  [45] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_node, 3),
  [47] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_node, 3),
  [49] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_node, 2),
  [51] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_node, 2),
  [53] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__bindable, 2),
  [55] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym__bindable, 2),
  [57] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__block, 1),
  [59] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__block, 1),
  [61] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [63] = {.entry = {.count = 1, .reusable = false}}, SHIFT(37),
  [65] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_children, 3),
  [67] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_children, 3),
  [69] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [71] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_node, 1),
  [73] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_node, 1),
  [75] = {.entry = {.count = 1, .reusable = false}}, SHIFT(39),
  [77] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym__block, 2),
  [79] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [81] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [83] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binding, 3),
  [85] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [87] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [89] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [91] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [93] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
};

#ifdef __cplusplus
extern "C" {
#endif
void *tree_sitter_puddlejumper_external_scanner_create(void);
void tree_sitter_puddlejumper_external_scanner_destroy(void *);
bool tree_sitter_puddlejumper_external_scanner_scan(void *, TSLexer *, const bool *);
unsigned tree_sitter_puddlejumper_external_scanner_serialize(void *, char *);
void tree_sitter_puddlejumper_external_scanner_deserialize(void *, const char *, unsigned);

#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_puddlejumper(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .keyword_lex_fn = ts_lex_keywords,
    .keyword_capture_token = sym_identifier,
    .external_scanner = {
      &ts_external_scanner_states[0][0],
      ts_external_scanner_symbol_map,
      tree_sitter_puddlejumper_external_scanner_create,
      tree_sitter_puddlejumper_external_scanner_destroy,
      tree_sitter_puddlejumper_external_scanner_scan,
      tree_sitter_puddlejumper_external_scanner_serialize,
      tree_sitter_puddlejumper_external_scanner_deserialize,
    },
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
