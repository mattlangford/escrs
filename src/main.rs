struct State {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
}
struct Mass {
    m: f64,
}

struct Force {
    fx: f64,
    fy: f64,
}

#[derive(Default)]
struct Components {
    state: Vec<(usize, State)>,
    mass: Vec<(usize, Mass)>,
    force: Vec<(usize, Force)>,
}

#[derive(Default)]
struct Entity {
    id: usize,
    state: Option<usize>,
    mass: Option<usize>,
    force: Option<usize>,
}

fn push_and_index<T>(vec: &mut Vec<T>, t: T) -> usize {
    vec.push(t);
    vec.len() - 1
}

trait EntityAccess<T> {
    fn get(&self) -> Option<usize>;
    fn set(&mut self, i: usize);
    fn reset(&mut self) -> Option<usize>;
}
trait ComponentAccess<T> {
    fn get(&self, i: usize) -> &T;
    fn get_mut(&mut self, i: usize) -> &mut T;
    fn add(&mut self, t: T, id: usize) -> usize;
}

#[macro_export]
macro_rules! add_accessors {
    ($x:ident, $t:ty) => {
        impl EntityAccess<$t> for Entity {
            fn get(&self) -> Option<usize> {
                self.$x
            }
            fn set(&mut self, i: usize) {
                self.$x = Some(i);
            }
            fn reset(&mut self) -> Option<usize> {
                let ret = self.$x;
                self.$x = None;
                ret
            }
        }

        impl ComponentAccess<$t> for Components {
            fn get(&self, i: usize) -> &$t {
                &self.$x[i].1
            }
            fn get_mut(&mut self, i: usize) -> &mut $t {
                &mut self.$x[i].1
            }
            fn add(&mut self, t: $t, id: usize) -> usize {
                push_and_index(&mut self.$x, (id, t))
            }
        }
    };
}

add_accessors!(state, State);
add_accessors!(mass, Mass);
add_accessors!(force, Force);

#[derive(Default)]
struct Manager {
    entities: Vec<Entity>,
    components: Components,
}

struct EntityBuilder<'a> {
    e: &'a mut Entity,
    c: &'a mut Components,
}

impl EntityBuilder<'_> {
    fn add<T>(&mut self, t: T) -> &mut Self
    where
        Entity: EntityAccess<T>,
        Components: ComponentAccess<T>,
    {
        <Entity as EntityAccess<T>>::set(self.e, self.c.add(t, self.e.id));
        self
    }
}

impl Manager {
    fn add_entity(&mut self) -> EntityBuilder<'_> {
        let i = push_and_index(&mut self.entities, Entity::default());
        let e = self.entities.last_mut().unwrap();
        e.id = i;
        EntityBuilder {
            e: self.entities.last_mut().unwrap(),
            c: &mut self.components,
        }
    }
}

impl Entity {
    fn get<'a, T>(&self, v: &'a Vec<(usize, T)>) -> Option<&'a T>
    where
        Entity: EntityAccess<T>,
    {
        <Entity as EntityAccess<T>>::get(self).map(|i| &v[i].1)
    }
    fn get_mut<'a, T>(&self, v: &'a mut Vec<(usize, T)>) -> Option<&'a mut T>
    where
        Entity: EntityAccess<T>,
    {
        <Entity as EntityAccess<T>>::get(self).map(move |i| &mut v[i].1)
    }
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

fn print_state(m: &mut Manager) {
    let c = &mut m.components;
    for e in &m.entities {
        if let (Some(state), Some(mass)) = (e.get_mut(&mut c.state), e.get(&c.mass)) {
            state.x = 123.0;
            println!(
                "2 id:{} pos: {:.3},{:.3} vel: {:.3},{:.3} mass: {:.3}",
                e.id, state.x, state.y, state.vx, state.vy, mass.m
            )
        }
    }
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
    print_state(&mut m);
}
