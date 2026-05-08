#include<stdbool.h>
#include<stdio.h>

/*
CHRONOS_SCHEDULE
hyperperiod: 1;
task_1: [0, 0];
*/

/* Some properties that nuXmv (can) prove:
 *
 * LTLSPEC -- error follows malfunction
 *   G("sensor_faulty" -> F[0,1](G("sensor_error")))
 * LTLSPEC -- error is permanent
 *   G("sensor_error" -> G("sensor_error"))
 * LTLSPEC -- malfunction free runs are also error free
 *   -- this can fail as global variable is not updated...
 *   G(!"sensor_faulty") -> G(!"sensor_error")
*/

unsigned int scheduler_state = 0;
void scheduler() {
    if ( scheduler_state == 1 ) {
        scheduler_state = 0;
    } else {
        scheduler_state += 1;
    }
}

bool sensor_faulty = false;
bool sensor_error = false;

void init() {
    assume(scheduler_state == 0 && !sensor_faulty && !sensor_error);
}

int read_sensor() {
    unsigned char value;
    havoc(value);
    
    if ( sensor_faulty ) {
        return -1;
    } else {
        return (int) value;
    }
}

void task_1() {
    if ( scheduler_state == 0 ) {
        havoc(sensor_faulty);
        int measured_value = read_sensor();
        if ( measured_value < 0 ) {
            sensor_error = true;
        } else {
            sensor_error = sensor_error;
        }
    }
}

int main() {
    while ( true ) {
        task_1();
        scheduler();
    }
    return 0;
}