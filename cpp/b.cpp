#include <iostream>
#include <string>

class Name
{
private:
    std::string m_name;

public:
    Name(std::string name) : m_name{ name }
    {
    }

    Name(const Name& name) = default;
    Name& operator=(const Name& name) = default;

    //Name(Name&& name) = delete;
    Name& operator=(Name&& name) = delete;

    const std::string& get() { return m_name; }
};

Name getJoe()
{
    Name joe{ "Joe" };
    return joe; // error: Move constructor was deleted
}

int main()
{
    //Name n{ getJoe() };
    Name n= getJoe() ;

    std::cout << n.get() << '\n';

    return 0;
}
