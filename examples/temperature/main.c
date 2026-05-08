#include <stdbool.h>
#include <stdio.h>

/*
CHRONOS_SCHEDULE
hyperperiod: 1;

sensor_task:    [0, 0.1];
display_task:   [0, 0.1];
control_task:   [0, 0.1];

control_task:   [0.2, 0.3];
sensor_task:    [0.5, 0.6];
control_task:   [0.6, 0.7];
control_task:   [0.8, 0.9];
*/

/*
 * Some properties that can be checked after nuXmv translation
 *
 * DEFINE
 * temp_value := READ("%mem.0", "&temp_queue");
 * fan_on := "fan_status";
 * fan_off := !"fan_status";
 * 
 * LTLSPEC
 *   G(temp_value >= 0)
 * 
 * LTLSPEC -- 486.43 seconds
 *   G((temp_value > 30) -> F[0,1](fan_on))
*/


// scheduler annotation
int scheduler_state = 0;
void scheduler() {
    if ( scheduler_state == 10 ) {
        scheduler_state = 0;
    } else {
        scheduler_state += 1;
    }
}

bool fan_status = false;
double temp_queue = 0.0;

void init() {
    assume(
        !fan_status
        && temp_queue == 0.0
        && scheduler_state == 0
    );
}

void fan_on() {
    fan_status = true;
}

void fan_off() {
    fan_status = false;
}

bool queue_receive(double* queue, double* dest) {
    *dest = *queue;
    return true;
}

void queue_send(double* queue, double* src) {
    *queue = *src;
}

double read_sensor() {
    double temp = -1.0;
    while ( temp < 0.0 || temp > 100 ) {
        havoc(temp);
    }
    return temp;
}

void sensor_task() {
    if ( scheduler_state == 0 || scheduler_state == 6 ) {
        double temp = read_sensor();
        queue_send(&temp_queue, &temp);
    }
}

void display_task() {
    if ( scheduler_state == 1 ) {
        double temp;
        if(queue_receive(&temp_queue, &temp)) {
            printf("Temperature: %.2f C\n", temp);
        }
    }
}

void control_task() {
    if ( scheduler_state == 2 || scheduler_state == 4 || scheduler_state == 7 || scheduler_state == 9 ) {

        double temp;
        if(queue_receive(&temp_queue, &temp)) {
            if(temp >= 28)
                fan_on();
            else
                fan_off();
        }

    }
}

int main() {
    while ( 1 ) {
        sensor_task();
        display_task();
        control_task();
        scheduler();
    }
    return 0;
}
