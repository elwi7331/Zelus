/*
 * We can havoc variables with constrained values.
 * e.g. we can prove
 * LTLSPEC
 *   G(0 <= "x" & "x" <= 10)
 * and disprove
 * LTLSPEC
 *   G(0 < "x" & "x" <= 10)
*/

#include <stdbool.h>

bool scheduled = true;
int x = 0;
int y = 10;

void init() {
    assume(x == 0 && y == 10 && scheduled);
}

void havoc_x() {
    if ( scheduled ) {
        havoc(x);
        assume( 0 <= x && x <= 10 );
    }
}

void update_y() {
    if ( !scheduled ) {
        y = x + 10;
    }
}

void scheduler() {
    scheduled = !scheduled;
}

int main() {
    while ( 1 ) {
        havoc_x();
        update_y();
        scheduler();
    }
    return 0;
}