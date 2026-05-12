#define OPEN 1
#define CLOSED 0
#define N 32

/*
 * Adapted from the IO example from
 * Amilon, Lidström, Gurov: 10.1007/978-3-031-19849-6_2
 * with a "heater" that is triggered when temperature is below freezing
 */

/* CHRONOS_SCHEDULE
hyperperiod: 1;
task1: [0,0.1];
task2: [0.5,0.6];
*/

// chronos annotation
unsigned int scheduler_state = 0;
void scheduler() {
    if ( scheduler_state == 3 ) {
        scheduler_state = 0;
    } else {
        scheduler_state += 1;
    }
}

int file_status = CLOSED;
int input;

// KRATOS annotation
void assume() {
    assume(scheduler_state == 0 && file_status == CLOSED);
}

void havoc_input () {
    havoc(input);
    assume(-(N*2) <= input && input <= N*2);
}

int read_file(int i){
    return i; // Dummy statement
}

void write_file(int i) {
    // pass
}

void file_operation(int n) {
    int i; i = 0;
    int tmp; tmp = 0;
    int sum; sum = 0;
    if (file_status == OPEN) {
        while (i < n) {
            tmp = read_file(i);
            sum += tmp;
            i += 1;
        }
        write_file(sum);
        file_status = CLOSED;
    }
}

void task1() {
    if ( scheduler_state == 0 ) {
        if (0 < input && input < N) {
            file_status = OPEN;
        }
    }
}

void task2() {
    if ( scheduler_state == 2 ) {
        if ( file_status == OPEN ) {
            file_operation(input);
        }
    }
}

void main() {
    while (1) {
        havoc_input ();
        task1();
        task2();
        scheduler();
    }
}