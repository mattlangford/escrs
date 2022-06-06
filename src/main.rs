use paste::paste;

mod macros;
mod traits;

use macros::*;
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
    run_system!(m, (e, s: State, m: Mass) {
        println!(
            "before id: {} pos: ({:.3},{:.3}) vel: ({:.3},{:.3}) mass: {}",
            e.id, s.x, s.y, s.vx, s.vy, m.m
        )
    });
    run_system!(m, mut (e, s: State, m: Mass) {
        s.x = 123.0;
        println!(
            "id: {} pos: ({:.3},{:.3}) vel: ({:.3},{:.3}) mass: {}",
            e.id, s.x, s.y, s.vx, s.vy, m.m
        )
    });
}

fn update_state2(m: &mut Manager, dt: f64) {
    for (n, state) in m.components.state.iter_mut() {
        state.x += state.vx * dt;
        state.y += state.vy * dt;
        if let Some(mass_i) = m.entities[*n].mass {
            for (_, force_other) in &m.components.force {
                let (_, mass) = &m.components.mass[mass_i];
                state.vx += dt * force_other.fx / mass.m;
                state.vy += dt * force_other.fy / mass.m;
            }
        }
    }
}

/*
fn update_state(m: &mut Manager, dt: f64) {
    let c = &mut m.components;
    for e in &m.entities {
        if let (Some(state), Some(mass)) = (e.get_mut(&mut c.state), e.get(&c.mass)) {
            state.x = 123.0;
            println!(
                "2 id:{} pos: {:.3},{:.3} vel: {:.3},{:.3} mass: {:.3}",
                e.id, state.x, state.y, state.vx, state.vy, mass
            )
        }
    }
}*/

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
}
