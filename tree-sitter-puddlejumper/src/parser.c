#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 48
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 17
#define ALIAS_COUNT 0
#define TOKEN_COUNT 10
#define EXTERNAL_TOKEN_COUNT 4
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 4
#define PRODUCTION_ID_COUNT 1

enum {
  aux_sym_node_token1 = 1,
  sym_identifier = 2,
  anon_sym_AT = 3,
  anon_sym_COLON = 4,
  sym_content = 5,
  sym_indent = 6,
  sym_dedent = 7,
  sym_newline = 8,
  sym_mega_newline = 9,
  sym_document = 10,
  aux_sym__stanza = 11,
  sym_block_node = 12,
  sym_node = 13,
  sym_children = 14,
  sym_binding = 15,
  aux_sym_block_node_repeat1 = 16,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [aux_sym_node_token1] = "node_token1",
  [sym_identifier] = "identifier",
  [anon_sym_AT] = "@",
  [anon_sym_COLON] = ":",
  [sym_content] = "content",
  [sym_indent] = "indent",
  [sym_dedent] = "dedent",
  [sym_newline] = "newline",
  [sym_mega_newline] = "mega_newline",
  [sym_document] = "document",
  [aux_sym__stanza] = "_stanza",
  [sym_block_node] = "block_node",
  [sym_node] = "node",
  [sym_children] = "children",
  [sym_binding] = "binding",
  [aux_sym_block_node_repeat1] = "block_node_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [aux_sym_node_token1] = aux_sym_node_token1,
  [sym_identifier] = sym_identifier,
  [anon_sym_AT] = anon_sym_AT,
  [anon_sym_COLON] = anon_sym_COLON,
  [sym_content] = sym_content,
  [sym_indent] = sym_indent,
  [sym_dedent] = sym_dedent,
  [sym_newline] = sym_newline,
  [sym_mega_newline] = sym_mega_newline,
  [sym_document] = sym_document,
  [aux_sym__stanza] = aux_sym__stanza,
  [sym_block_node] = sym_block_node,
  [sym_node] = sym_node,
  [sym_children] = sym_children,
  [sym_binding] = sym_binding,
  [aux_sym_block_node_repeat1] = aux_sym_block_node_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [aux_sym_node_token1] = {
    .visible = false,
    .named = false,
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
  [sym_mega_newline] = {
    .visible = true,
    .named = true,
  },
  [sym_document] = {
    .visible = true,
    .named = true,
  },
  [aux_sym__stanza] = {
    .visible = false,
    .named = false,
  },
  [sym_block_node] = {
    .visible = true,
    .named = true,
  },
  [sym_node] = {
    .visible = true,
    .named = true,
  },
  [sym_children] = {
    .visible = true,
    .named = true,
  },
  [sym_binding] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_block_node_repeat1] = {
    .visible = false,
    .named = false,
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
  [4] = 3,
  [5] = 5,
  [6] = 5,
  [7] = 3,
  [8] = 8,
  [9] = 9,
  [10] = 8,
  [11] = 9,
  [12] = 12,
  [13] = 9,
  [14] = 12,
  [15] = 15,
  [16] = 8,
  [17] = 12,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 19,
  [24] = 18,
  [25] = 20,
  [26] = 26,
  [27] = 27,
  [28] = 27,
  [29] = 26,
  [30] = 21,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 19,
  [35] = 18,
  [36] = 26,
  [37] = 37,
  [38] = 31,
  [39] = 39,
  [40] = 39,
  [41] = 41,
  [42] = 42,
  [43] = 33,
  [44] = 39,
  [45] = 42,
  [46] = 37,
  [47] = 47,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(3);
      if (lookahead == ':') ADVANCE(8);
      if (lookahead == '@') ADVANCE(6);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(5);
      END_STATE();
    case 1:
      if (lookahead == '\t' ||
          lookahead == '\r') ADVANCE(10);
      if (lookahead == '\n' ||
          lookahead == ' ') SKIP(1)
      if (lookahead != 0 &&
          lookahead != '@') ADVANCE(11);
      END_STATE();
    case 2:
      if (eof) ADVANCE(3);
      if (lookahead == '@') ADVANCE(6);
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
      ACCEPT_TOKEN(aux_sym_node_token1);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(4);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(sym_identifier);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(5);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(anon_sym_AT);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(anon_sym_AT);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(11);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(sym_content);
      if (lookahead == ' ') ADVANCE(9);
      if (lookahead == '@') ADVANCE(7);
      if (lookahead == '\t' ||
          lookahead == '\r') ADVANCE(9);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(11);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(sym_content);
      if (lookahead == ' ') ADVANCE(10);
      if (lookahead == '@') ADVANCE(11);
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

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0, .external_lex_state = 1},
  [1] = {.lex_state = 2},
  [2] = {.lex_state = 2},
  [3] = {.lex_state = 2, .external_lex_state = 2},
  [4] = {.lex_state = 2, .external_lex_state = 2},
  [5] = {.lex_state = 2, .external_lex_state = 2},
  [6] = {.lex_state = 2},
  [7] = {.lex_state = 2, .external_lex_state = 2},
  [8] = {.lex_state = 2, .external_lex_state = 3},
  [9] = {.lex_state = 2},
  [10] = {.lex_state = 2, .external_lex_state = 4},
  [11] = {.lex_state = 2},
  [12] = {.lex_state = 2, .external_lex_state = 3},
  [13] = {.lex_state = 2},
  [14] = {.lex_state = 2, .external_lex_state = 4},
  [15] = {.lex_state = 2},
  [16] = {.lex_state = 0, .external_lex_state = 5},
  [17] = {.lex_state = 0, .external_lex_state = 5},
  [18] = {.lex_state = 2, .external_lex_state = 2},
  [19] = {.lex_state = 2, .external_lex_state = 2},
  [20] = {.lex_state = 4, .external_lex_state = 6},
  [21] = {.lex_state = 2},
  [22] = {.lex_state = 0, .external_lex_state = 7},
  [23] = {.lex_state = 2},
  [24] = {.lex_state = 2},
  [25] = {.lex_state = 4, .external_lex_state = 6},
  [26] = {.lex_state = 2, .external_lex_state = 2},
  [27] = {.lex_state = 0, .external_lex_state = 7},
  [28] = {.lex_state = 0, .external_lex_state = 7},
  [29] = {.lex_state = 2},
  [30] = {.lex_state = 2, .external_lex_state = 2},
  [31] = {.lex_state = 0},
  [32] = {.lex_state = 0, .external_lex_state = 7},
  [33] = {.lex_state = 4, .external_lex_state = 6},
  [34] = {.lex_state = 0, .external_lex_state = 7},
  [35] = {.lex_state = 0, .external_lex_state = 7},
  [36] = {.lex_state = 0, .external_lex_state = 7},
  [37] = {.lex_state = 4, .external_lex_state = 6},
  [38] = {.lex_state = 0},
  [39] = {.lex_state = 1},
  [40] = {.lex_state = 1},
  [41] = {.lex_state = 4},
  [42] = {.lex_state = 0},
  [43] = {.lex_state = 4},
  [44] = {.lex_state = 1},
  [45] = {.lex_state = 0},
  [46] = {.lex_state = 4},
  [47] = {.lex_state = 0},
};

