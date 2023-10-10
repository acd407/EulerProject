#include <stdio.h>
#define M       (28123 + 1)
#define M_RANGE (7000 + 5)

int abundant[M_RANGE];
int vis[M] = {0};

int main() {
    int cnt = 0, ans = 0;

    for (int i = 1; i < M; i++) {
        int factor_sum = 0;
        for (int j = 1; j <= i / 2; j++)
            if (i % j == 0)
                factor_sum += j;
        if (factor_sum > i)
            abundant[cnt++] = i;
    }

    for (int i = 0; i < cnt; i++)
        for (int j = i; j < cnt; j++) {
            int sum = abundant[i] + abundant[j];
            if (sum < M && ! vis[sum])
                vis[sum] = 1;
        }

    for (int i = 0; i < M; i++)
        if (! vis[i])
            ans += i;

    printf ("%d\n", ans);
    return 0;
}
