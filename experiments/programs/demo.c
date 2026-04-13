#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

int main() {
    // Syscall 
   system("sh -c 'echo shell_triggered'");

    // Problème mémoire
    int *ptr = malloc(sizeof(int) * 2);
    ptr[5] = 42; // overflow

    free(ptr);

    return 0;
}
