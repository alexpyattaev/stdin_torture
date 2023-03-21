#include <unistd.h>
#include <stdint.h>
#include <stdio.h>
#define BUFFER_SIZE 1*1024*1024

int main(void)
{
  int fd = 0;
  ssize_t bytes_read;
  char buf[BUFFER_SIZE + 1];
  size_t lines = 0;

  while ((bytes_read = read (fd, buf, BUFFER_SIZE)) > 0)
    {

      char *p = buf;
      char *end = buf + bytes_read;

      /* Avoid function call overhead for shorter lines.  */
      while (p != end)
      {
        lines += *p++ == '\n';
      }

    }
printf("Counted %lu lines\n", lines );
}
