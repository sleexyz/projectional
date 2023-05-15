#include <tree_sitter/parser.h>

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 20
#define LARGE_STATE_COUNT 4
#define SYMBOL_COUNT 9
#define ALIAS_COUNT 0
#define TOKEN_COUNT 5
#define EXTERNAL_TOKEN_COUNT 3
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 3
#define PRODUCTION_ID_COUNT 1

enum {
  sym_content = 1,
  sym__indent = 2,
  sym__dedent = 3,
  sym_newline = 4,
  sym_document = 5,
  aux_sym__block = 6,
  sym_node = 7,
  sym_children = 8,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_content] = "content",
  [sym__indent] = "_indent",
  [sym__dedent] = "_dedent",
  [sym_newline] = "newline",
  [sym_document] = "document",
  [aux_sym__block] = "_block",
  [sym_node] = "node",
  [sym_children] = "children",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_content] = sym_content,
  [sym__indent] = sym__indent,
  [sym__dedent] = sym__dedent,
  [sym_newline] = sym_newline,
  [sym_document] = sym_document,
  [aux_sym__block] = aux_sym__block,
  [sym_node] = sym_node,
  [sym_children] = sym_children,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_content] = {
    .visible = true,
    .named = true,
  },
  [sym__indent] = {
    .visible = false,
    .named = true,
  },
  [sym__dedent] = {
    .visible = false,
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
  [3] = 2,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 5,
  [8] = 6,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 9,
  [14] = 11,
  [15] = 12,
  [16] = 10,
  [17] = 17,
  [18] = 17,
  [19] = 19,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(1);
      if (lookahead == '\n') SKIP(0)
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(2);
      if (lookahead != 0) ADVANCE(3);
      END_STATE();
    case 1:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 2:
      ACCEPT_TOKEN(sym_content);
      if (lookahead == '\t' ||
          lookahead == '\r' ||
          lookahead == ' ') ADVANCE(2);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(3);
      END_STATE();
    case 3:
      ACCEPT_TOKEN(sym_content);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(3);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0, .external_lex_state = 1},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0, .external_lex_state = 2},
  [3] = {.lex_state = 0, .external_lex_state = 1},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0, .external_lex_state = 3},
  [7] = {.lex_state = 0, .external_lex_state = 3},
  [8] = {.lex_state = 0, .external_lex_state = 3},
  [9] = {.lex_state = 0, .external_lex_state = 4},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0, .external_lex_state = 4},
  [12] = {.lex_state = 0, .external_lex_state = 4},
  [13] = {.lex_state = 0, .external_lex_state = 5},
  [14] = {.lex_state = 0, .external_lex_state = 5},
  [15] = {.lex_state = 0, .external_lex_state = 5},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 0, .external_lex_state = 3},
  [19] = {.lex_state = 0},
};

enum {
  ts_external_token__indent = 0,
  ts_external_token__dedent = 1,
  ts_external_token_newline = 2,
};

static const TSSymbol ts_external_scanner_symbol_map[EXTERNAL_TOKEN_COUNT] = {
  [ts_external_token__indent] = sym__indent,
  [ts_external_token__dedent] = sym__dedent,
  [ts_external_token_newline] = sym_newline,
};

