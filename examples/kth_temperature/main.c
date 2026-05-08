unsigned short in_kelvin; // integer type only matters if bit-vectors are used
int out_celsius;


/*
 * Adapted from the temperature example from
 * Amilon, Lidström, Gurov: 10.1007/978-3-031-19849-6_2
 * with a "heater" that is triggered when temperature is below freezing
 */

/*
-- #################### FALSE PROPERTIES ####################

-- by disproving these properties, we know that the implications below does not hold vacuously
-- the counterexamples demonstrate that there are runs where the temperature is eventually
-- always positive/negative
-- I have not found a provable analogue to this in LTL
LTLSPEC G(!G("in_kelvin" > 273))
LTLSPEC G(!G("in_kelvin" < 273))

-- #################### TRUE PROPERTIES ####################

-- unbounded properties
LTLSPEC
  G(G("in_kelvin" > 273) -> F(G("out_celsius" > 0)))
LTLSPEC
  G(G("in_kelvin" < 273) -> F(G("out_celsius" < 0)))

--terminating bounded
LTLSPEC
  G(G[0,1]("in_kelvin" > 273) -> F("out_celsius" > 0))
LTLSPEC
  G(G[0,1]("in_kelvin" < 273) -> F("out_celsius" < 0))

LTLSPEC
  G(G("in_kelvin" > 273) -> F[0,1](G("out_celsius" > 0)))
LTLSPEC
  G(G("in_kelvin" < 273) -> F[0,1](G("out_celsius" < 0)))
*/

int convert_temp(int k) {
    int res = k;
    res = res - 273;
    return res;
}

/* CHRONOS_SCHEDULE
hyperperiod: 1;
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

int main() {
    while ( 1 ) {
        havoc(in_kelvin);
        task1();
        scheduler();
    }
}