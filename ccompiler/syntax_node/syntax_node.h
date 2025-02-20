#ifndef MYCC_SYNTAX_NODE_H
#define MYCC_SYNTAX_NODE_H

#include "../tokenizer/tokenizer.h"

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

Node *generate_node(Token *token);

#endif // !MYCC_SYNTAX_NODE_H
