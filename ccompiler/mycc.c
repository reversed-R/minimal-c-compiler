#include <ctype.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "syntax_node/syntax_node.h"
#include "tokenizer/tokenizer.h"

/* foward declarations *********/
/* Node *new_node(NodeKind kind, Node *lhs, Node *rhs); */
/* Node *new_node_int(int val); */
/* Node *expr(); */
/* void error_at(char *loc, char *fmt, ...); */
/* bool consume(char op); */
/* void expect(char op); */
/* int expect_int(); */
/* Token *tokenize(char *p); */
/* bool at_eof(); */
/* Token *new_token(TokenKind kind, Token *cur, char *str); */
/* bool is_operand(TokenKind kind); */
/* bool consume_token(TokenKind kind); */
/* Node *mul(); */
/* Node *primary(); */
void gen(Node *node);
/******** foward declarations */
/* Token *token; */
char *user_input;

void gen(Node *node) {
  if (node->kind == ND_INT) {
    printf("  push %d\n", node->val);
    return;
  }

  gen(node->lhs);
  gen(node->rhs);

  printf("  pop rdi\n");
  printf("  pop rax\n");

  switch (node->kind) {
  case ND_ADD:
    printf("  add rax, rdi\n");
    break;
  case ND_SUB:
    printf("  sub rax, rdi\n");
    break;
  case ND_MUL:
    printf("  imul rax, rdi\n");
    break;
  case ND_DIV:
    printf("  cqo\n");
    printf("  idiv rdi\n");
    // `idiv _register_` means
    // divide (rdx rax(128bit)) by _register_
    // and set divided to rax, set remain to rdx
    // `cqo` means
    // extend and set rax(64bit) to (rdx rax(128bit))
    break;
  }

  printf("  push rax\n");
}

int main(int argc, char **argv) {
  if (argc != 2) {
    fprintf(stderr, "Invalid number of arguments\n");
    return 1;
  }

  user_input = argv[1];
  Token *token = tokenize(user_input);
  Node *node = expr();

  // foward part of assembly
  printf(".intel_syntax noprefix\n");
  printf(".global main\n");
  printf("main:\n");

  // generate asm with descending node from root
  gen(node);

  // result value must remain in stack-top
  printf("  pop rax\n");
  printf("  ret\n");
  return 0;
}
