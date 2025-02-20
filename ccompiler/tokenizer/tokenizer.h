#ifndef MYCC_TOKENIZER_H
#define MYCC_TOKENIZER_H

typedef enum {
  /* TK_RESERVED, */
  TK_PLUS,              // "+"
  TK_MINUS,             // "-"
  TK_SLASH,             // "/"
  TK_ASTERISC,          // "*"
  TK_OPEN_PARENTHESIS,  // "("
  TK_CLOSE_PARENTHESIS, // ")"
  TK_INT,
  TK_EOF,
} TokenKind;

typedef struct Token Token;

struct Token {
  TokenKind kind;
  Token *next;
  int val;
  char *str;
};

Token *tokenize(char *p);

#endif // !MYCC_TOKENIZER_H
