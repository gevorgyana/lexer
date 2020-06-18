#include "../include/ds.h"

#include <string>
#include <functional>
#include <vector>

// todo use gtest instead
#include "assert.h"

int dummy_test(std::string in, std::vector<token> expected) {
  assert(in.size() == expected.size());
  return 0;
}

int main() {
  std::vector<std::function<int(std::string, std::vector<token>)>> scenarios;
  scenarios.push_back(&dummy_test);
  return 0;
}
