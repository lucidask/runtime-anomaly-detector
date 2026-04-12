#include <stdio.h>
#include <stdlib.h>

int main() {
    int x = 10;

    // free invalide : x n'a pas été alloué par malloc
    free(&x);

    return 0;
}
