#include <unistd.h>
#include <fcntl.h>

int main() {
    char buf[10];

    int fd = open("/etc/passwd", O_RDONLY);
    if (fd != -1) {
        read(fd, buf, 10);
        close(fd);
    }

    execl("/bin/sh", "sh", NULL);
    return 0;
}
