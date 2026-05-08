
/*
    Translating to nuXmv yields a model with infinite runs.

    nuXmv succeeds to prove that:
        G( F("!{$(in_main#0)<$(in_foo#0)<i>>}" = 0) )
    and disprove its negation

    i and x seem to "move together" in simulation, so that they are always are equal to each other. 
    This is the case also when slicing is disabled.
*/

void foo() {
    int x = 0;
    for ( int i = 0; i < 10; ++i ) {
        ++x;
    }
}

int main() {
    while ( 1 ) {
        foo();
    }
    return 0;
}
