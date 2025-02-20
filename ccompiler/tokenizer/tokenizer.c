#include "tokenizer.h"

#include <ctype.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
/* #include <string.h> */

/* foward declarations *********/
bool at_eof(Token *token);
Token *new_token(TokenKind kind, Token *cur, char *str);
/******** foward declarations */

bool at_eof(Token *token) { return token->kind == TK_EOF; }

Token *new_token(TokenKind kind, Token *cur, char *str) {
  Token *tok = calloc(1, sizeof(Token));
  tok->kind = kind;
  tok->str = str;
  cur->next = tok;
  return tok;
}

Token *tokenize(char *p) {
  printf("tokenize() ---->\n");
  Token head;
  head.next = NULL;
  Token *cur = &head;

  while (*p) {
    if (isspace(*p)) {
      printf("[SP]");
      p++;
      continue;
    }

    if (*p == '+') {
      printf("[PLUS]");
      cur = new_token(TK_PLUS, cur, p++);
      continue;
    }
    if (*p == '-') {
      printf("[MINUS]");
      cur = new_token(TK_MINUS, cur, p++);
      continue;
    }
    if (*p == '/') {
      printf("[SLASH]");
      cur = new_token(TK_SLASH, cur, p++);
      continue;
    }
    if (*p == '*') {
      printf("[ASTERISC]");
      cur = new_token(TK_ASTERISC, cur, p++);
      continue;
    }

    /* if (*p == '+' || *p == '-' || *p == '*' || *p == '/') { */
    /*   printf("[OP:%c]", *p); */
    /*   cur = new_token(TK_RESERVED, cur, p++); */
    /*   continue; */
    /* } */

    if (*p == '(') {
      printf("[OPEN_PARE]");
      cur = new_token(TK_OPEN_PARENTHESIS, cur, p++);
      continue;
    }

    if (*p == ')') {
      printf("[CLOSE_PARE]");
      cur = new_token(TK_CLOSE_PARENTHESIS, cur, p++);
      continue;
    }

    if (isdigit(*p)) {
      printf("[DIGIT:%c]", *p);
      cur = new_token(TK_INT, cur, p);
      cur->val = strtol(p, &p, 10);
      continue;
    }

    printf("ERROR\n");
    /* error_at(token->str, "failed to tokenize"); */
  }

  printf("\n<---- tokenize()\n");
  new_token(TK_EOF, cur, p);
  return head.next;
}
