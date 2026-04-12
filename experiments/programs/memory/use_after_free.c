#include <stdio.h>
#include <stdlib.h>

int main() {
    int *p = malloc(sizeof(int));
    *p = 42;

    free(p);

    // utilisation après libération
    printf("%d\n", *p);

    return 0;
}
