#include <iostream>

using namespace std;
class A
{
  public:
   A() { cout << "1\n"; }
   A(A& a) { cout << "2\n";}
   A& operator=(A& a) { cout << "3\n"; return *this;}
};

int main()
{
  A a;
  A a1(a);
  A a2 = a;
  A a3;
   a3 = a;
return 0;
}


