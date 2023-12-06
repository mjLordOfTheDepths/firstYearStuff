
/*
Author: Michael Moore C23373973
Date: 28th November 2023
Program Description: RNG Guessing game epic style
*/

#include <stdlib.h>
#include <stdio.h>
#include <time.h>

#define MAX_ATTEMPTS 5

int main() {
    int num = 0;
    int guessHistory[MAX_ATTEMPTS];
    int userGuess; 
    int i; // Counter Variable
    int winningCounter = 0; // "Victory Point"
    int numGuesses = 0;
    printf("Generating a random number between 1 – 25 \n");
    // seed the random number generator with a range 0 – large number
    srand(time(NULL));
    // num is assigned a random number between 1 – 10
    num = (rand() % 25) + 1;

    // Lab Test Starts:

    /* printf("%d\n\n", num); // Testing Purposes, uncomment to have num viewable by user */

    for (i = 0; i < (MAX_ATTEMPTS); i++) { // Start forOne

        while ((userGuess > 25) || (userGuess < 1)) { // Start whileOne; Entering a value where 1 <= n <= 25
            printf("Please enter yuor guess where 1 <= n <= 25:\n");
            scanf("%d", &userGuess);
            guessHistory[i] = userGuess;
        
        } // End whileOne

        if (userGuess == num) { // Winning Condition
            printf("Yuo win! Yuo guessed %d correctly!\n", num);
            i = MAX_ATTEMPTS + 1; // Breaking out of the loop
            winningCounter = 1; // Victory point
        }
        
        if (userGuess > num) {
            printf("Too High!\n");
        }

        if (userGuess < num) {
            printf("Too Low!\n");
        }

        userGuess = 0; // Resetting the value of user guesses to a value outside of the given range
        numGuesses++;

    } // End forOne

    if ((winningCounter == 0) && (i == MAX_ATTEMPTS)) {
        printf("Yuo lose... it's so over\n");
    }

    printf("Yuo guessed the following numbers:\n[");

    for (i = 0; i < numGuesses - 1; i++) { // Printing guessHistory indexes of 0 to n - 1 
        printf("%d, ", guessHistory[i]);
    }

    printf("%d]\n", guessHistory[i]); // Printing last value in guessHistory 

    return 0;
} // end main()
