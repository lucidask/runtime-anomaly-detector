#include <stdio.h>
#include <stdlib.h>

int main() {
    int *arr = malloc(5 * sizeof(int));

    for (int i = 0; i < 5; i++) {
        arr[i] = i;
    }

    // lecture hors limite → invalid read
    int x = arr[5];
    printf("%d\n", x);

    free(arr);
    return 0;
}
