#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    char *buffer = malloc(8);

    // dépassement de tampon sur le tas
    strcpy(buffer, "AAAAAAAAAAAAAAAAAAAA");

    printf("%s\n", buffer);

    free(buffer);
    return 0;
}
