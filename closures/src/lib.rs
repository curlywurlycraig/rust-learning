use std::collections::HashMap;
use std::hash::Hash;

pub struct Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: Eq + Hash + Copy,
    V: Copy
{
    calculation: T,
    values: HashMap<U, V>
}

impl<T, U, V> Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: Eq + Hash + Copy,
    V: Copy
{
    pub fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            values: HashMap::new()
        }
    }

    pub fn call(&mut self, arg: U) -> V {
        match self.values.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}
