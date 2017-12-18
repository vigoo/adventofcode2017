extern crate linked_list;

use self::linked_list::{Cursor, LinkedList};

fn insert(steps: i32, n: i32, cursor: &mut Cursor<i32>) {
    for _ in 0..steps {
        match cursor.next() {
            None => {
                cursor.reset();
                cursor.next();
            },
            Some(_) => {}
        }
    }

    cursor.insert(n);
    cursor.next();
}

pub fn after_last_written(cursor: &mut Cursor<i32>) -> i32 {
    let at_end = cursor.peek_next().is_none();
    if at_end {
        cursor.reset();
        let result = *cursor.peek_next().unwrap();
        cursor.prev();
        cursor.prev();
        result
    }
    else {
        *cursor.peek_next().unwrap()
    }
}

fn after_zero(items: &LinkedList<i32>) -> i32 {
    let mut prev = None;
    let mut result = items.iter().next().map(|item| *item);
    for item in items.iter() {
        if prev == Some(0) {
            result = Some(*item);
        }
        prev = Some(*item)
    }

    result.unwrap()
}

pub fn run() {
    let steps = 343;
    let mut items = LinkedList::new();
    items.push_front(0);

    {
        let mut cursor = items.cursor();

        for n in 1..2018 {
            insert(steps, n, &mut cursor);
        }

        println!("Day 17 result 1: {}", after_last_written(&mut cursor));

        for n in 2018..50000001 {
            insert(steps, n, &mut cursor);

            if (n % 10000) == 0 {
                println!("{}", n)
            }
        }
    }

    println!("Day 17 result 2: {}", after_zero(&items));
}