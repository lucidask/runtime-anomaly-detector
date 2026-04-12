#include <unistd.h>

int main() {
    char *args[] = {"/bin/sh", NULL};
    execve("/bin/sh", args, NULL);
    return 0;
}