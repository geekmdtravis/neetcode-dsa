#include <cassert>
#include <map>
#include <vector>

class Solution {
public:
  bool hasDuplicate(std::vector<int> &nums) {

    auto list = std::map<int, bool>();
    for (auto num : nums) {
      if (list.find(num) != list.end()) {
        return true;
      }

      auto res = list.insert({num, true});
    }
    return false;
  };
};

int main() {
  auto test1 = std::vector<int>{1, 2, 3, 1};
  auto test2 = std::vector<int>{1, 2, 3, 4};
  auto test3 = std::vector<int>{1, 1, 3, 4};
  auto test4 = std::vector<int>{1, 2, 2, 4};
  auto test5 = std::vector<int>{1, 2, 3, 3};

  auto solution = Solution();
  assert(solution.hasDuplicate(test1) == true);
  assert(solution.hasDuplicate(test2) == false);
  assert(solution.hasDuplicate(test3) == true);
  assert(solution.hasDuplicate(test4) == true);
  assert(solution.hasDuplicate(test5) == true);

  return 0;
}
