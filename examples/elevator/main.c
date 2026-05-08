#include<stdbool.h>
#include<stdio.h>

/*
 * Some Specifications that can be proven with nuXmv (put in module Globals):
 *
 * LTLSPEC -- elevator will initially be stationary
 *   F[0,0]("elevator_direction_monitor" = 0ub2_01)
 * LTLSPEC -- elevator direction will be still within 3 seconds
 *   G(F[0,3]("elevator_direction_monitor" = 0ub2_01))
 * LTLSPEC -- fairness sanity check
 *   G(F("scheduler_state" = 0) & F("scheduler_state" = 9))
 * LTLSPEC -- not true that elevator direction will be up (?) within 3 seconds
 *   !G(F[0,3]("elevator_direction_monitor" = 0ub2_10))
*/

/*
CHRONOS_SCHEDULE
hyperperiod: 3;

elevator_door_controller:   [0, 1];
elevator_speed_controller:  [0, 1];
move_elevator:              [0, 1];

elevator_door_controller:   [1, 2];
elevator_speed_controller:  [1, 2];
move_elevator:              [1, 2];

elevator_door_controller:   [2, 3];
elevator_speed_controller:  [2, 3];
move_elevator:              [2, 3];
get_destination:            [2, 3];
*/

typedef enum {
    STILL,
    UP,
    DOWN,
} Direction;

typedef enum {
    BASEMENT,
    GROUND,
    FIRST,
    SECOND
} Floor;

typedef struct {
    Direction direction;
    bool door_open;
    Floor position;
    Floor destination;
} Elevator;

// scheduler annotation:
unsigned int scheduler_state = 0;
void scheduler() {
    if ( scheduler_state == 9 ) {
        scheduler_state = 0;
    } else {
        scheduler_state += 1;
    }
}

// Kratos encodes the struct using array-modeled memory.
// Thus, we introduce these monitors to expose values for LTL specifications
bool elevator_door_monitor = false;
Direction elevator_direction_monitor = STILL;

// Kratos init function - reduces number of initial states in generated model
void init() {
    assume(
        scheduler_state == 0
        && !elevator_door_monitor
        && elevator_direction_monitor == STILL
    );
}

Elevator elevator1 = {
    .direction = STILL,
    .door_open = true,
    .position = GROUND,
    .destination = GROUND,
};

void elevator_door_controller(Elevator* elevator) {
    if ( scheduler_state == 0 || scheduler_state == 3 || scheduler_state == 6 ) {

        if ( elevator->position == elevator->destination && elevator->direction == STILL ) {
            elevator->door_open = true;
        } else {
            elevator->door_open = false;
        }

    }
}

void elevator_speed_controller(Elevator* elevator) {
    if ( scheduler_state == 1 || scheduler_state == 4 || scheduler_state == 7 ) {

        if ( elevator->position > elevator->destination ) {
            elevator->direction = DOWN;
        } else if ( elevator->position < elevator->destination ) {
            elevator->direction = UP;
        } else {
            elevator->direction = STILL;
        }

    }
}

void move_elevator(Elevator* elevator) {
    if ( scheduler_state == 2 || scheduler_state == 5 || scheduler_state == 8 ) {

        if ( elevator->direction == UP ) {
            elevator->position += 1;
        } else if ( elevator->direction == DOWN ) {
            elevator->position -= 1;
        }

    }
}

void get_destination(Elevator* elevator) {
    if ( scheduler_state == 9 ) {

        if ( elevator->position == elevator->destination ) {
            havoc(elevator->destination);
        }

    }
}

int main() {
    while ( 1 ) {
        elevator_door_monitor = elevator1.door_open;
        elevator_direction_monitor = elevator1.direction;

        elevator_door_controller(&elevator1);
        elevator_speed_controller(&elevator1);
        move_elevator(&elevator1);
        get_destination(&elevator1);
        scheduler();
    }
    return 0;
}
