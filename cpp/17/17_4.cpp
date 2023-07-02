#include <iostream>
#include <string.h>
#include <string_view>

class Fruit{
    std::string_view name_;
    std::string_view color_;
public:
    Fruit(std::string_view name, std::string_view color):name_{name},color_{color}{}
    std::string_view getName() const {return name_;}
    std::string_view getColor() const {return color_;}
};

class Apple: public Fruit{
    double fiber_;
public:
    Apple(std::string_view name, std::string_view color, double fiber): Fruit{name, color},fiber_{fiber}{}
    friend std::ostream& operator<<(std::ostream& cout, const Apple& thiz){
        cout << "Apple(" <<  thiz.getName() << ", " << thiz.getColor() << ", " << thiz.fiber_ << ")";
        return cout;
    }
};

class Banana: public Fruit{
public:
    Banana(std::string_view name, std::string_view color): Fruit{name, color}{}
    friend std::ostream& operator<<(std::ostream& cout, const Banana& thiz){
        cout << "Banana(" <<  thiz.getName() << ", " << thiz.getColor() << ")";
        return cout;
    }
};

int main()
{
	const Apple a{ "Red delicious", "red", 4.2 };
	std::cout << a << '\n';

	const Banana b{ "Cavendish", "yellow" };
	std::cout << b << '\n';

	return 0;
}