static const bool ts_external_scanner_states[6][EXTERNAL_TOKEN_COUNT] = {
  [1] = {
    [ts_external_token__indent] = true,
    [ts_external_token__dedent] = true,
    [ts_external_token_newline] = true,
  },
  [2] = {
    [ts_external_token__indent] = true,
    [ts_external_token_newline] = true,
  },
  [3] = {
    [ts_external_token__dedent] = true,
  },
  [4] = {
    [ts_external_token_newline] = true,
  },
  [5] = {
    [ts_external_token__dedent] = true,
    [ts_external_token_newline] = true,
  },
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_content] = ACTIONS(1),
    [sym__indent] = ACTIONS(1),
    [sym__dedent] = ACTIONS(1),
    [sym_newline] = ACTIONS(1),
  },
  [1] = {
    [sym_document] = STATE(19),
    [aux_sym__block] = STATE(4),
    [sym_node] = STATE(9),
    [sym_content] = ACTIONS(3),
  },
  [2] = {
    [sym_children] = STATE(11),
    [ts_builtin_sym_end] = ACTIONS(5),
    [sym_content] = ACTIONS(5),
    [sym__indent] = ACTIONS(7),
    [sym_newline] = ACTIONS(5),
  },
  [3] = {
    [sym_children] = STATE(14),
    [sym_content] = ACTIONS(5),
    [sym__indent] = ACTIONS(9),
    [sym__dedent] = ACTIONS(5),
    [sym_newline] = ACTIONS(5),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 4,
    ACTIONS(3), 1,
      sym_content,
    ACTIONS(11), 1,
      ts_builtin_sym_end,
    STATE(5), 1,
      aux_sym__block,
    STATE(9), 1,
      sym_node,
  [13] = 4,
    ACTIONS(13), 1,
      ts_builtin_sym_end,
    ACTIONS(15), 1,
      sym_content,
    STATE(5), 1,
      aux_sym__block,
    STATE(9), 1,
      sym_node,
  [26] = 4,
    ACTIONS(18), 1,
      sym_content,
    ACTIONS(20), 1,
      sym__dedent,
    STATE(7), 1,
      aux_sym__block,
    STATE(13), 1,
      sym_node,
  [39] = 4,
    ACTIONS(13), 1,
      sym__dedent,
    ACTIONS(22), 1,
      sym_content,
    STATE(7), 1,
      aux_sym__block,
    STATE(13), 1,
      sym_node,
  [52] = 4,
    ACTIONS(18), 1,
      sym_content,
    ACTIONS(25), 1,
      sym__dedent,
    STATE(7), 1,
      aux_sym__block,
    STATE(13), 1,
      sym_node,
  [65] = 2,
    ACTIONS(29), 1,
      sym_newline,
    ACTIONS(27), 2,
      ts_builtin_sym_end,
      sym_content,
  [73] = 3,
    ACTIONS(18), 1,
      sym_content,
    STATE(6), 1,
      aux_sym__block,
    STATE(13), 1,
      sym_node,
  [83] = 1,
    ACTIONS(31), 3,
      sym_newline,
      ts_builtin_sym_end,
      sym_content,
  [89] = 1,
    ACTIONS(33), 3,
      sym_newline,
      ts_builtin_sym_end,
      sym_content,
  [95] = 2,
    ACTIONS(35), 1,
      sym_newline,
    ACTIONS(27), 2,
      sym__dedent,
      sym_content,
  [103] = 1,
    ACTIONS(31), 3,
      sym__dedent,
      sym_newline,
      sym_content,
  [109] = 1,
    ACTIONS(33), 3,
      sym__dedent,
      sym_newline,
      sym_content,
  [115] = 3,
    ACTIONS(18), 1,
      sym_content,
    STATE(8), 1,
      aux_sym__block,
    STATE(13), 1,
      sym_node,
  [125] = 1,
    ACTIONS(13), 2,
      ts_builtin_sym_end,
      sym_content,
  [130] = 1,
    ACTIONS(13), 2,
      sym__dedent,
      sym_content,
  [135] = 1,
    ACTIONS(37), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(4)] = 0,
  [SMALL_STATE(5)] = 13,
  [SMALL_STATE(6)] = 26,
  [SMALL_STATE(7)] = 39,
  [SMALL_STATE(8)] = 52,
  [SMALL_STATE(9)] = 65,
  [SMALL_STATE(10)] = 73,
  [SMALL_STATE(11)] = 83,
  [SMALL_STATE(12)] = 89,
  [SMALL_STATE(13)] = 95,
  [SMALL_STATE(14)] = 103,
  [SMALL_STATE(15)] = 109,
  [SMALL_STATE(16)] = 115,
  [SMALL_STATE(17)] = 125,
  [SMALL_STATE(18)] = 130,
  [SMALL_STATE(19)] = 135,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_node, 1),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [11] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_document, 1),
  [13] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__block, 2),
  [15] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__block, 2), SHIFT_REPEAT(2),
  [18] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [20] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [22] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym__block, 2), SHIFT_REPEAT(3),
  [25] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [27] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym__block, 1),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [31] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_node, 2),
  [33] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_children, 3),
  [35] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [37] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
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
