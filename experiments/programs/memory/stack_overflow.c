#include <stdio.h>
#include <string.h>

int main() {
    char buffer[8];

    // dépassement de tampon sur la pile
    strcpy(buffer, "AAAAAAAAAAAAAAAAAAAAAAAA");

    printf("%s\n", buffer);
    return 0;
}
