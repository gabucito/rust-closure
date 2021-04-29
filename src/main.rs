use std::collections::HashMap;
use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
    hash: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // When we initialize the Cacher the value will be none
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
            hash: HashMap::new(),
        }
    }

    // When it asks for a result
    // If the Cacher has Some value stored then return that value
    // If the value is None then calculate the value, store it and return it
    // ! Caveat the Cacher struct can only hold 1 value, if the struct was first called with intensity 10, even if we call it next with intensity 15 it will return the stored value of 10
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg); // ! I don't understand this
                self.value = Some(v);
                v
            }
        }
    }

    fn hash(&mut self, arg: u32) -> u32 {
        let result = self.hash.get(&arg);
        match result {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(arg);
                self.hash.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num: u32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.hash(intensity));
        println!("Next, do {} situps!", expensive_closure.hash(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.hash(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.hash(1);
        let v2 = c.hash(2);

        assert_eq!(v2, 2);
    }
}
