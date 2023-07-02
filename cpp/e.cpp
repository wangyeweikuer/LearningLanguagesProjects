#include <string>
#include <map>
#include <vector>
#include <iostream>
using namespace std;

std::map<int,int> func()
{
  std::map<int, int> map;
  map[1] = 2;
  map[3] = 4;
  return map;
}

int main()
{
    std::map<int, int> a = func();
    std::map<int, int> b = std::move(func());
		std::vector<int&> v;

    int x = 10;
    int y = 20;
    v.push_back(x); 
    return 0;
}
