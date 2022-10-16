# Kapacitet

Kapacitet is a little tool for roadmap and capacity planning. It
takes your roadmap items as input, calculates an urgency score
for each of them and assigns individual contributors to them.
Essentially, you can use it to answer the question _"what do we
have the capacity to work on right now?"_.

## Input

- Individual contributors
    - Name
    - Seniority (1-5)
- Roadmap items
    - Name
    - Start date
    - Target date
    - Estimated complexity (1-5)
    - Estimated value (1-5)

## Output

Kapacitet calculates an urgency score for each roadmap item
based on the inputs `start date`, `target_date`, `estimated
complexity` and `estimated value`. Roadmap items are then ordered
in descending order based on their calculated urgency score.

The respective components of the complexity score are weighted as
follows:

```rust
static DURATION_FACTOR: f64 = 0.1; // 10%
static COMPLEXITY_FACTOR: f64 = 0.3; // 30%
static TARGET_DATE_FACTOR: f64 = 0.2; // 20%
static VALUE_FACTOR: f64 = 0.4; // 40%
```

Individual contributors are assigned to roadmap items after the
calculation of the urgency score. The algorithm for assigning
individual contributors follows two simple rules:

1. Even out the individual contributor seniority across items.
2. Assign an appropriate number of contributors depending on
   urgency thresholds.

The first one aims to reduce risk by not over-assigning senior
members of the team to a subset of the roadmap items. The goal of the
second one is to assign the right amount of people to any given item.

The output is printed to stdout in the form of a csv:

```
name,start date,target date,urgency (0-1),contributors
allan,2023-01-01,2023-02-02,0.34,IC1;IC2
```

## Usage

Grab a pre-built binary from
the [release page](https://github.com/simpajjj/kapacitet/releases)
and go wild. Apple Silicon is currently the only support target
architecture.

If you want to run Kapacitet on any other architecture you'll
have to build it from source. Kapacitet is built with Rust, so
please refer to [this](https://www.rust-lang.org/tools/install) on how
to get started.

Kapacitet lets you add contributors and roadmap items directly from 
the command line or from CSV files. 

The contributors CSV should look like this:

```
name,seniority
Contributor Uno,5
```

with `seniority` being an integer between 1 and 5.

The roadmap CSV should look like this:

```
name,estimated_complexity,estimated_value,start_date,target_date
MVP,4,5,2022-06-01,2022-11-01
```

with `estimated_complexity` and `estimated_value` being integers 
between 1 and 5 and `start_date` and `target_date` following the 
date format `YYYY-mm-dd`.

## TODO

Kapacitet currently lacks the capability to schedule future
roadmap items. Items which there isn't any capacity for right now
are simply discarded. The workaround for now is to re-run the
planning once a roadmap item has finished and capacity has been
freed up, or when some input has been changed, forcing
re-prioritization.
