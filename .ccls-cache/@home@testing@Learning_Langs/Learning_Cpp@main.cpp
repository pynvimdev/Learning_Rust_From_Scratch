#include <cstdlib>
#include <iostream>
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
  printf("function : allocate_stack_mem \n");
  printf("Value of A is %i \n", z);
  printf("Value of z is %i \n", x);
}

void allaocate_heap_mem() {
  // Creates a Memory for 10 integers
  printf("allaocate_heap_mem \n");
  int ptr = 10;
  malloc(ptr);
}

void printsusingstd(){
    printf("printsusingstd \n");
    std::cout << "Hello Wrld";
}

void MergeSort(){

}

int main(){
    allocate_stack_mem();
    allaocate_heap_mem();
}
