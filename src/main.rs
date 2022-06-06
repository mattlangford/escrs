use paste::paste;

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

fn push_and_index<T>(vec: &mut Vec<T>, t: T) -> usize {
    vec.push(t);
    vec.len() - 1
}

trait EntityAccess<T> {
    fn set(&mut self, i: usize) {
        *self.get_mut() = Some(i);
    }
    fn reset(&mut self) -> Option<usize> {
        let ret = *self.get();
        *self.get_mut() = None;
        ret
    }

    fn get(&self) -> &Option<usize>;
    fn get_mut(&mut self) -> &mut Option<usize>;
}
trait ComponentAccess<T> {
    fn get(&self, i: usize) -> &T {
        &self.raw_get()[i].1
    }
    fn get_mut(&mut self, i: usize) -> &mut T {
        &mut self.raw_get_mut()[i].1
    }
    fn add(&mut self, t: T, id: usize) -> usize {
        push_and_index(self.raw_get_mut(), (id, t))
    }

    fn raw_get(&self) -> &Vec<(usize, T)>;
    fn raw_get_mut(&mut self) -> &mut Vec<(usize, T)>;
}

trait EntityBuilder<E, C> {
    fn add<T>(&mut self, t: T) -> &mut Self
    where
        E: EntityAccess<T>,
        C: ComponentAccess<T>;
}

#[macro_export]
macro_rules! add_accessors {
    ($x:ident, $t:ty) => {
        impl EntityAccess<$t> for Entity {
            fn get(&self) -> &Option<usize> {
                &self.$x
            }
            fn get_mut(&mut self) -> &mut Option<usize> {
                &mut self.$x
            }
        }

        impl ComponentAccess<$t> for Components {
            fn raw_get(&self) -> &Vec<(usize, $t)> {
                &self.$x
            }
            fn raw_get_mut(&mut self) -> &mut Vec<(usize, $t)> {
                &mut self.$x
            }
        }
    };
}

#[macro_export]
macro_rules! generate {
    ($($t:ty),*) => {
        paste! {
            #[derive(Debug, Default)]
            struct Components {
                $([<$t:lower>]: Vec<(usize, $t)>,)*
            }

            #[derive(Debug, Default)]
            struct Entity {
                id: usize,
                $([<$t:lower>]: Option<usize>,)*
            }

            $(
                add_accessors!([<$t:lower>], $t);
            )*


            struct Builder<'a> {
                e: &'a mut Entity,
                c: &'a mut Components,
            }
            impl<'a> EntityBuilder<Entity, Components> for Builder<'a> {
                fn add<T>(&mut self, t: T) -> &mut Self where Entity: EntityAccess<T>, Components: ComponentAccess<T> {
                    <Entity as EntityAccess<T>>::set(self.e, self.c.add(t, self.e.id));
                    self
                }
            }

            #[derive(Debug, Default)]
            struct Manager {
                entities: Vec<Entity>,
                components: Components,
            }

            impl Manager {
               fn add_entity(&mut self) -> impl EntityBuilder<Entity, Components> + '_ {
                    let i = push_and_index(&mut self.entities, Entity::default());
                    let e = self.entities.last_mut().unwrap();
                    e.id = i;
                    Builder {
                        e: self.entities.last_mut().unwrap(),
                        c: &mut self.components,
                    }
                }
            }
        }
    };
}

macro_rules! run_system {
    ($m: ident, ($e:ident $(,$name:ident: $type: ty)*) { $($body:expr);* }) => {
        for $e in &$m.entities {
            if let ( $(Some($name),)* ) = (
                $(EntityAccess::<$type>::get($e).map(|i| ComponentAccess::<$type>::get(&$m.components, i)),)*
            ) {
                $($body);*
            }
        }
    };

    ($m: ident, mut ($e:ident $(,$name:ident: $type: ty)*) { $($body:expr);* }) => {
        for $e in &$m.entities {
            paste!(
            if let ( $(Some($name),)* ) = (
                // TODO: This is is really annoying that it requires paste!() to work...
                $($e.[<$type:lower>].map(|i| &mut $m.components.[<$type:lower>][i].1),)*
            ) {
                $($body);*
            }
            )
        }
    };
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
