
#include <stdio.h>

void _print_int(int x) {
    printf("%i\n", x);
}

void _print_float(double x) {
    printf("%lf\n", x);
}


extern void calc_main();

int main() {
    calc_main();
    return 0;
}