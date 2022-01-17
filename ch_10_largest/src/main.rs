fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest2<'a, T: PartialOrd>(list:  &'a [T]) ->  &'a T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest3<T: PartialOrd + Clone>(list: &[T]) -> T {
    let list = list.clone();
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest.clone()
}


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![34, 50, 25, 200, 65];
    let result = largest2(&number_list);
    println!("The largest number is {}", result);


    let number_list = vec![34, 50, 25, 300, 65];
    let result = largest3(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

