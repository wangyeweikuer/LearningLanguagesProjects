#include <iostream>
#include <set>
#include <string>
#include <assert.h>

using namespace std;

class Reference
{
private:
    std::string *_from;
    std::string *_name;

public:
    Reference(std::string *from, std::string *name) : _from(from), _name(name) {}
    std::string &from() const
    {
        return *_from;
    }
    std::string &name() const
    {
        return *_name;
    }
    friend std::ostream &operator<<(std::ostream &out, const Reference &x)
    {
        out << "[from:" << (*x._from) << ", name:" << (*x._name) << "\n";
        return out;
    }
};

static bool refCompare(const Reference& l, const Reference& r) {
    return l.from() < r.from() || (l.from() == r.from() && l.name() < r.name());
}

#define REGISTER_RULE(OperatorType) \
    registerRule(new ##OperatorType##NonnullTypePropagationRule(id++));

enum Type{
    A,
    B
};

#define R(op_type) \
    assert(Type::op_type >= 0);

int main(){
    R(A)
    R(B)

    std::cout << "xxx\n";
    std::string f1("f1");
    std::string f2("f2");
    std::string f3("f3");
    std::string n1("n1");
    std::string n2("n2");
    std::string n3("n3");

    Reference r1(&f1, &n1);
    Reference r2(&f1, &n2);
    Reference r11(&f1, &n1);
    Reference r22(&f1, &n2);
    Reference r3(&f2, &n1);
    Reference r4(&f2, &n2);
    Reference r5(&f3, &n1);
    Reference r51(&f3, &n1);

    std::set<Reference, bool(*)(const Reference& , const Reference&)> res(&refCompare);
    res.insert(r1);
    res.insert(r2);
    res.insert(r3);
    res.insert(r4);
    res.insert(r5);
    res.insert(r51);
    res.insert(r11);
    res.insert(r22);

    std::cout << res.size() << std::endl;
    std::set<Reference, bool (*)(const Reference &, const Reference &)>::iterator it;
    for (it = res.begin(); it != res.end(); it++)
    {
        std::cout << (*it) << std::endl;
    }
    return 0;    
}
