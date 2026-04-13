#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void test_passwd_shell() {
    FILE *f = fopen("/etc/passwd", "r");
    if (f != NULL) {
        fclose(f);
    }
    system("/bin/sh");
}

void test_shadow_shell() {
    FILE *f = fopen("/etc/shadow", "r");
    if (f != NULL) {
        fclose(f);
    }
    system("/bin/sh");
}

void test_tmp_exec() {
    system("/tmp/tmp_exec");
}

void test_freq() {
    for (int i = 0; i < 200; i++) {
        FILE *f = fopen("/etc/hostname", "r");
        if (f != NULL) {
            char c;
            fread(&c, 1, 1, f);
            fclose(f);
        }
    }
}

void test_invalid_read() {
    int *arr = malloc(5 * sizeof(int));
    for (int i = 0; i < 5; i++) {
        arr[i] = i;
    }
    int x = arr[5];
    printf("%d\n", x);
    free(arr);
}

void test_invalid_write() {
    int *arr = malloc(5 * sizeof(int));
    for (int i = 0; i <= 5; i++) {
        arr[i] = i;
    }
    free(arr);
}

void test_invalid_free() {
    int x = 10;
    free(&x);
}

void test_uninit() {
    int x;
    if (x > 0) {
        printf("positif\n");
    } else {
        printf("negatif ou zero\n");
    }
}

void test_uaf() {
    int *p = malloc(sizeof(int));
    *p = 42;
    free(p);
    printf("%d\n", *p);
}

void test_heap_overflow() {
    char *buffer = malloc(8);
    strcpy(buffer, "AAAAAAAAAAAAAAAAAAAA");
    printf("%s\n", buffer);
    free(buffer);
}

void test_stack_overflow() {
    char buffer[8];
    strcpy(buffer, "AAAAAAAAAAAAAAAAAAAAAAAA");
    printf("%s\n", buffer);
}

void test_segfault() {
    int *p = NULL;
    *p = 42;
}

int main(int argc, char *argv[]) {
    if (argc < 2) {
        printf("Usage: %s <mode>\n", argv[0]);
        return 1;
    }

    if (strcmp(argv[1], "passwd_shell") == 0) {
        test_passwd_shell();
    } else if (strcmp(argv[1], "shadow_shell") == 0) {
        test_shadow_shell();
    } else if (strcmp(argv[1], "tmp_exec") == 0) {
        test_tmp_exec();
    } else if (strcmp(argv[1], "freq") == 0) {
        test_freq();
    } else if (strcmp(argv[1], "invalid_read") == 0) {
        test_invalid_read();
    } else if (strcmp(argv[1], "invalid_write") == 0) {
        test_invalid_write();
    } else if (strcmp(argv[1], "invalid_free") == 0) {
        test_invalid_free();
    } else if (strcmp(argv[1], "uninit") == 0) {
        test_uninit();
    } else if (strcmp(argv[1], "uaf") == 0) {
        test_uaf();
    } else if (strcmp(argv[1], "heap_overflow") == 0) {
        test_heap_overflow();
    } else if (strcmp(argv[1], "stack_overflow") == 0) {
        test_stack_overflow();
    } else if (strcmp(argv[1], "segfault") == 0) {
        test_segfault();
    } else {
        printf("Unknown mode: %s\n", argv[1]);
        return 1;
    }

    return 0;
}
