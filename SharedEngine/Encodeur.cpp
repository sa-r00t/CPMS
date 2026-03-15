#include <iostream>
#include "Encodeur.hpp"

using namespace std;

void Encodeur::Encode(const char* input, const size_t sizeof_input, std::string& output)
{
	const uint8_t MAGIC[2] = { 0x4D, 0x53 };
	const uint8_t VERSION = 1;
	const uint8_t TYPE = 1;

	// J'utilise pas mutex ? Mais est-ce vraiment si utile ?

	output.append(reinterpret_cast<const char*>(MAGIC), sizeof(MAGIC));
	output.push_back(static_cast<char>(VERSION));
	output.push_back(static_cast<char>(TYPE));
	output.append(input, sizeof_input);;

	// Mais il manque pas la fin ?
}

std::string Encodeur::Encode(const char* input, const size_t sizeof_input)
{
	string temp;

	const uint8_t MAGIC[2] = { 0x4D, 0x53 };
	const uint8_t VERSION = 1;
	const uint8_t TYPE = 1;

	// J'utilise pas mutex ? Mais est-ce vraiment si utile ?

	temp.append(reinterpret_cast<const char*>(MAGIC), sizeof(MAGIC));
	temp.push_back(static_cast<char>(VERSION));
	temp.push_back(static_cast<char>(TYPE));
	temp.append(input, sizeof_input);

	// Mais il manque pas la fin ?

	return temp;
}

void Encodeur::Decode(const char* input, const size_t sizeof_input, std::string& output)
{

}

std::string Encodeur::Decode(const char* input, const size_t sizeof_input)
{
	string* test = new string("test");

	return *test;
}

