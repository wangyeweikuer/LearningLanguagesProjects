#include <iostream>

class Base
{
private:
    /* data */
public:
    Base(/* args */);
    virtual ~Base();
};

Base::Base(/* args */)
{
    std::cout << "Base()\n";
}

Base::~Base()
{
    std::cout << "~Base()\n";
}

class Derived : public Base
{
private:
    /* data */
public:
    Derived(/* args */) {std::cout << "Derived()\n";}
    ~Derived() override { std::cout << "~Derived()\n"; }
};

int main(int argc, char const *argv[])
{
    Derived d1;
    std::cout << "1 ---\n";
    Derived *d2{new Derived()};
    std::cout << "2 ---\n";
    Base *d3{new Derived()};
    std::cout << "3 ---\n";
    delete d2;
    std::cout << "4 ---\n";
    delete d3;
    std::cout << "5 ---\n";
    return 0;
}
