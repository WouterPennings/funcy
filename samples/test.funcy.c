#include<stdio.h>

// Instantiating of the emulated memory
int MEMORY[255];

void print_str() {
	start:
	if(MEMORY[MEMORY[0]] == 0)	goto stop;
	printf("%c", MEMORY[MEMORY[0]]);
	MEMORY[0] = MEMORY[0] + 1;
	goto start;
	stop:
	return;
}

void main() {
	MEMORY[10] = 72;
	MEMORY[11] = 69;
	MEMORY[12] = 76;
	MEMORY[13] = 76;
	MEMORY[14] = 79;
	MEMORY[15] = 44;
	MEMORY[16] = 32;
	MEMORY[17] = 87;
	MEMORY[18] = 79;
	MEMORY[19] = 82;
	MEMORY[20] = 76;
	MEMORY[21] = 68;
	MEMORY[22] = 33;
	MEMORY[23] = 0;
	MEMORY[0] = 10;
	print_str();
	return;
}


