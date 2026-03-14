#include "iostream"

#include "Encodeur.hpp"

using namespace std;
using namespace Encodeur;

int main()
{
	string* str = new string("Hello World!");
	std::cout << Encode(*str) << std::endl;
	delete str;

	std::cin.ignore();
}