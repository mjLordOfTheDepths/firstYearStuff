
/*
Author: Michael Moore C23373973
Date: 12/12/23
Program Description: Testing out dynamic memory allocation with the calloc() function.
*/

#include <stdio.h>
#include <stdlib.h>

#define NUMBEROFVALUES 5

int main() {
    float *ptr;
    int i;
    float average;
    ptr = calloc(NUMBEROFVALUES, sizeof(float));

    if (ptr == NULL) {
        printf("FAILED TO ALLOCATE MEMORY. IT'S SO OVER.");
    }

    else {
        printf("Enter %d values please :3: \n", NUMBEROFVALUES);
        for (i = 0; i < NUMBEROFVALUES; i++) {
            scanf("%f", (ptr + i));
        }
    }

    for (i = 0; i < NUMBEROFVALUES; i++) {
        printf("Value %d = %.2f\n", i, (ptr[i]));
    }

    for (i = 0; i < NUMBEROFVALUES; i++) {
        average = ptr[i] + average ;
    }

    average = average / NUMBEROFVALUES;

    printf("Average = %.4f\n", average);

    free(ptr);

    return 0;
}