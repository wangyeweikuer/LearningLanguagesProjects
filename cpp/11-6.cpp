#include <iostream>
#include <typeinfo>
#include <cstring>
#include <string>
#include <string_view>

int main()
{
    char str[] = "strings";
    const std::type_info& x = typeid(str);

    std::cout << x.name() << '\n';
    
    std::string_view sv{"haha"};
    
    using namespace std::literals;
    auto y = "haha"sv;

    y.starts_with("");

    std::string str2{"hh"};

    str2.append()
    return 0;
}