enum {
  ts_external_token_indent = 0,
  ts_external_token_dedent = 1,
  ts_external_token_newline = 2,
  ts_external_token_mega_newline = 3,
};

static const TSSymbol ts_external_scanner_symbol_map[EXTERNAL_TOKEN_COUNT] = {
  [ts_external_token_indent] = sym_indent,
  [ts_external_token_dedent] = sym_dedent,
  [ts_external_token_newline] = sym_newline,
  [ts_external_token_mega_newline] = sym_mega_newline,
};

static const bool ts_external_scanner_states[8][EXTERNAL_TOKEN_COUNT] = {
  [1] = {
    [ts_external_token_indent] = true,
    [ts_external_token_dedent] = true,
    [ts_external_token_newline] = true,
    [ts_external_token_mega_newline] = true,
  },
  [2] = {
    [ts_external_token_dedent] = true,
  },
  [3] = {
    [ts_external_token_indent] = true,
    [ts_external_token_dedent] = true,
    [ts_external_token_newline] = true,
  },
  [4] = {
    [ts_external_token_indent] = true,
    [ts_external_token_newline] = true,
  },
  [5] = {
    [ts_external_token_indent] = true,
    [ts_external_token_newline] = true,
    [ts_external_token_mega_newline] = true,
  },
  [6] = {
    [ts_external_token_newline] = true,
  },
  [7] = {
    [ts_external_token_newline] = true,
    [ts_external_token_mega_newline] = true,
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
    [sym_mega_newline] = ACTIONS(1),
  },
  [1] = {
    [sym_document] = STATE(47),
    [aux_sym__stanza] = STATE(2),
    [sym_block_node] = STATE(2),
    [sym_node] = STATE(2),
    [sym_binding] = STATE(20),
    [ts_builtin_sym_end] = ACTIONS(3),
    [anon_sym_AT] = ACTIONS(5),
    [sym_content] = ACTIONS(7),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 5,
    ACTIONS(5), 1,
      anon_sym_AT,
    ACTIONS(7), 1,
      sym_content,
    ACTIONS(9), 1,
      ts_builtin_sym_end,
    STATE(20), 1,
      sym_binding,
    STATE(6), 3,
      aux_sym__stanza,
      sym_block_node,
      sym_node,
  [18] = 5,
    ACTIONS(5), 1,
      anon_sym_AT,
    ACTIONS(11), 1,
      sym_content,
    ACTIONS(13), 1,
      sym_dedent,
    STATE(25), 1,
      sym_binding,
    STATE(5), 3,
      aux_sym__stanza,
      sym_block_node,
      sym_node,
  [36] = 5,
    ACTIONS(5), 1,
      anon_sym_AT,
    ACTIONS(11), 1,
      sym_content,
    ACTIONS(15), 1,
      sym_dedent,
    STATE(25), 1,
      sym_binding,
    STATE(5), 3,
      aux_sym__stanza,
      sym_block_node,
      sym_node,
  [54] = 5,
    ACTIONS(17), 1,
      anon_sym_AT,
    ACTIONS(20), 1,
      sym_content,
    ACTIONS(23), 1,
      sym_dedent,
    STATE(25), 1,
      sym_binding,
    STATE(5), 3,
      aux_sym__stanza,
      sym_block_node,
      sym_node,
  [72] = 5,
    ACTIONS(17), 1,
      anon_sym_AT,
    ACTIONS(23), 1,
      ts_builtin_sym_end,
    ACTIONS(25), 1,
      sym_content,
    STATE(20), 1,
      sym_binding,
    STATE(6), 3,
      aux_sym__stanza,
      sym_block_node,
      sym_node,
  [90] = 5,
    ACTIONS(5), 1,
      anon_sym_AT,
    ACTIONS(11), 1,
      sym_content,
    ACTIONS(28), 1,
      sym_dedent,
    STATE(25), 1,
      sym_binding,
    STATE(5), 3,
      aux_sym__stanza,
      sym_block_node,
      sym_node,
  [108] = 5,
    ACTIONS(32), 1,
      sym_indent,
    ACTIONS(34), 1,
      sym_dedent,
    ACTIONS(36), 1,
      sym_newline,
    STATE(26), 1,
      sym_children,
    ACTIONS(30), 2,
      anon_sym_AT,
      sym_content,
  [125] = 4,
    ACTIONS(5), 1,
      anon_sym_AT,
    ACTIONS(11), 1,
      sym_content,
    STATE(25), 1,
      sym_binding,
    STATE(3), 3,
      aux_sym__stanza,
      sym_block_node,
      sym_node,
  [140] = 5,
    ACTIONS(34), 1,
      ts_builtin_sym_end,
    ACTIONS(38), 1,
      sym_indent,
    ACTIONS(40), 1,
      sym_newline,
    STATE(29), 1,
      sym_children,
    ACTIONS(30), 2,
      anon_sym_AT,
      sym_content,
  [157] = 4,
    ACTIONS(5), 1,
      anon_sym_AT,
    ACTIONS(11), 1,
      sym_content,
    STATE(25), 1,
      sym_binding,
    STATE(4), 3,
      aux_sym__stanza,
      sym_block_node,
      sym_node,
  [172] = 5,
    ACTIONS(32), 1,
      sym_indent,
    ACTIONS(44), 1,
      sym_dedent,
    ACTIONS(46), 1,
      sym_newline,
    STATE(18), 1,
      sym_children,
    ACTIONS(42), 2,
      anon_sym_AT,
      sym_content,
  [189] = 4,
    ACTIONS(5), 1,
      anon_sym_AT,
    ACTIONS(11), 1,
      sym_content,
    STATE(25), 1,
      sym_binding,
    STATE(7), 3,
      aux_sym__stanza,
      sym_block_node,
      sym_node,
  [204] = 5,
    ACTIONS(38), 1,
      sym_indent,
    ACTIONS(44), 1,
      ts_builtin_sym_end,
    ACTIONS(48), 1,
      sym_newline,
    STATE(24), 1,
      sym_children,
    ACTIONS(42), 2,
      anon_sym_AT,
      sym_content,
  [221] = 4,
    ACTIONS(50), 1,
      anon_sym_AT,
    ACTIONS(52), 1,
      sym_content,
    STATE(32), 1,
      sym_node,
    STATE(41), 1,
      sym_binding,
  [234] = 3,
    ACTIONS(54), 1,
      sym_indent,
    STATE(36), 1,
      sym_children,
    ACTIONS(34), 2,
      sym_newline,
      sym_mega_newline,
  [245] = 3,
    ACTIONS(54), 1,
      sym_indent,
    STATE(35), 1,
      sym_children,
    ACTIONS(44), 2,
      sym_newline,
      sym_mega_newline,
  [256] = 2,
    ACTIONS(58), 1,
      sym_dedent,
    ACTIONS(56), 2,
      anon_sym_AT,
      sym_content,
  [264] = 2,
    ACTIONS(62), 1,
      sym_dedent,
    ACTIONS(60), 2,
      anon_sym_AT,
      sym_content,
  [272] = 3,
    ACTIONS(64), 1,
      aux_sym_node_token1,
    ACTIONS(66), 1,
      sym_newline,
    STATE(28), 1,
      aux_sym_block_node_repeat1,
  [282] = 2,
    ACTIONS(68), 1,
      ts_builtin_sym_end,
    ACTIONS(70), 2,
      anon_sym_AT,
      sym_content,
  [290] = 3,
    ACTIONS(72), 1,
      sym_newline,
    ACTIONS(75), 1,
      sym_mega_newline,
    STATE(22), 1,
      aux_sym_block_node_repeat1,
  [300] = 2,
    ACTIONS(62), 1,
      ts_builtin_sym_end,
    ACTIONS(60), 2,
      anon_sym_AT,
      sym_content,
  [308] = 2,
    ACTIONS(58), 1,
      ts_builtin_sym_end,
    ACTIONS(56), 2,
      anon_sym_AT,
      sym_content,
  [316] = 3,
    ACTIONS(66), 1,
      sym_newline,
    ACTIONS(77), 1,
      aux_sym_node_token1,
    STATE(27), 1,
      aux_sym_block_node_repeat1,
  [326] = 2,
    ACTIONS(81), 1,
      sym_dedent,
    ACTIONS(79), 2,
      anon_sym_AT,
      sym_content,
  [334] = 3,
    ACTIONS(66), 1,
      sym_newline,
    ACTIONS(83), 1,
      sym_mega_newline,
    STATE(22), 1,
      aux_sym_block_node_repeat1,
  [344] = 3,
    ACTIONS(66), 1,
      sym_newline,
    ACTIONS(85), 1,
      sym_mega_newline,
    STATE(22), 1,
      aux_sym_block_node_repeat1,
  [354] = 2,
    ACTIONS(81), 1,
      ts_builtin_sym_end,
    ACTIONS(79), 2,
      anon_sym_AT,
      sym_content,
  [362] = 2,
    ACTIONS(68), 1,
      sym_dedent,
    ACTIONS(70), 2,
      anon_sym_AT,
      sym_content,
  [370] = 2,
    ACTIONS(87), 1,
      sym_identifier,
    ACTIONS(89), 1,
      anon_sym_COLON,
  [377] = 1,
    ACTIONS(75), 2,
      sym_newline,
      sym_mega_newline,
  [382] = 1,
    ACTIONS(91), 2,
      sym_newline,
      aux_sym_node_token1,
  [387] = 1,
    ACTIONS(62), 2,
      sym_newline,
      sym_mega_newline,
  [392] = 1,
    ACTIONS(58), 2,
      sym_newline,
      sym_mega_newline,
  [397] = 1,
    ACTIONS(81), 2,
      sym_newline,
      sym_mega_newline,
  [402] = 1,
    ACTIONS(93), 2,
      sym_newline,
      aux_sym_node_token1,
  [407] = 2,
    ACTIONS(95), 1,
      sym_identifier,
    ACTIONS(97), 1,
      anon_sym_COLON,
  [414] = 1,
    ACTIONS(99), 1,
      sym_content,
  [418] = 1,
    ACTIONS(101), 1,
      sym_content,
  [422] = 1,
    ACTIONS(103), 1,
      aux_sym_node_token1,
  [426] = 1,
    ACTIONS(105), 1,
      anon_sym_COLON,
  [430] = 1,
    ACTIONS(91), 1,
      aux_sym_node_token1,
  [434] = 1,
    ACTIONS(107), 1,
      sym_content,
  [438] = 1,
    ACTIONS(109), 1,
      anon_sym_COLON,
  [442] = 1,
    ACTIONS(93), 1,
      aux_sym_node_token1,
  [446] = 1,
    ACTIONS(111), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 18,
  [SMALL_STATE(4)] = 36,
  [SMALL_STATE(5)] = 54,
  [SMALL_STATE(6)] = 72,
  [SMALL_STATE(7)] = 90,
  [SMALL_STATE(8)] = 108,
  [SMALL_STATE(9)] = 125,
  [SMALL_STATE(10)] = 140,
  [SMALL_STATE(11)] = 157,
  [SMALL_STATE(12)] = 172,
  [SMALL_STATE(13)] = 189,
  [SMALL_STATE(14)] = 204,
  [SMALL_STATE(15)] = 221,
  [SMALL_STATE(16)] = 234,
  [SMALL_STATE(17)] = 245,
  [SMALL_STATE(18)] = 256,
  [SMALL_STATE(19)] = 264,
  [SMALL_STATE(20)] = 272,
  [SMALL_STATE(21)] = 282,
  [SMALL_STATE(22)] = 290,
  [SMALL_STATE(23)] = 300,
  [SMALL_STATE(24)] = 308,
  [SMALL_STATE(25)] = 316,
  [SMALL_STATE(26)] = 326,
  [SMALL_STATE(27)] = 334,
  [SMALL_STATE(28)] = 344,
  [SMALL_STATE(29)] = 354,
  [SMALL_STATE(30)] = 362,
  [SMALL_STATE(31)] = 370,
  [SMALL_STATE(32)] = 377,
  [SMALL_STATE(33)] = 382,
  [SMALL_STATE(34)] = 387,
  [SMALL_STATE(35)] = 392,
  [SMALL_STATE(36)] = 397,
  [SMALL_STATE(37)] = 402,
  [SMALL_STATE(38)] = 407,
  [SMALL_STATE(39)] = 414,
  [SMALL_STATE(40)] = 418,
  [SMALL_STATE(41)] = 422,
  [SMALL_STATE(42)] = 426,
  [SMALL_STATE(43)] = 430,
  [SMALL_STATE(44)] = 434,
  [SMALL_STATE(45)] = 438,
  [SMALL_STATE(46)] = 442,
  [SMALL_STATE(47)] = 446,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_document, 0),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(38),
  [7] = {.entry = {.count = 1, .reusable = false}}, SHIFT(10),
  [9] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_document, 1),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(8),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [17] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__stanza, 2), SHIFT_REPEAT(38),
  [20] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__stanza, 2), SHIFT_REPEAT(8),
  [23] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__stanza, 2),
  [25] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym__stanza, 2), SHIFT_REPEAT(10),
  [28] = {.entry = {.count = 1, .reusable = true}}, SHIFT(23),
  [30] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_node, 1),
  [32] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [34] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_node, 1),
  [36] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [38] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [40] = {.entry = {.count = 1, .reusable = true}}, SHIFT(29),
  [42] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_node, 3),
  [44] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_node, 3),
  [46] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [48] = {.entry = {.count = 1, .reusable = true}}, SHIFT(24),
  [50] = {.entry = {.count = 1, .reusable = false}}, SHIFT(31),
  [52] = {.entry = {.count = 1, .reusable = false}}, SHIFT(16),
  [54] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [56] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_node, 4),
  [58] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_node, 4),
  [60] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_children, 3),
  [62] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_children, 3),
  [64] = {.entry = {.count = 1, .reusable = true}}, SHIFT(39),
  [66] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [68] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block_node, 3),
  [70] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block_node, 3),
  [72] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_block_node_repeat1, 2), SHIFT_REPEAT(15),
  [75] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_block_node_repeat1, 2),
  [77] = {.entry = {.count = 1, .reusable = true}}, SHIFT(40),
  [79] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_node, 2),
  [81] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_node, 2),
  [83] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [85] = {.entry = {.count = 1, .reusable = true}}, SHIFT(21),
  [87] = {.entry = {.count = 1, .reusable = true}}, SHIFT(42),
  [89] = {.entry = {.count = 1, .reusable = true}}, SHIFT(46),
  [91] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binding, 3),
  [93] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binding, 2),
  [95] = {.entry = {.count = 1, .reusable = true}}, SHIFT(45),
  [97] = {.entry = {.count = 1, .reusable = true}}, SHIFT(37),
  [99] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [101] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [103] = {.entry = {.count = 1, .reusable = true}}, SHIFT(44),
  [105] = {.entry = {.count = 1, .reusable = true}}, SHIFT(43),
  [107] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [109] = {.entry = {.count = 1, .reusable = true}}, SHIFT(33),
  [111] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
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
