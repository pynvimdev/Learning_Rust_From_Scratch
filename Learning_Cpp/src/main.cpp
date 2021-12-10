#include "iostream"

void allocate_mem(){
    int *pointer;
    pointer = new int;
    *pointer = 45;
}

void free_mems(){
    int *pointer;
    pointer = new int;
    *pointer = 45;
    std::cout << *pointer; 
    delete pointer;
}

int main(){
    allocate_mem();
    free_mems();
}
