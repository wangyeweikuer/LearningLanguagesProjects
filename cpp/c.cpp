#include <iostream>
#include <vector>
#include <set>
#include <string>

using namespace std;

int main(){
  std::set<std::string> fs {"a","b","c"};
  std::cout << (fs.find("aia") != fs.end()) << endl;
  return 0;
}

