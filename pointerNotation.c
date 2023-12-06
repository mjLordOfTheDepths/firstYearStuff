
/* 
Author: Michael Moore C23379373
Date: 5/12/23
Program Description: Copying contents of Array A to array B using pointer notation.
*/

#include <stdlib.h>
#include <stdio.h>

#define SIZE 3

int main() {

    float firstArray[SIZE];
    float secondArray[SIZE];
    int i = 0;

    for (i=0;i < SIZE;i++) {
        printf("Enter a value for index location %d in the first array.\n", i);
        scanf("%f", & *(firstArray + i));
    }

    for (i=0;i < SIZE;i++) {
        printf("%f\n", *(firstArray + i));
    }

    for (i=0;i < SIZE;i++) {
    printf("Second Array contains the following number in index location %d:\n", i);
    *(secondArray + i) = *(firstArray +i);
    printf("%f\n", *(secondArray + i));
    }

    return 0;
}