use easybench::{bench, bench_gen_env};
use std::collections::HashSet;

use internment::{ArcIntern, Intern, LocalIntern};

#[derive(Eq, PartialEq, Hash)]
pub struct TestStruct(String,u64);

// Trait so same test can run for ArcIntern and arc_interner::ArcIntern
trait InternLike<T> {
    fn new(val: T) -> Self;
    fn num_objects_interned() -> usize;
}

impl<T: 'static + std::cmp::Eq + std::hash::Hash + Send + Sync> InternLike<T> for ArcIntern<T> {
    fn new(val: T) -> Self {
        ArcIntern::<T>::new(val)
    }
    fn num_objects_interned() -> usize {
        ArcIntern::<T>::num_objects_interned()
    }
}

impl<T: 'static + std::cmp::Eq + std::hash::Hash + Send + Sync> InternLike<T> for arc_interner::ArcIntern<T> {
    fn new(val: T) -> Self {
        arc_interner::ArcIntern::<T>::new(val)
    }
    fn num_objects_interned() -> usize {
        arc_interner::ArcIntern::<T>::num_objects_interned()
    }
}

// Quickly create and destroy a small number of interned objects from
// multiple threads.
fn multithreading<T: InternLike<TestStruct>>() {
    use std::thread;
    let mut thandles = vec![];
    for _i in 0..10 {
        thandles.push(thread::spawn(|| {
            for _i in 0..100_000 {
                let _interned1 = T::new(TestStruct("foo".to_string(), 5));
                let _interned2 = T::new(TestStruct("bar".to_string(), 10));
            }
        }));
    }
    for h in thandles.into_iter() {
        h.join().unwrap()
    }

    assert_eq!(T::num_objects_interned(), 0);
}

