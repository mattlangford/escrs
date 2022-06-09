#![macro_use]

#[macro_export]
macro_rules! generate {
    (impl $x:ident, $t:ty) => {
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
                generate!(impl [<$t:lower>], $t);
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
                    let size = self.entities.len();
                    self.entities.push(Entity::default());
                    let e = self.entities.last_mut().unwrap();
                    e.id = size;
                    Builder {
                        e: self.entities.last_mut().unwrap(),
                        c: &mut self.components,
                    }
               }
            }
        }
    };
}

#[macro_export]
macro_rules! test {
    // TODO: This is is really annoying that it requires paste!() to work...
    (&mut $type:ty) => {
        paste!($($e.[<$type:lower>].map(|i| &mut $m.components.[<$type:lower>][i].1),)*)
    };
}

#[macro_export]
macro_rules! run_system {
    (@expand $m:ident $e:ident $t:ty) => {
        paste!($e.[<$t:lower>].map(|i| &$m.components.[<$t:lower>][i].1))
    };
    (@expand_mut $m:ident $e:ident $t:ty) => {
        paste!($e.[<$t:lower>].map(|i| &mut $m.components.[<$t:lower>][i].1))
    };

    // Base case, no more types
    (@parse [$m:ident $e:ident $body:block] [$($ns:ident)*] [$($cs:expr)*]) => {
        if let ($(Some($ns),)*) = ($($cs,)*) { $body }
    };

    // Matches ...name: &mut Type, ... and then passes along info to the next recursion
    (@parse [$m:ident $e:ident $body:block] [$($ns:ident)*] [$($cs:expr)*] $name:ident: &mut $t:ty, $($rest:tt)*) => {
        run_system!(@parse [$m $e $body] [$($ns)* $name] [$($cs)* run_system!(@expand_mut $m $e $t)] $($rest)*)
    };

    // Matches ...name: &Type, ... and then passes along info to the next recursion
    (@parse [$m:ident $e:ident $body:block] [$($ns:ident)*] [$($cs:expr)*] $name:ident: &$t:ty, $($rest:tt)*) => {
        run_system!(@parse [$m $e $body] [$($ns)* $name] [$($cs)* run_system!(@expand $m $e $t)] $($rest)*)
    };

    ($m: ident, |$e:ident, ($($rest:tt)*)| $body:block) => {
        for $e in &$m.entities {
            run_system!(@parse [$m $e $body] [] [] $($rest)*,)
        }
    };
}

#[macro_export]
macro_rules! component_iter {
    ($m: ident, $t:ty) => {
        paste!(
            $m.components.[<$t:lower>].iter().map(|(_, c)| c)
        )
    }
}
