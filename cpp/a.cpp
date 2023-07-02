#include <iostream>

using namespace std;

int32_t newId()
{
	static int32_t idx = 0;
	idx++;
	cout <<"idx="<<idx<<"\n";
	return idx;
}

class C
{
	private:
		size_t id;
	public:
		C(){
			id = newId();
			cout<<id <<"-constructed\n";
		}
		~C(){cout<<id <<"-destructed\n";}
		C(const C& c){
			id = newId();
			cout<<id <<"-constructed\n";
		}
		C& operator=(const C& c){
			id = newId();
			cout<<id <<"-constructed\n";
      return *this;
		}
		void test() const { cout<<id<<"-test().\n";} 
};

//C func(const C c)
//C func(C c)
C func(const C&& c)
{ 
  cout <<"--21--\n";
	c.test();
  cout <<"--22--\n";
	C c2{c};
  cout <<"--23--\n";
  return c2;
}

int main(){
  cout <<"--1--\n";
	C c;
  cout <<"--2--\n";
	C c2 = func(c);
  cout <<"--3--\n";
	c2.test();
	return 0;
}

