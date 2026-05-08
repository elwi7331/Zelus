/*
    My understanding is that transition system encodings do not support recursion.
    Accordingly, Kratos (using nuXmv) fails to conclude that this program is SAFE.
*/

int fac(int n) {
    if ( n == 1) {
        return 1;
    }
    return n * fac(n - 1);
}

int main() {
    int res = fac(3);
    assert(res == 6);
    return res;
}
