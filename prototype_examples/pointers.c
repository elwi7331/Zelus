/*
    When translated into nuXmv, an Integer array is used to model memory.
*/

typedef struct {
    int first;
    int second;
} Values;

typedef enum {
    a,
    b,
} Scheduled;

Values values;
Scheduled scheduled = a;

void task_a(Scheduled* scheduled, Values* values) {
    if ( *scheduled == a ) {
        values->first += 1;
        *scheduled  = b;
    }
}

void task_b(Scheduled* scheduled, Values* values) {
    if ( *scheduled == b ) {
        values->second += 2;
        *scheduled = a;
    }
}

int main() {
    values.first = 0;
    values.second = 0;
    while ( 1 ) {
        task_a(&scheduled, &values);
        task_b(&scheduled, &values);
    }
    return 0;
}
