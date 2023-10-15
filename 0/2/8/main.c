#include <stdio.h>
int main() {
    int i, sum = 0;
    for (i = 1; i <= 501; i++)
        sum += 16 * i * i - 28 * i + 16;
    printf ("%d", sum - 3);
}
