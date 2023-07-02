#include <iostream>

using namespace std;

struct fraction
{
    int numerator{1};
    int denominator{0};
};

fraction read(){
    fraction f;
    cout << "Enter a value for the numerator: ";
    cin >> f.numerator;
    cout << "Enter a value for the denominator: ";
    cin >> f.denominator;
    return f;
}

fraction multiply(const fraction& f1, const fraction& f2){
    fraction f3;
    f3.denominator = f1.denominator * f2.denominator;
    f3.numerator = f1.numerator * f2.numerator;
    return f3;
}

int main()
{
    fraction f1 = read();
    fraction f2 = read();

    fraction f3 = multiply(f1,f2);
    cout << "Your fractions multiplied together: " << f3.numerator << "/" << f3.denominator << endl;
    
    return 0;
}

// struct Foo1
// {
//     short a{};
//     short qq{}; // note: qq is defined here
//     int b{};
//     double c{};
// };

// struct Foo2
// {
//     short a{};
//     int b{};
//     double c{};
//     short qq{}; // note: qq is defined here
// };

// int main()
// {
//     std::cout << "The size of Foo1 is " << sizeof(Foo1) << '\n';
//     std::cout << "The size of Foo2 is " << sizeof(Foo2) << '\n';

//     return 0;
// }
