#include <cstdio>

int main()
{
    int x { 5 };    // normal variable
    int& ref { x }; // a reference to an integer (bound to x)
    int* ptr;       // a pointer to an integer

    int value{ 45 };
    int* ptr2{ &value }; // declare a pointer and initialize with address of value
    u_int64_t h  = (u_int64_t)(&value);

    printf("%d,%d,%d,%d\n", (&x),(&ref),(&ptr),h);
    
    NULL
    return 0;
}