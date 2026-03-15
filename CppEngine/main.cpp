#include "iostream"

#include "Encodeur.hpp"

using namespace std;
using namespace Encodeur;

struct Base {
	virtual void foo() {}
};

int main()
{
	string* str = new string("Hello World!");
	std::cout << Encode(str->c_str(), str->size()) << std::endl;
	delete str;

	std::cin.ignore();
}