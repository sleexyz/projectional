#include "tree_sitter/parser.h"
#include <cassert>
#include <cstring>
#include <cwctype>
#include <stdio.h>
#include <vector>
namespace {

using std::vector;

enum TokenType {
  INDENT,
  DEDENT,
  NEWLINE,
};

struct Scanner {
  vector<uint16_t> indent_length_stack;

  Scanner() { deserialize(NULL, 0); }

  unsigned serialize(char *buffer) {
    size_t i = 0;

    vector<uint16_t>::iterator iter = indent_length_stack.begin() + 1,
                               end = indent_length_stack.end();

    for (; iter != end && i < TREE_SITTER_SERIALIZATION_BUFFER_SIZE; ++iter) {
      buffer[i++] = *iter;
    }

    return i;
  }

  void deserialize(const char *buffer, unsigned length) {
    indent_length_stack.clear();
    indent_length_stack.push_back(0);

    if (length > 0) {
      size_t i = 0;
      for (; i < length; i++) {
        indent_length_stack.push_back(buffer[i]);
      }
    }
  }

  void skip(TSLexer *lexer) {
    // Do not skip so that we can consume the whitespace.
    lexer->advance(lexer, /* skip */ false);
  }

  bool scan(TSLexer *lexer, const bool *valid_symbols) {
    bool found_end_of_line = false;
    uint32_t indent_length = 0;
    for (;;) {
      if (lexer->lookahead == '\n') {
        found_end_of_line = true;
        indent_length = 0;
        skip(lexer);
      } else if (lexer->lookahead == ' ') {
        indent_length++;
        skip(lexer);
      } else if (lexer->lookahead == '\t') {
        indent_length += 8;
        skip(lexer);
      } else if (lexer->lookahead == '\r') {
        indent_length = 0;
        skip(lexer);
      } else if (lexer->lookahead == '\f') {
        indent_length = 0;
        skip(lexer);
      } else if (lexer->lookahead == 0) {
        indent_length = 0;
        found_end_of_line = true;
        break;
      } else {
        break;
      }
    }

    if (found_end_of_line) {
      uint16_t current_indent_length =
          indent_length_stack.empty() ? 0 : indent_length_stack.back();

      if (!indent_length_stack.empty()) {
        if (valid_symbols[INDENT] && indent_length > current_indent_length) {
          indent_length_stack.push_back(indent_length);
          lexer->result_symbol = INDENT;
          return true;
        }

        if ((valid_symbols[DEDENT] || !valid_symbols[NEWLINE]) &&
            indent_length < current_indent_length) {
          indent_length_stack.pop_back();
          lexer->result_symbol = DEDENT;
          return true;
        }
      }

      if (valid_symbols[NEWLINE] &&
          (current_indent_length == indent_length ||
           !valid_symbols[INDENT] && !valid_symbols[DEDENT])) {
        lexer->result_symbol = NEWLINE;
        return true;
      }
    }

    return false;
  }
};

} // namespace

extern "C" {

void *tree_sitter_puddlejumper_external_scanner_create() {
  return new Scanner();
}

bool tree_sitter_puddlejumper_external_scanner_scan(void *payload,
                                                    TSLexer *lexer,
                                                    const bool *valid_symbols) {
  Scanner *scanner = static_cast<Scanner *>(payload);
  return scanner->scan(lexer, valid_symbols);
}

unsigned tree_sitter_puddlejumper_external_scanner_serialize(void *payload,
                                                             char *buffer) {
  Scanner *scanner = static_cast<Scanner *>(payload);
  return scanner->serialize(buffer);
}

void tree_sitter_puddlejumper_external_scanner_deserialize(void *payload,
                                                           const char *buffer,
                                                           unsigned length) {
  Scanner *scanner = static_cast<Scanner *>(payload);
  scanner->deserialize(buffer, length);
}

void tree_sitter_puddlejumper_external_scanner_destroy(void *payload) {
  Scanner *scanner = static_cast<Scanner *>(payload);
  delete scanner;
}
}