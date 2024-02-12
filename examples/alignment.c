#include <stdio.h>
#include <stdalign.h>
int main(void){
  alignas(65536) int i; //i zajmuje 0x10000 bajtów
  alignas(256) int j; //j zajmuje 0x100 bajtów
  int k; //k zajmuje 4 bajty
  printf("Address of i is %p\n", &i);
  printf("Address of j is %p\n", &j);
  printf("Address of k is %p, sizeof %d\n", &k, sizeof(k));
	printf("Alignment requirement for char is %zu.\n", alignof(char)); //Alignment requirement for char is 1.
	printf("Alignment requirement for int is %zu.\n", alignof(int)); //Alignment requirement for int is 4.
	printf("Alignment requirement for float is %zu.\n", alignof(float)); //Alignment requirement for float is 4.
	printf("Alignment requirement for double is %zu.\n", alignof(double)); //Alignment requirement for double is 8.
}