
/* Generating nuXmv from this, we can verify that
 * F(G(("x" = 6)U("x" = -6) | ("x" = -6)U("x" = 6)))
*/
int x = 0;

int mul(int a, int b) {
    int res = 1;
    //  int res = 0;
    for ( int i = 0; i < b; ++i ) {
        res += a;
    }
    for (int i = 0; i < -b; ++i) {
        res -= a;
    }
    return res;
}

int main() {
    while ( 1 ) {
        x = mul(2, 3);
        x = mul(2, -3);
    }
    return 0;
}