fn main() {
    Intern::new(0i64);
    LocalIntern::new(0i64);
    ArcIntern::new(0i64);
    arc_interner::ArcIntern::new(0i64);
    let s1: HashSet<_> = (1..3000)
        .map(|i| format!("hello this is a pretty long string {:500}", i))
        .collect();
    let s2: HashSet<_> = (1..1000)
        .map(|i| format!("hello this is a pretty long string also {:500}", i))
        .collect();
    for x in 0u8..=255 {
        Intern::new(x);
        LocalIntern::new(x);
        ArcIntern::new(x);
        arc_interner::ArcIntern::new(x);
    }

    {
        fn mkset(s: &mut HashSet<String>) -> HashSet<String> {
            let mut n = HashSet::new();
            for x in s.iter() {
                n.insert(x.clone());
            }
            n
        }
        mkset(&mut s1.clone());
        println!("String::new {}", bench_gen_env(|| s1.clone(), mkset));
        fn rmset(s: &mut (HashSet<String>, HashSet<String>)) {
            for x in s.1.iter() {
                s.0.remove(x);
            }
        }
        let s1: HashSet<_> = s1.clone();
        let s2: HashSet<_> = s2.clone();
        println!(
            "String::compare/hash {}",
            bench_gen_env(|| (s1.clone(), s2.clone()), rmset)
        );
    }
    println!();

    println!(
        "Intern<i64>::new {}",
        bench_gen_env(|| rand::random::<i64>(), |x| Intern::new(*x))
    );
    println!(
        "Intern<u8>::new {}",
        bench_gen_env(|| rand::random::<u8>(), |x| Intern::new(*x))
    );
    {
        fn mkset(s: &mut HashSet<String>) -> HashSet<Intern<String>> {
            let mut n = HashSet::new();
            for x in s.iter() {
                n.insert(Intern::new(x.clone()));
            }
            n
        }
        mkset(&mut s1.clone());
        println!(
            "Intern<String>::new {}",
            bench_gen_env(|| s1.clone(), mkset)
        );
        fn mksetfrom(s: &mut HashSet<String>) -> HashSet<Intern<String>> {
            let mut n = HashSet::new();
            for x in s.iter() {
                n.insert(Intern::from(x as &str));
            }
            n
        }
        println!(
            "Intern<String>::from {}",
            bench_gen_env(|| s1.clone(), mksetfrom)
        );
        fn rmset(s: &mut (HashSet<Intern<String>>, HashSet<Intern<String>>)) {
            for x in s.1.iter() {
                s.0.remove(x);
            }
        }
        let s1: HashSet<_> = s1.iter().cloned().map(|s| Intern::new(s)).collect();
        let s2: HashSet<_> = s2.iter().cloned().map(|s| Intern::new(s)).collect();
        println!(
            "Intern<String>::compare/hash {}",
            bench_gen_env(|| (s1.clone(), s2.clone()), rmset)
        );
    }
    let i = Intern::new(7i64);
    println!("Intern<i64>::clone {}", bench(|| i.clone()));
    println!();

    println!(
        "LocalIntern<i64>::new {}",
        bench_gen_env(|| rand::random::<i64>(), |x| LocalIntern::new(*x))
    );
    println!(
        "LocalIntern<u8>::new {}",
        bench_gen_env(|| rand::random::<u8>(), |x| LocalIntern::new(*x))
    );
    {
        fn mkset(s: &mut HashSet<String>) -> HashSet<LocalIntern<String>> {
            let mut n = HashSet::new();
            for x in s.iter() {
                n.insert(LocalIntern::new(x.clone()));
            }
            n
        }
        mkset(&mut s1.clone());
        println!(
            "LocalIntern<String>::new {}",
            bench_gen_env(|| s1.clone(), mkset)
        );
        fn mksetfrom(s: &mut HashSet<String>) -> HashSet<LocalIntern<String>> {
            let mut n = HashSet::new();
            for x in s.iter() {
                n.insert(LocalIntern::from(x as &str));
            }
            n
        }
        println!(
            "LocalIntern<String>::from {}",
            bench_gen_env(|| s1.clone(), mksetfrom)
        );
        fn rmset(s: &mut (HashSet<LocalIntern<String>>, HashSet<LocalIntern<String>>)) {
            for x in s.1.iter() {
                s.0.remove(x);
            }
        }
        let s1: HashSet<_> = s1.iter().cloned().map(|s| LocalIntern::new(s)).collect();
        let s2: HashSet<_> = s2.iter().cloned().map(|s| LocalIntern::new(s)).collect();
        println!(
            "LocalIntern<String>::compare/hash {}",
            bench_gen_env(|| (s1.clone(), s2.clone()), rmset)
        );
    }
    let i = LocalIntern::new(7i64);
    println!("LocalIntern<i64>::clone {}", bench(|| i.clone()));
    println!();

    println!(
        "ArcIntern<i64>::new {}",
        bench_gen_env(|| rand::random::<i64>(), |x| ArcIntern::new(*x))
    );
    println!(
        "ArcIntern<u8>::new {}",
        bench_gen_env(|| rand::random::<u8>(), |x| ArcIntern::new(*x))
    );
    {
        fn mkset(s: &mut HashSet<String>) -> HashSet<ArcIntern<String>> {
            let mut n = HashSet::new();
            for x in s.iter() {
                n.insert(ArcIntern::new(x.clone()));
            }
            n
        }
        mkset(&mut s1.clone());
        println!(
            "ArcIntern<String>::new {}",
            bench_gen_env(|| s1.clone(), mkset)
        );
        fn mksetfrom(s: &mut HashSet<String>) -> HashSet<ArcIntern<String>> {
            let mut n = HashSet::new();
            for x in s.iter() {
                n.insert(ArcIntern::from(x as &str));
            }
            n
        }
        println!(
            "ArcIntern<String>::from {}",
            bench_gen_env(|| s1.clone(), mksetfrom)
        );
        fn rmset(s: &mut (HashSet<ArcIntern<String>>, HashSet<ArcIntern<String>>)) {
            for x in s.1.iter() {
                s.0.remove(x);
            }
        }
        let s1: HashSet<_> = s1.iter().cloned().map(|s| ArcIntern::new(s)).collect();
        let s2: HashSet<_> = s2.iter().cloned().map(|s| ArcIntern::new(s)).collect();
        println!(
            "ArcIntern<String>::compare/hash {}",
            bench_gen_env(|| (s1.clone(), s2.clone()), rmset)
        );
    }
    let i = ArcIntern::new(7i64);
    println!("ArcIntern<i64>::clone {}", bench(|| i.clone()));
    println!(
        "ArcIntern<TestStruct> multithreading {}",
        bench(|| multithreading::<arc_interner::ArcIntern<TestStruct>>())
    );
    println!();

    println!(
        "arc_interner::ArcIntern<i64>::new {}",
        bench_gen_env(
            || rand::random::<i64>(),
            |x| arc_interner::ArcIntern::new(*x)
        )
    );
    println!(
        "arc_interner::ArcIntern<u8>::new {}",
        bench_gen_env(
            || rand::random::<u8>(),
            |x| arc_interner::ArcIntern::new(*x)
        )
    );
    {
        fn mkset(s: &mut HashSet<String>) -> HashSet<arc_interner::ArcIntern<String>> {
            let mut n = HashSet::new();
            for x in s.iter() {
                n.insert(arc_interner::ArcIntern::new(x.clone()));
            }
            n
        }
        mkset(&mut s1.clone());
        println!(
            "arc_interner::ArcIntern<String>::new {}",
            bench_gen_env(|| s1.clone(), mkset)
        );
        fn rmset(
            s: &mut (
                HashSet<arc_interner::ArcIntern<String>>,
                HashSet<arc_interner::ArcIntern<String>>,
            ),
        ) {
            for x in s.1.iter() {
                s.0.remove(x);
            }
        }
        let s1: HashSet<_> = s1
            .iter()
            .cloned()
            .map(|s| arc_interner::ArcIntern::new(s))
            .collect();
        let s2: HashSet<_> = s2
            .iter()
            .cloned()
            .map(|s| arc_interner::ArcIntern::new(s))
            .collect();
        println!(
            "arc_interner::ArcIntern<String>::compare/hash {}",
            bench_gen_env(|| (s1.clone(), s2.clone()), rmset)
        );
    }

    let i = arc_interner::ArcIntern::new(7i64);
    println!(
        "arc_interner::ArcIntern<i64>::clone {}",
        bench(|| i.clone())
    );

    println!(
        "arc_interner::ArcIntern<TestStruct> multithreading {}",
        bench(|| multithreading::<arc_interner::ArcIntern<TestStruct>>())
    );
    println!();
}
