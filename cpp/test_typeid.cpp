#include <iostream>
#include <typeinfo>
#include <string>

using namespace std;
  static std::string MODES[] = {"NONE", "TRANSLIT", "IGNORE"};

int add(int x, int y)
{
  return (x + y);
}

void _repeat(string *results, const string &str, const int &count)
{
  for (int i = 0; i < count; i++)
  {
    results[i] = str;
  }
}

int main()
{
  cout << sizeof(MODES) << endl;
  cout << sizeof(MODES[0]) << endl;
  cout << sizeof(MODES[1]) << endl;
  cout << sizeof(MODES[2]) << endl;
  std::string mode = "NONE";
  std::cerr << "aaaaaaaaa s-8-1 " << mode << "\n";
  int32_t i = 0;
  int32_t len = sizeof(MODES) / sizeof(MODES[0]);
  std::cerr << "aaaaaaaaa s-8-2 " << len << "\n";
  for (; i < len; i++)
  {
    std::cerr << "aaaaaaaaa s-8-3 " << MODES[i] << "\n";
    if (MODES[i] != mode)
    {
      std::cerr << "aaaaaaaaa s-8-3-1 " << MODES[i] << "\n";
      break;
    }
  }
  cout << len << endl;
  cout << std::string("1234", 3) << endl;

  // auto a = "xxx";
  // cout << strlen(a) << endl;
  cout << typeid(5LU + 3.4f).name() << endl;
  cout << int(10.5) << endl;
  cout << (int)20.3 << endl;

  cout << sizeof(size_t) << endl;
  string charsets[10];
  _repeat(charsets, "gb2312", 10);

  for (size_t i = 0; i < 10; i++)
  {
    cout << charsets[i] << endl;
  }
  cout << sizeof(charsets) / sizeof(charsets[0]) << endl;

  cout << charsets->length() << endl;
  return 0;
}
