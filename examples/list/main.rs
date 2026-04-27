mod list {
    #![allow(unused)]

    pub enum List<T> {
        Node(T, Box<List<T>>),
        Empty,
    }

    pub fn new<T>() -> List<T> {
        List::Empty
    }

    pub fn from_vec<T>(vec: Vec<T>) -> List<T> {
        let mut l = new();
        for v in vec {
            l = add(l, v);
        }
        reverse(l)
    }

    pub fn add<T>(l: List<T>, value: T) -> List<T> {
        List::Node(value, Box::new(l))
    }

    pub fn reverse<T>(l: List<T>) -> List<T> {
        fn reverse<T>(old: List<T>, new: List<T>) -> List<T> {
            match old {
                List::Empty => new,
                List::Node(value, old) => reverse(*old, add(new, value)),
            }
        }
        reverse(l, List::Empty)
    }

    pub fn iter<T>(l: &List<T>, f: impl Fn(&T)) {
        match l {
            List::Empty => (),
            List::Node(value, l) => {
                f(value);
                iter(l, f)
            }
        }
    }

    pub fn iteri<T>(l: &List<T>, f: impl Fn(u32, &T)) {
        fn iteri<T>(i: u32, l: &List<T>, f: impl Fn(u32, &T)) {
            match l {
                List::Empty => (),
                List::Node(value, l) => {
                    f(i, value);
                    iteri(i + 1, l, f)
                }
            }
        }
        iteri(0, l, f)
    }

    pub fn iter2<T, U>(a: &List<T>, b: &List<U>, f: impl Fn(&T, &U)) -> Option<()> {
        match (a, b) {
            (List::Empty, List::Empty) => Some(()),
            (List::Node(va, a), List::Node(vb, b)) => {
                f(va, vb);
                iter2(a, b, f)
            }
            _ => None,
        }
    }

    pub fn print<T: std::fmt::Debug>(l: &List<T>) {
        print!("[");
        iter(l, |v| print!("{:?}, ", v));
        print!("]");
        println!();
    }

    pub fn map<T>(l: List<T>, f: impl Fn(T) -> T) -> List<T> {
        fn map<T>(old: List<T>, new: List<T>, f: impl Fn(T) -> T) -> List<T> {
            match old {
                List::Empty => reverse(new),
                List::Node(value, old) => map(*old, add(new, f(value)), f),
            }
        }
        map(l, List::Empty, f)
    }

    pub fn mapi<T>(l: List<T>, f: impl Fn(u32, T) -> T) -> List<T> {
        fn mapi<T>(i: u32, old: List<T>, new: List<T>, f: impl Fn(u32, T) -> T) -> List<T> {
            match old {
                List::Empty => reverse(new),
                List::Node(value, old) => mapi(i + 1, *old, add(new, f(i, value)), f),
            }
        }
        mapi(0, l, List::Empty, f)
    }

    pub fn fold<T, U>(l: List<T>, init: U, f: impl Fn(U, T) -> U) -> U {
        match l {
            List::Empty => init,
            List::Node(value, l) => fold(*l, f(init, value), f),
        }
    }

    pub fn zip<T>(a: List<T>, b: List<T>) -> Option<List<T>> {
        fn zip<T>(a: List<T>, b: List<T>, out: List<T>) -> Option<List<T>> {
            match (a, b) {
                (List::Empty, List::Empty) => Some(reverse(out)),
                (List::Node(va, a), List::Node(vb, b)) => zip(*a, *b, add(add(out, vb), va)),
                (_, _) => None,
            }
        }
        zip(a, b, List::Empty)
    }

    pub fn map2<T, U, V>(a: List<T>, b: List<U>, f: impl Fn(T, U) -> V) -> Option<List<V>> {
        fn map2<T, U, V>(
            a: List<T>,
            b: List<U>,
            out: List<V>,
            f: impl Fn(T, U) -> V,
        ) -> Option<List<V>> {
            match (a, b) {
                (List::Empty, List::Empty) => Some(reverse(out)),
                (List::Node(va, a), List::Node(vb, b)) => map2(*a, *b, add(out, f(va, vb)), f),
                _ => None,
            }
        }
        map2(a, b, List::Empty, f)
    }
}

fn main() {
    let l = list::from_vec(vec![1, 2, 3, 4, 5]);
    list::print(&l);

    let l = list::reverse(l);
    list::print(&l);

    let l = list::map(l, |v| v * 2);
    list::print(&l);

    let l = list::mapi(l, |i, v| v * 2 + i);
    list::print(&l);

    let sum = list::fold(l, 0, |acc, v| acc + v);
    println!("{sum}");

    let l1 = list::from_vec(vec![1, 2, 3, 4, 5]);
    let l2 = list::from_vec(vec![6, 7, 8, 9, 10]);

    let l = list::zip(l1, l2).unwrap();
    list::print(&l);

    let l1 = list::from_vec(vec![1, 2, 3, 4, 5]);
    let l2 = list::from_vec(vec![true, false, true, true, false]);
    let l3 = list::map2(l1, l2, |a, b| {
        if b {
            std::char::from_digit(a, 10).unwrap()
        } else {
            '%'
        }
    })
    .unwrap();
    list::print(&l3);
}
