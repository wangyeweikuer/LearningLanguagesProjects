#include <iostream>
#include <string>

using namespace std;
void printValue(std::string &y) // type changed to std::string&
{
    std::cout << y << '\n';
    y.append("xx");
} // y is destroyed here

std::string ToTruncatedUpperCase(const char *buffer, size_t truncatedSize)
{
    assert(buffer && "buffer should be not null");
    assert(truncatedSize >= 0 && "truncatedSize should be non-negative.");
    std::string res(truncatedSize, '\0');
    std::cerr << " ------- from char: " << buffer << " and truncatedSize: " << truncatedSize << std::endl;
    for (size_t i = 0; i < truncatedSize; i++)
    {
        char x = buffer[i];
                    res[i] = (islower(x) ? toupper(x) : x);
    }
    std::cerr << " ------- upperstring " << res << std::endl;
    return res;
}

int main()
{
    std::string x = "Hello, world!" ;

    cout << ToTruncatedUpperCase(x.data(), x.size()) << '\n';

    std::cout << static_cast<char>(toupper('A')) << static_cast<char>(tolower('a')) << std::endl;

    printValue(x); // x is now passed by reference into reference parameter y (inexpensive)

    std::cout << x << '\n';
    std::string s(2, 'a');
    std::cout << s << '\n';
    std::cout << s.data() << '\n';
    std::cout << sizeof(s.data()) / sizeof(s.data()[0]) << '\n';
    return 0;
}
