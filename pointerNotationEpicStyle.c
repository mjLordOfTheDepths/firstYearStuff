
/*
Author: Michael Moore C23373973
Date: 5/12/23
Program Description: Faffing about with pointers.
*/

#include <stdlib.h>
#include <stdio.h>

int main() {
    int num1 = 1;
    char char1 = '\\';
    int *ptr1;
    char *ptr2;
    int *ptr3 = &ptr1;
    int *ptr4 = &ptr2;
    int *pointerThatPointsAtCharactersDespiteBeingAnIntDataTypeVariable;

    ptr1 = &num1;
    ptr2 = &char1;
    pointerThatPointsAtCharactersDespiteBeingAnIntDataTypeVariable = &char1;

    printf("Address of num1: %d\n", ptr1);
    printf("Address of char1: %d\n", ptr2);

    printf("Contents of num1: %d\n", *ptr1);
    printf("Contents of char1: %c\n", *ptr2);

    printf("Address of ptr1: %d\n", ptr3);
    printf("Address of ptr2: %d\n", ptr4);
    printf("Address of char1 when pointed at by an int: %d\n", pointerThatPointsAtCharactersDespiteBeingAnIntDataTypeVariable); // An integer pointer is capable of finding the memory address of a chracter.



    return 0;
}