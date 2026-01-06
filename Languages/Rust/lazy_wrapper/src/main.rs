use std::cell::{OnceCell, RefCell};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

struct Cached<T, F> {
    data: OnceCell<T>,
    compute: RefCell<Option<F>>,
}

impl<T, F> Cached<T, F>
where
    F: FnOnce() -> T,
{
    fn new(compute: F) -> Self {
        Self {
            data: OnceCell::new(),
            compute: RefCell::new(Some(compute)),
        }
    }

    fn get(&self) -> &T {
        self.data.get_or_init(|| {
            let f = self.compute.borrow_mut().take().unwrap();
            f()
        })
    }
}
fn main() {
    let cache = Cached::new(|| 1 + 1);

    println!("{}", *cache.get());
    println!("{}", *cache.get()); // cached
}

// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::hash::Hash;

// struct Cached<K, V, F>
// where
//     K: Eq + Hash + Clone,
//     F: Fn(&K) -> V,
// {
//     data: RefCell<HashMap<K, V>>,
//     compute: F,
// }

// impl<K, V, F> Cached<K, V, F>
// where
//     K: Eq + Hash + Clone,
//     F: Fn(&K) -> V,
// {
//     fn new(compute: F) -> Self {
//         Self {
//             data: RefCell::new(HashMap::new()),
//             compute,
//         }
//     }

//     fn get(&self, key: K) -> std::cell::Ref<'_, V> {
//         if !self.data.borrow().contains_key(&key) {
//             let value = (self.compute)(&key);
//             self.data.borrow_mut().insert(key.clone(), value);
//         }

//         std::cell::Ref::map(self.data.borrow(), |m| {
//             m.get(&key).unwrap()
//         })
//     }
// }

// fn main() {
//     let cache = Cached::new(|x: &u32| x * 2);

//     println!("{}", *cache.get(10));
//     println!("{}", *cache.get(10)); // cached
// }

// use std::cell::RefCell;

// struct Cached<T, F>
// where
//     F: Fn() -> T,
// {
//     value: RefCell<Option<T>>,
//     compute: F,
// }

// impl<T, F> Cached<T, F>
// where
//     F: Fn() -> T,
// {
//     fn new(compute: F) -> Self {
//         Self {
//             value: RefCell::new(None),
//             compute,
//         }
//     }

//     fn get(&self) -> std::cell::Ref<'_, T> {
//         if self.value.borrow().is_none() {
//             let value = (self.compute)();
//             *self.value.borrow_mut() = Some(value);
//         }

//         std::cell::Ref::map(self.value.borrow(), |v| {
//             v.as_ref().unwrap()
//         })
//     }
// }

// static CONFIG: OnceCell<String> = OnceCell::new();

// fn config() -> &'static str {
//     CONFIG.get_or_init(|| {
//         // expensive computation
//         "hello".to_string()
//     })
// }

// use std::sync::LazyLock;

// static CONFIG: LazyLock<String> = LazyLock::new(|| "hello".to_string());

fn fib(n: u64) -> u64 {
    static CACHE: OnceLock<Mutex<HashMap<u64, u64>>> = OnceLock::new();

    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut map = cache.lock().unwrap();

    if let Some(v) = map.get(&n) {
        return *v;
    }

    let result = { if n <= 1 { n } else { fib(n - 1) + fib(n - 2) } };

    map.insert(n, result);
    result
}

// macro_rules! memoized {
//     (
//         fn $name:ident($arg:ident : $arg_ty:ty) -> $ret:ty $body:block
//     ) => {
//         fn $name($arg: $arg_ty) -> $ret
//         where
//             $arg_ty: std::hash::Hash + Eq + Clone,
//             $ret: Clone,
//         {
//             use std::collections::HashMap;
//             use std::sync::OnceLock;
//             use std::sync::Mutex;

//             static CACHE: OnceLock<Mutex<HashMap<$arg_ty, $ret>>> = OnceLock::new();

//             let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));

//             let mut map = cache.lock().unwrap();
//             if let Some(v) = map.get(&$arg) {
//                 return v.clone();
//             }

//             let result = (|| $body)();
//             map.insert($arg.clone(), result.clone());
//             result
//         }
//     };
// }

// memoized! {
//     fn fib(n: u64) -> u64 {
//         if n <= 1 {
//             n
//         } else {
//             fib(n - 1) + fib(n - 2)
//         }
//     }
// }
