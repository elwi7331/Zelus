#include <stdbool.h>

/*
 * Adapted and extended from the temperature example from
 * Amilon, Lidström, Gurov: 10.1007/978-3-031-19849-6_2
 * with a "heater" that is triggered when temperature is below freezing

 * Compared to our previous version of the program,
 * this example introduces
 * - multiple tasks
 * - tasks with different periods
*/

unsigned short in_kelvin;
int out_celsius;
bool heater_on = false;

/* SCHEDULE
hyperperiod: 2;
conversion_task: [0,0];
heater_task: [0.5,0.5];
conversion_task: [1,1];
*/

// chronos annotation
unsigned int scheduler_state = 0;
void scheduler() {
    if ( scheduler_state == 5 ) {
        scheduler_state = 0;
    } else {
        scheduler_state += 1;
    }
}

// kratos annotation
void init() {
    assume(
      scheduler_state == 0 &&
      in_kelvin > 0 && in_kelvin < 374 &&
      !heater_on
    );
}

void havoc_input() {
    havoc(in_kelvin);
    assume(in_kelvin >= 0 && in_kelvin < 374);
}

int convert_temp(int k) {
    int res = k;
    res = res - 273;
    return res;
}

void conversion_task() {
    if ( scheduler_state == 0 || scheduler_state == 4 ) {
        out_celsius = convert_temp((int) in_kelvin);
    }
}

void heater_task() {
    if ( scheduler_state == 2 ) {
        if ( out_celsius < 0 ) {
            heater_on = true;
        } else {
            heater_on = false;
        }
    }
}

int main() {
    while ( 1 ) {
        havoc_input();

        conversion_task();
        heater_task();

        scheduler();
    }
}
