#include <stdio.h>

/*
- Stack Memory in cpp
- Memory is allaocated and removed quickly
- But in heap Memory its slow but can be changed or modifeid

Stack Memory function(allaocateStackMemory)
----------------------------------------------------------------------------/
| Address | Name | Value
|   0   |  x  | 10
|   1   |  y  | 20

Heap Memory function(allaocateHeapMemory)
----------------------------------------------------------------------------/
| Address | Name | Value
|   0   |  x  | 10
|   1   |  y  | 20
*/
void allocate_stack_mem() {
  int z = 10;
  int x = 10;
  printf("Value of A is %i", z);
  printf("allocate_stack_mem \n");
  printf("Value of A is %i", x);
}

void allaocate_heap_mem() {
  // Creates a Memory for 10 integers
  printf("allaocate_heap_mem \n");
  int *ptr = new int[10];
}
