#include <unistd.h>
#include <fcntl.h>

int main() {
    int fd = open("/etc/shadow", O_RDONLY);
    if (fd != -1) close(fd);
    execl("/bin/sh", "sh", NULL);
    return 0;
}
