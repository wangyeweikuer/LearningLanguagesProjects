#include <iostream>

enum Color{
    blue,
    white,
};

enum Color2{
    blue2,
    white2,
};

int main(){



    Color c1{Color::blue};
    Color2 c2{Color2::white2};

    std::cout << c1<<","<<c2<<std::endl;

    return 0;
}