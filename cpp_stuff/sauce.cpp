#include <stdio.h>
#include <cstdlib>

#if defined(__clang__) || defined(__GNUC__) || defined(__GNUG__)

	int log2(int x) {
		return 31 ^ __builtin_clz(x);
	}

#elif defined(_MSC_VER)
	#include <intrin.h>

	int log2(int x) {
		unsigned long r = 0;
		_BitScanReverse(&r, x);
		return r;
	}
#endif

template<int cl2>
int magicFunc(int x, int y) {
	int xy = x + y;

	int val = 1;
	for (int i = 0; i < cl2; ++i)
		val += (((x >> i) + (y >> i)) & 1) << i;

	return val;
}


template<int cl2>
char* createGrid(int size) {
	int index = 0;

	char* output = (char*) malloc(size * size);

	for (int y = 0; y < size; ++y) {
		for (int x = 0; x < size; ++x) {
			output[index++] = magicFunc<cl2>(x, y);
		}
	}
	return output;
}

char* createGrid10() {
	return createGrid<4>(10);
}

char* createGrid50() {
	return createGrid<6>(50);
}

char* createGrid51() {
	return createGrid<6>(51);
}

char* createGrid100() {
	return createGrid<7>(100);
}
