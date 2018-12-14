use std::collections::HashMap;
use std::hash::Hash;

//#[derive(PartialEq,Eq,Hash)]
pub struct Cacher<T, U, V>
where
    T: Fn(U) -> (V),
    U: Hash + Eq + Copy,
    V: Copy,
{
    calculation: T,
    values: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
    T: Fn(U) -> (V),
    U: Hash + Eq + Copy,
    V: Copy,
{
    pub fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }
    pub fn value(&mut self, arg: U) -> V {
        match self.values.get(&arg) {
            Some(v) => return *v,
            None => (),
        }
        let v = (self.calculation)(arg);
        self.values.insert(arg, v);
        v
    }
    pub fn values_ref(&self) -> &HashMap<U, V> {
        &self.values
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn set_value() {
        let mut t_cacher = Cacher::new(|a| a);
        let c = t_cacher.value(42);
        assert_eq!(42, c);
    }
    #[test]
    fn diff_values() {
        let mut t_cacher = Cacher::new(|a| a);
        let c1 = t_cacher.value(1);
        let c2 = t_cacher.value(2);
        assert_eq!(c2, 2)
    }
}
