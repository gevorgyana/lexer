#include "include/common.h"

#include <string>
#include <functional>
#include <vector>
#include <iostream>

std::vector<token> tokenize(std::string in);

std::string token_stream_to_string(std::vector<token> token_stream) {
  std::string string;
  for (auto& token : token_stream) {
    string += (token.type == TokenType::Ident);
  }
  return string;
}

// todo use gtest instead

int verify(std::string in, std::vector<token> expected) {
  auto tokens = tokenize(in);
  if (tokens.size() != expected.size()) {
    std::cerr << "FAILED " << '\n';
    std::cerr << " Lhs: " << in << ", Rhs: " << token_stream_to_string(expected)
              << '\n';
  }
  return 0;
}

int main() {
  return 0;
}
