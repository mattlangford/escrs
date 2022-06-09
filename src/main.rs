use paste::paste;

mod macros;
mod traits;

use traits::*;

#[derive(Debug, Default)]
struct State {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
}

#[derive(Debug, Default)]
struct Mass {
    m: f64,
}

#[derive(Debug, Default)]
struct Force {
    fx: f64,
    fy: f64,
}

generate!(State, Mass, Force);

fn print_state(m: &mut Manager) {
    run_system!(m, |e, (s: &State, m: &Mass)| {
        println!(
            "id: {} pos: ({:.3},{:.3}) vel: ({:.3},{:.3}) mass: {}",
            e.id, s.x, s.y, s.vx, s.vy, m.m
        )
    });
}

fn update_state(m: &mut Manager, dt: f64) {
    run_system!(m, |e, (state: &mut State)| {
        state.x += state.vx * dt;
        state.y += state.vy * dt;
    });
    run_system!(m, |e, (state: &mut State, mass: &Mass)| {
        for force in component_iter!(m, Force) {
            state.vx += dt * force.fx / mass.m;
            state.vy += dt * force.fy / mass.m;
        }
    });
}

fn main() {
    let mut m = Manager::default();
    m.add_entity()
        .add(State {
            x: 0.0,
            y: 10.0,
            vx: 1.0,
            vy: 0.0,
        })
        .add(Mass { m: 10.0 });
    m.add_entity().add(State {
        x: 100.0,
        y: 100.0,
        vx: 100.0,
        vy: 100.0,
    });
    m.add_entity().add(Force { fx: 0.0, fy: -1.0 });
    m.add_entity()
        .add(State {
            x: 0.0,
            y: 20.0,
            vx: 2.0,
            vy: 0.0,
        })
        .add(Mass { m: 20.0 });

    print_state(&mut m);
    update_state(&mut m, 1.0);
    print_state(&mut m);
}
