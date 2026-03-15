#pragma once

#include "framework.h"

#define ENCODEUR_VERSION "1.0.0"

#ifdef SHAREDENGINE_EXPORTS
#define ENCODEUR_API __declspec(dllexport)
#else
#define ENCODEUR_API __declspec(dllimport)
#endif

#ifndef __ENCODEUR__

namespace Encodeur
{
	/// <summary>
	/// Encodes the input string and stores the result in the output string.
	/// </summary>
	/// <param name="input"> Input </param>
	/// <param name="output"> Output </param>
	/// <returns></returns>
	ENCODEUR_API void Encode(const char* input, const size_t sizeof_input, std::string& output);

	/// <summary>
	/// Encodes the input string and stores the result in the output string.
	/// </summary>
	/// <param name="input"> Input </param>
	/// <param name="output"> Output </param>
	/// <returns></returns>
	ENCODEUR_API std::string Encode(const char* input, const size_t sizeof_input);

	/// <summary>
	/// Dedodes the input string and stores the result in the output string.
	/// </summary>
	/// <param name="input"> Input </param>
	/// <param name="output"> Output </param>
	/// <returns></returns>
	ENCODEUR_API void Decode(const char* input, const size_t sizeof_input, std::string& output);

	/// <summary>
	/// Decodes the input string and stores the result in the output string.
	/// </summary>
	/// <param name="input"> Input </param>
	/// <param name="output"> Output </param>
	/// <returns></returns>
	ENCODEUR_API std::string Decode(const char* input, const size_t sizeof_input);
}

#define __ENCODEUR__
#endif