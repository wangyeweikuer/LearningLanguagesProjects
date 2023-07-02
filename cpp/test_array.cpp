#include <iostream>
using namespace std;

int main(){
    int size, v, sum = 0;
    int[][] arr = new int[][];
    cout << "Please input a size:" << endl;

    while(cin >> size){
        cout << "Input " << size << " numbers:" << endl;
        for (size_t i = 0; i < size; i++)
        {
            if(cin >> v){
                sum += v;
            }
        }
        cout << "The sum is " << sum << endl;
        cout << "Please input a size:" << endl;
        sum = 0;
    }   
}
