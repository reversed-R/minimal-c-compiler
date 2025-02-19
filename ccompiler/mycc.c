#include <ctype.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef enum {
  TK_RESERVED,
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

typedef enum {
  ND_ADD,
  ND_SUB,
  ND_MUL,
  ND_DIV,
  ND_INT,
} NodeKind;

typedef struct Node Node;

struct Node {
  NodeKind kind;
  Node *lhs; // left hand side
  Node *rhs; // right hand side
  int val;
};

/* foward declarations *********/
Node *new_node(NodeKind kind, Node *lhs, Node *rhs);
Node *new_node_int(int val);
Node *expr();
void error_at(char *loc, char *fmt, ...);
bool consume(char op);
void expect(char op);
int expect_int();
Token *tokenize(char *p);
bool at_eof();
Token *new_token(TokenKind kind, Token *cur, char *str);
Node *mul();
Node *primary();
void gen(Node *node);
/******** foward declarations */

Node *new_node(NodeKind kind, Node *lhs, Node *rhs) {
  Node *node = calloc(1, sizeof(Node));
  node->kind = kind;
  node->lhs = lhs;
  node->rhs = rhs;
  return node;
}

Node *new_node_int(int val) {
  Node *node = calloc(1, sizeof(Node));
  node->kind = ND_INT;
  node->val = val;
  return node;
}

Node *expr() {
  Node *node = mul();

  for (;;) {
    if (consume('+'))
      node = new_node(ND_ADD, node, mul());
    else if (consume('-'))
      node = new_node(ND_SUB, node, mul());
    else
      return node;
  }
}

Node *mul() {
  Node *node = primary();

  for (;;) {
    if (consume('*'))
      node = new_node(ND_MUL, node, primary());
    else if (consume('/'))
      node = new_node(ND_DIV, node, primary());
    else
      return node;
  }
}

Node *primary() {
  if (consume('(')) {
    Node *node = expr();
    expect(')');
    return node;
  }

  return new_node_int(expect_int());
}

/* tokenizer ********/

Token *token;
char *user_input;

/* void error(char *fmt, ...) { */
/*   va_list ap; */
/*   va_start(ap, fmt); */
/*   vfprintf(stderr, fmt, ap); */
/*   fprintf(stderr, "\n"); */
/*   exit(1); */
/* } */

void error_at(char *loc, char *fmt, ...) {
  va_list ap;
  va_start(ap, fmt);

  int pos = loc - user_input;
  fprintf(stderr, "%s\n", user_input);
  fprintf(stderr, "%*s", pos, " "); // print `pos` time of whitespaces' '
  fprintf(stderr, "^ ");
  vfprintf(stderr, fmt, ap);
  fprintf(stderr, "\n");
  exit(1);
}

bool consume(char op) {
  if (token->kind != TK_RESERVED || token->str[0] != op)
    return false;

  token = token->next;
  return true;
}

void expect(char op) {
  if (token->kind != TK_RESERVED || token->str[0] != op)
    error_at(token->str, "'%c' not expected", op);
  token = token->next;
}

int expect_int() {
  if (token->kind != TK_INT)
    error_at(token->str, "integer value expected");
  int val = token->val;
  token = token->next;
  return val;
}

bool at_eof() { return token->kind == TK_EOF; }

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

    if (*p == '+' || *p == '-' || *p == '*' || *p == '/') {
      printf("[OP:%c]", *p);
      cur = new_token(TK_RESERVED, cur, p++);
      continue;
    }

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
    error_at(token->str, "failed to tokenize");
  }

  printf("\n<---- tokenize()\n");
  new_token(TK_EOF, cur, p);
  return head.next;
}
/********* tokenizer */

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
  token = tokenize(user_input);
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
