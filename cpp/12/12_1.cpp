#include <iostream>
#include <string>
#include <vector>

int cmp(const std::string& c1, const std::string& c2)
{
    if(c1.length()< c2.length()){
        return -1;
    }else if(c1.length()> c2.length()){
        return 1;
    }
    int x = c1.compare(c2);
    return x;
}

void swap(std::string& s1, std::string& s2){
    std::string h = s1;
    s1 =s2;
    s2 = h;
}

void sort(std::vector<std::string>& array, int (*cmp)(const std::string&, const std::string&))
{
    for (size_t i = 0; i < array.size(); i++)
    {
        for (size_t j = i+1; j < array.size(); j++)
        {
            int x = cmp(array[i], array[j]);
            if(x > 0){
                swap(array[i], array[j]);
            }
        }    
    }
    
}

int main()
{
    std::vector<std::string> array = {"a","aad","b","ba", "b","aac"};
    sort(array, cmp);

    for (auto &&i : array)
    {
        std::cout<< i << " ";   
    }
    std::cout << "\n";

    return 0;
}