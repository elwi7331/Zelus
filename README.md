# Zelus

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://github.com/elwi7331/Zelus/blob/main/documentation/figures/zelus_dark_mode.svg">
  <source media="(prefers-color-scheme: light)" srcset="https://github.com/elwi7331/Zelus/blob/main/documentation/figures/zelus_light_mode.svg">
  <img src="https://github.com/elwi7331/Zelus/blob/main/documentation/figures/zelus_light_mode.svg" width="200" align="right">
</picture>

Created as part of a master's thesis _Formal Specification and Verification of Real Time Properties of C programs using nuXmv_,
Zelus is a tool for verifying _Real Time Properties_ on a subset of _Embedded Real Time Programs_ written in C.
It utilizes _Kratos2_ to translate C programs with scheduling constraints on _continuous time_ to _Timed Transition Systems_ on which properties can be checked with the _Timed Extension of nuXmv_.

## Dependencies
- _nuXmv_.
- For the automatic model generation from C files:
  - the `c2kratos` variable in [`scripts/config`](./scripts/config) must point to the `c2kratos.py` script, as provided with Kratos. Note that the script must remain bundled with its dependencies.
  - The `python` variable must point to a python executable (I recommend to use a virtual environment), with packages from [`scripts/requirements.txt`](./scripts/requirements.txt) installed. The script does not work with the latest versions of Python.
  - Variable `kratos` should point to the executable of _kratos_.
  - Build `chronos` using make.
- For the model checking scripts to work, the variable `nuxmv` in [`scripts/config`](./scripts/config) must point to a _nuXmv_ binary.

## Embedded C to nuXmv
As input, Zelus accepts C programs were tasks are represented by functions. The main function should repeatedly call *all tasks*, i.e.

    int main() {
        while ( 1 ) {
            task_1();
            ...
            task_n();
        }
        return 0;
    }

For exposing program variables to later _LTL_-specifications, I have found declaring them as global is the most convenient approach.

The scheduling of tasks is specified in a schedule, the grammar of which is described below. Note that the ordering of jobs matters. Jobs will be performed in order according to their scheduling. It is thereby not legal for a job's time interval to precede any before it.

The schedule can be supplied either directly in the C code, or in a separate file. In the former case, it should be contained within a block comment (`/*<schedule>*/`) as its sole content. At most one schedule can be supplied in a C file, and its location does not matter.

Calling [`chronos`](scripts/chronos) yields the annotations (to stdout) given by the schedule file (C or separate file, passed as argument to [`chronos`](scripts/chronos)). These are given in tow parts:
- Annotations to the C file (order of execution)
- Annotations to the _nuXmv_ file (timing)

After the C annotations have been added, the C file be converted to _nuxmv_ using _kratos_. Using the scripts [`c2k2`](scripts/c2k2.sh) and [`k22nuxmv`](scripts/k22nuxmv.sh) ensures that the correct options are supplied. After conversion, the _nuXmv_ annotations from `chronos` are introduced.

### Schedule Grammar
    <interval> ::= <l_delim> <time_point> "," <time_point> <r_delim>`
    <l_delim> ::= "(" | "["
    <r_delim> ::= ")" | "]"
    <time_point> ::= <natural> | "f'" <natural> "/" <natural> | <natural> "." <natural>

    <job> ::= <task_identifier> ":" <interval> ";"
    <job_list> ::= <job> | <job> <job_list> 

    <schedule> ::= "CHRONOS_SCHEDULE" "hyperperiod" ":" <time_point> ";" <job_list>


## Usage
- Get C and _nuXmv_ annotations using [`chronos`](scripts/chronos) `<c_or_schedule_file>`
- insert annotations in C file
- convert C file to K2 intermediate format [`c2k2`](scripts/c2k2.sh) `main.c main.k2`
- convert K2 file to xmv using [`k22nuxmv`](scripts/k22nuxmv.sh) `main.k2 main_timed.xmv`
- insert annotations in xmv file
- introduce _LTL_-specifications to the _nuXmv_ file
- [`check_model`](scripts/check_model.sh) `<model.xmv> [options]`: initialize model and check its specifications.
  - `-v` verbose, show all output when checking properties on timed models

### Naming Conventions
[`check_model`](scripts/check_model.sh) using the naming convention:
- `.xmv`: untimed model containing only variables of _finite domain_ types.
- `_infinite.xmv`: untimed model contaning variables of _infinite domain_.
- `_timed.xmv`: timed models.

For timed models, the `_timed` suffix should always be used for the script to automatically work. Note however that before _chronos_ annotation, models are either _finite_ or _infinite_ state untimed models, on which the other suffixes apply.
