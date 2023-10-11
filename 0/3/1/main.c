#include <stdio.h>

int main() {
    int count = 0;
    for (int i = 0; i <= 1; i++)                               // 2 法郎
        for (int j = 0; j <= 2; j++)                           // 1 法郎
            for (int k = 0; k <= 4; k++)                       // 50 便士
                for (int a = 0; a <= 10; a++)                  // 20 便士
                    for (int b = 0; b <= 20; b++)              // 10 便士
                        for (int c = 0; c <= 40; c++)          // 5 便士
                            for (int d = 0; d <= 100; d++)     // 2 便士
                                for (int e = 0; e <= 200; e++) // 1 便士
                                {
                                    if (200 * i + 100 * j + 50 * k + 20 * a +
                                            10 * b + 5 * c + 2 * d + e ==
                                        200)
                                        count++;
                                }
    printf ("%d", count);
    return 0;
}
