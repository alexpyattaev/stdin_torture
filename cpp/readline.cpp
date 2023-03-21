#include <iostream>
//#include <cstdio>

using namespace std;

int main() {
//	std::ios_base::sync_with_stdio(false);
    string input_line;
    long line_count = 0;

    while (cin) {
        getline(cin, input_line);
	input_line.clear();
   //     if (!cin.eof())
            line_count++;
    };
    return 0;
}

// Compiled with:
// g++ -O3 -o readline_cpp readline.cpp
