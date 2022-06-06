pub trait EntityAccess<T> {
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
pub trait ComponentAccess<T> {
    fn get(&self, i: usize) -> &T {
        &self.raw_get()[i].1
    }
    fn get_mut(&mut self, i: usize) -> &mut T {
        &mut self.raw_get_mut()[i].1
    }
    fn add(&mut self, t: T, id: usize) -> usize {
        self.raw_get_mut().push((id, t));
        self.raw_get().len() - 1
    }

    fn raw_get(&self) -> &Vec<(usize, T)>;
    fn raw_get_mut(&mut self) -> &mut Vec<(usize, T)>;
}

pub trait EntityBuilder<E, C> {
    fn add<T>(&mut self, t: T) -> &mut Self
    where
        E: EntityAccess<T>,
        C: ComponentAccess<T>;
}
