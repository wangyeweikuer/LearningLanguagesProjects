#include <iostream>
#include <string_view>
#include <string>
#include <cassert>
#include <array>
#include <cstdlib> // for rand() and srand()
#include <ctime> // for time()
#include <random>

class Creature
{
protected:
    std::string m_name;
    char m_symbol;
    int m_health;
    int m_damage;
    int m_gold;

public:
    Creature(std::string_view name, char symbol, int health, int damage, int gold) 
        :m_name{name}, m_symbol{symbol}, m_health{health}, m_damage{damage}, m_gold{gold} {}
    Creature(const Creature &c) : Creature{c.m_name, c.m_symbol, c.m_health, c.m_damage, c.m_gold} {}

    ~Creature(){};
    const std::string& getName() const { return m_name; }
    char getSymbol() const { return m_symbol; }
    int getHealth() const { return m_health; }
    int getDamage() const { return m_damage; }
    int getGold() const { return m_gold; }

    void reduceHealth(int);
    bool isDead();
    void addGold(int);
};

void Creature::reduceHealth(int x)
{
    m_health -= x;
}

bool Creature::isDead()
{
    return m_health <= 0;
}

void Creature::addGold(int x)
{
    m_gold += x;
}

class Player : public Creature
{
private:
    int m_level{1};

public:
    Player(std::string_view name, int level = 1) : Creature{name, '@', 10, 1, 0}, m_level{level} {}
    ~Player(){};

    void levelUp()
    {
        m_level++;
        m_damage--;
    }
    int getLevel() const { return m_level; }
    bool hasWon()
    {
        return m_level >= 20;
    }

    friend std::ostream &operator<<(std::ostream &cout, const Player &p)
    {
        cout << "You have " << p.getHealth() << " health and is carrying " << p.getGold() << " gold.\n";
        return cout;
    }
};


class Monster : public Creature
{
public:
    enum class Type
    {
        dragon,
        orc,
        slime,
        max_types
    };

private:
    static const Creature &getDefaultCreature(Type type)
    {
        static const std::array<Creature, static_cast<size_t>(Type::max_types)> lookups
        {
            {
            {"dragon", 'D', 20, 4, 100},
            {"orc", 'o', 4, 2, 25},
            {"slime", 's', 1, 1, 10}
            }
        };
        return lookups.at(static_cast<std::size_t>(type));
    }


public:
    Monster(Type type) : Creature(getDefaultCreature(type)){}
    ~Monster(){};
    static Monster getRandomMonster(){
        std::mt19937 mt{ std::random_device{}() };
        std::uniform_int_distribution ud{0,  static_cast<int>(Type::max_types)-1};
        auto h = ud(mt);
        return Monster{static_cast<Type>(h)};
    }
};

int main()
{
    //     Creature o{"orc", 'o', 4, 2, 10};
    //     o.addGold(5);
    //     o.reduceHealth(1);
    //     std::cout << "The " << o.getName() << " has " << o.getHealth() << " health and is carrying " << o.getGold() << " gold.\n";

    // std::string name;
    // std::cout << "Enter your name: ";
    // std::cin >> std::ws >> name;
    // std::cout << "Welcome, " << name << '\n';
    // Player p{name};
    // std::cout << p;

    // Monster m{Monster::Type::orc};
    // std::cout << "A " << m.getName() << " (" << m.getSymbol() << ") was created.\n";

    std::srand(static_cast<unsigned int>(std::time(nullptr))); // set initial seed value to system clock
	std::rand(); // get rid of first result
    
    for (int i{ 0 }; i < 10; ++i)
	{
		Monster m{ Monster::getRandomMonster() };
		std::cout << "A " << m.getName() << " (" << m.getSymbol() << ") was created.\n";
	}
    return 0;
}

