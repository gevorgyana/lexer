enum class TokenType {
  Ident
};

struct token {
  int start, end;
  TokenType type;
};
