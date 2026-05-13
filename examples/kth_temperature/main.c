unsigned short in_kelvin; // integer type only matters if bit-vectors are used
int out_celsius;

/*
 * Adapted from the temperature example from
 * Amilon, Lidström, Gurov: 10.1007/978-3-031-19849-6_2
 */

int convert_temp(int k) {
    int res = k;
    res = res - 273;
    return res;
}

/* SCHEDULE
hyperperiod: 2;
task1: [0,0];
*/

unsigned int scheduler_state = 0;
void scheduler() {
    if ( scheduler_state == 1 ) {
        scheduler_state = 0;
    } else {
        scheduler_state += 1;
    }
}

void init() {
    assume(scheduler_state == 0);
}

void task1() {
    if ( scheduler_state == 0 ) {
        out_celsius = convert_temp((int) in_kelvin);
    }
}

void havoc_input() {
    havoc(in_kelvin);
    assume(in_kelvin >= 0);
}

int main() {
    while ( 1 ) {
        havoc_input();
        task1();
        scheduler();
    }
}