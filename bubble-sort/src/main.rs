fn print_vec(container: &Vec<u64>) {
    for &x in container {
        println!("{}", x);
    }
}

fn bubble_sort(container: &mut Vec<u64>) {
    let length = container.len();
    if length <= 1 {
        return
    }

    let mut swap: bool = true;

    while swap {
        swap = false;

        for i in 0..length - 1 {
            if container[i] > container[i + 1] {
                container.swap(i, i + 1);
                swap = true;
            }
        }
    }
}

fn main() {
    let mut values : Vec<u64> = vec![9, 12, 1, 45, 99, 4];

    print_vec(&values);

    bubble_sort(&mut values);

    print_vec(&values);
}
