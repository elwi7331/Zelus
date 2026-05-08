/*
 * Simple example of a program were intermediate states are observable in nuXmv translation.
*/

unsigned int counter = 0;

void init() {
    assume(counter == 0);
}

void increment() {
    if ( counter == 10 ) {
        counter = 0;
    } else {
        counter += 1;
    }
}

int main() {
    while ( 1 ) {
        increment();
    }
    return 0;
}
