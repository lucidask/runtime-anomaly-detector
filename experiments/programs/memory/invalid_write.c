#include <stdio.h>
#include <stdlib.h>

int main() {
    int *arr = malloc(5 * sizeof(int));

    // écriture hors limite → invalid write
    for (int i = 0; i <= 5; i++) {
        arr[i] = i;
    }

    free(arr);
    return 0;
}
