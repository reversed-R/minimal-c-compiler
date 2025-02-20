#include "syntax_node.h"
#include "../tokenizer/tokenizer.h"
#include <ctype.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* foward declarations *********/
Node *new_node(Token *token, NodeKind kind, Node *lhs, Node *rhs);
Node *new_node_int(Token *token);
Node *expr(Token *token);
/* void error_at(char *loc, char *fmt, ...); */
/* bool consume(char op); */
void expect(char op);
int expect_int();
bool is_operand(TokenKind kind);
bool consume_token(TokenKind kind);
Node *mul(Token *token);
Node *primary(Token *token);
Token *consume(Token *token);
void proceed(Token *token);
/******** foward declarations */

Node *new_node(Token *token, NodeKind kind, Node *lhs, Node *rhs) {
  Node *node = calloc(1, sizeof(Node));
  node->kind = kind;
  node->lhs = lhs;
  node->rhs = rhs;
  return node;
}

Node *new_node_int(Token *token) {
  Node *node = calloc(1, sizeof(Node));
  node->kind = ND_INT;
  node->val = token->val;
  proceed(token);
  return node;
}

Node *expr(Token *token) {
  Node *node = mul(token);

  for (;;) {
    if (consume(TK_PLUS))
      node = new_node(token, ND_ADD, node, mul(token));
    else if (consume_token(TK_MINUS))
      node = new_node(token, ND_SUB, node, mul(token));
    else
      return node;
  }
}

Node *mul(Token *token) {
  Node *node = primary(token);

  for (;;) {
    if (consume_token(TK_ASTERISC))
      node = new_node(token, ND_MUL, node, primary(token));
    else if (consume_token(TK_SLASH))
      node = new_node(token, ND_DIV, node, primary(token));
    else
      return node;
  }
}

Node *primary(Token *token) {
  if (consume_token(TK_OPEN_PARENTHESIS)) {
    Node *node = expr(token);
    expect(')');
    return node;
  }

  return new_node_int(expect_int());
}

bool is_operand(TokenKind kind) {
  return kind == TK_PLUS || kind == TK_MINUS || kind == TK_SLASH ||
         kind == TK_ASTERISC;
}

/* void error_at(char *loc, char *fmt, ...) { */
/*   va_list ap; */
/*   va_start(ap, fmt); */
/**/
/*   int pos = loc - user_input; */
/*   fprintf(stderr, "%s\n", user_input); */
/*   fprintf(stderr, "%*s", pos, " "); // print `pos` time of whitespaces' ' */
/*   fprintf(stderr, "^ "); */
/*   vfprintf(stderr, fmt, ap); */
/*   fprintf(stderr, "\n"); */
/*   exit(1); */
/* } */

/* bool consume(char op) { */
/*   if (is_operand(token->kind) || token->str[0] != op) */
/*     return false; */
/**/
/*   token = token->next; */
/*   return true; */
/* } */

/* bool consume_token(TokenKind kind) { */
/*   if (token->kind != kind) */
/*     return false; */
/**/
/*   token = token->next; */
/*   return true; */
/* } */

/* void expect(char op) { */
/*   if (is_operand(token->kind) || token->str[0] != op) */
/*     ; */
/* error_at(token->str, "'%c' not expected", op); */
/*   token = token->next; */
/* } */

Token *consume(Token *token) { return token->next; }
void proceed(Token *token) { token = token->next; }

/* int expect_int() { */
/*   if (token->kind != TK_INT) */
/*     error_at(token->str, "integer value expected"); */
/*   int val = token->val; */
/*   token = token->next; */
/*   return val; */
/* } */

Node *generate_node(Token *token) {
  Token *head = token;
  expr(head);
}
