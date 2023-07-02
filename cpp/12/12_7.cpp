#include <iostream>
#include <functional>
#include <string_view>
#include <array>
#include <algorithm>

double avg(int a, int b)
{
    return (a + b) / 2.0;
}

// int main()
// {
//     std::function<double(int, int)> fp = &avg;
//     std::function<double(int, int)> fp2 = avg;
//     double (*func)(int, int) = &avg;
//     double (*func2)(int, int) = avg;

//     std::cout << fp(1, 2) << '\n';
//     std::cout << fp2(1, 2) << '\n';
//     std::cout << func(1, 2) << '\n';
//     std::cout << func2(1, 2) << '\n';

//     std::greater{};

//     auto found1 = [](std::string_view str) -> bool
//     {
//         return str.find("nut") != std::string_view::npos;
//     };

//     auto found0 = [](std::string_view str){
//         return str.find("nut") != std::string_view::npos;
//     };

//     bool (*found2)(std::string_view str) = [](std::string_view str) -> bool
//     {
//         return str.find("nut") != std::string_view::npos;
//     };

//     std::function<bool(std::string_view)> found3 = [](std::string_view str) -> bool
//     {
//         return str.find("nut") != std::string_view::npos;
//     };

//     constexpr std::array<std::string_view, 4> arr = {"aaa", "bbb", "ccnutsss", "yyy"};

//     auto x = std::find_if(arr.begin(), arr.end(), found1);
//     if (x != arr.end())
//     {
//         std::cout << (*x) << '\n';
//     }

//     x = std::find_if(arr.begin(), arr.end(), found2);
//     if (x != arr.end())
//     {
//         std::cout << (*x) << '\n';
//     }

//     x = std::find_if(arr.begin(), arr.end(), found3);
//     if (x != arr.end())
//     {
//         std::cout << (*x) << '\n';
//     }
//     return 0;
// }

struct Student
{
    std::string_view name{};
    int points{};
};

struct Season
{
    std::string_view name{};
    double averageTemperature{};
};

int main()
{
    std::array<Student, 8> arr{
        {{"Albert", 3},
         {"Ben", 5},
         {"Christine", 2},
         {"Dan", 8}, // Dan has the most points (8).
         {"Enchilada", 4},
         {"Francis", 1},
         {"Greg", 3},
         {"Hagrid", 5}}};

    typedef const Student &cs;
    auto cmp = [](cs a, cs b)
    {
        return a.points < b.points;
    };
    auto it = std::max_element(arr.begin(), arr.end(), cmp);

    if (it != arr.end())
    {
        std::cout << it->name << " is the best student\n";
    }

    std::array<Season, 4> seasons{
        {{"Spring", 285.0},
         {"Summer", 296.0},
         {"Fall", 288.0},
         {"Winter", 263.0}}};

    auto cmp2 = [](const Season& f, const Season& s)
    {
        return f.averageTemperature < s.averageTemperature;
    };
    std::sort(seasons.begin(), seasons.end(), cmp2);
    for (const auto &season : seasons)
    {
        std::cout << season.name << '\n';
    }
    return 0;
}