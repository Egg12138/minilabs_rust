#include <errno.h>
#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <sys/types.h>
#include <fcntl.h>

#define BUFSIZE 20
int
main(int argc, char *argv[])
{

        int flags = O_RDONLY;
        printf("errno : %d  \n", errno);
        char *pathname = argv[1];
        int fd = open(pathname, flags);
        if (fd == -1) {
            fprintf(stderr, "Failed to open \n");
        }
        char buf[BUFSIZE];
        size_t numbytes = 20;
        ssize_t cnt = read(fd, buf, numbytes);
        if (cnt == -1) {
            if (errno == EINTR) fprintf(stderr, "read was interrupted by a signal\n");
            else printf("errno is %d\n", errno);
        } 

        return 0;
}
