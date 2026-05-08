
/**
    KRATOS determines that this program is SAFE, i.e. 'kratos -stage=mc <fac.k2>'

    If converted to nuXmv, an INVARSPEC is provided that seems to correspond to the assert.
    nuXmv can prove the property, and if it is negated the SPEC, it disproves it.

    If the nuXmv model is simulated:
        - I found there to be 2 possible initial states
            * For one of them, deadlock is instant.
            * For the other, there are a few transitions followed by a deadlock

    It seems that terminating programs can give rise to nuXmv models with no infinite runs.
    These are therefore unsuited for verification of properties in temporal logic
**/

int fac(int n) {
    int res = 1;
    while ( n > 1 ) {
        res *= n;
        --n;
    }
    return res;
}

int main() {
    assert(fac(3) == 6);
    return 0;
}
