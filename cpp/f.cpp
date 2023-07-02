#include <iostream>
#include <vector>

std::vector<int> EMPTY_LIST(0);

std::vector<int>& getChildren(int idx, std::vector<std::vector<int>>& children_) {
	return children_[idx];
    //auto it = children_[idx];
    //if (!it.empty()) {
    //    return it;
    //} else {
    //    // 这里最好在children中放入一个空的list
    //    return EMPTY_LIST;
    //}
}

int main()
{
  //uint64_t x = 1;
  //for(size_t i =0;i<64;i++)
  //std::cout << i << " -> " << (static_cast<uint64_t>(1u) << i) << std::endl;
  std::vector<std::vector<int>> vec;
  vec.push_back({1,2});
  vec.push_back({});
  vec.push_back({3});
  auto x = getChildren(0, vec);
  std::cout << x.size() << std::endl;
  auto y = getChildren(1, vec);
  std::cout << y.size() << std::endl;
  //auto z = getChildren(2, vec);
  //std::cout << z.size() << std::endl;
  auto z = getChildren(300, vec);
  std::cout << z.size() << std::endl;
	return 0;
}

