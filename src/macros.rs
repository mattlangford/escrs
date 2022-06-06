#![macro_escape]

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
