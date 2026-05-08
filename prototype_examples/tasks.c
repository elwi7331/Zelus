#include<stdbool.h>

/*
Working mock up of representing a cooperative program as a timed nuxmv model.
Run this file through c2k2.sh -> k22nuxmv.sh
and add:

###
@TIME_DOMAIN continuous
###

to the top of the file, and:

###
  t: clock;
INIT
  t = 0
INVAR
  (TRUE -> 0 <= t & t <= 3) &
  ("run_a" -> t <= 1) &
  ("run_b" -> 1 <= t & t <= 2) &
  ("run_c" -> 2 <= t)
TRANS
  next(t) = case
    "run_c" & !next("run_c") : 0;
    TRUE : t;
  esac
###

to bottom of the globals module.
Timed LTL specifications can be added e.g.

###
LTLSPEC
  F("a" = 2)
LTLSPEC
  F(("a"=1) U ("a"=2))
###

on the global module, which works as expected.

*/

int a = 0;
bool run_a = true;
bool run_b = false;
bool run_c = false;

void init() {
    assume(a == 0 && run_a && !run_b && !run_c);
}

void schedule() {
    if ( run_a ) {
        run_a = false;
        run_b = true;
    } else if ( run_b ) {
        run_b = false;
        run_c = true;
    } else if ( run_c ) {
        run_c = false;
        run_a = true;
    }
}

void task_a() {
    if ( run_a ) {
        a += 1;
    }
}

void task_b() {
    if ( run_b ) {
        a = a << 1;
    }
}

void task_c() {
    if ( run_c ) {
        a -= 1;
    }
}

int main() {
    while ( 1 ) {
        task_a();
        task_b();
        task_c();
        schedule();
    }
    return 0;
}
