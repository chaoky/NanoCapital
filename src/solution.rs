use std::collections::BTreeMap;

/// In Rust, the `type` keyword is used to define an alias for an existing type
/// If you want, you can learn more about it [here](https://doc.rust-lang.org/std/keyword.type.html)
type Item<'a> = (i32, &'a str);
type Inventory<'a> = Vec<Item<'a>>;

// 1 3 8 9 31
// 1 4 9 10 21 30
// 1 3 4 8 9
/// Don't worry about the `'a` syntax, this is a sligtly advanced concept in Rust called Lifetimes
/// but you don't need to deeple understand this in order to complete this challenge.
/// You can learn more about lifetimes [here](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html).
pub fn update_inventory_sort_first<'a>(
    mut cur_inv: Inventory<'a>,
    mut new_inv: Inventory<'a>,
) -> Inventory<'a> {
    cur_inv.sort_by_key(|x| x.1);
    new_inv.sort_by_key(|x| x.1);

    let mut xs = cur_inv.drain(..);
    let mut ys = new_inv.drain(..);

    let mut collect: Inventory = Vec::new();
    let mut maybe_x = xs.next();
    let mut maybe_y = ys.next();

    loop {
        //maybe peek here?
        match (maybe_x, maybe_y) {
            (None, None) => break,
            (None, Some(t)) => {
                collect.push(t);
                collect.append(&mut ys.collect());
                break;
            }
            (Some(t), None) => {
                collect.push(t);
                collect.append(&mut xs.collect());
                break;
            }
            (Some(x), Some(y)) => match x.1.cmp(y.1) {
                std::cmp::Ordering::Equal => {
                    collect.push((x.0 + y.0, x.1));
                    maybe_x = xs.next();
                    maybe_y = ys.next();
                }
                std::cmp::Ordering::Less => {
                    collect.push(x);
                    maybe_x = xs.next();
                }
                std::cmp::Ordering::Greater => {
                    collect.push(y);
                    maybe_y = ys.next();
                }
            },
        }
    }

    collect
}

pub fn update_inventory<'a>(
    mut cur_inv: Inventory<'a>,
    mut new_inv: Inventory<'a>,
) -> Inventory<'a> {
    cur_inv
        .drain(..)
        .chain(new_inv.drain(..))
        .fold(BTreeMap::new(), |mut t, (value, key)| {
            t.entry(key).and_modify(|x| *x += value).or_insert(value);
            t
        })
        .into_iter()
        .map(|x| (x.1, x.0))
        .collect()
}
