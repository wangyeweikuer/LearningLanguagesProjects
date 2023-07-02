#include <iostream>

using namespace std;

class A
{
public:
    A()
    {
        m_iNum = 1;
    }

public:
    int m_iNum;
};

void foo()
{
    A a0;
    const A &a1 = a0;
    A a2 = const_cast<A &>(a1); // 常量引用转为非常量引用

    a2.m_iNum = 200; // fine

    cout << a0.m_iNum << a1.m_iNum << a2.m_iNum << endl; // 1 1 200
}
int main()
{
    foo();
}