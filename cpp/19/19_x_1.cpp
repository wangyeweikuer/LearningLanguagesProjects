#include <iostream>

template <typename T>
class Pair1
{
private:
    T first_;
    T second_;

public:
    Pair1(T f1, T s2) : first_{f1}, second_{s2} {};
    T first() const { return first_; }
    T second() const { return second_; }
};

int main()
{
    Pair1<int> p1{5, 8};
    std::cout << "Pair: " << p1.first() << ' ' << p1.second() << '\n';

    const Pair1<double> p2{2.3, 4.5};
    std::cout << "Pair: " << p2.first() << ' ' << p2.second() << '\n';

    return 0;
}
