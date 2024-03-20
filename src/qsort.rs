/// A quick sort function that accepts any type using non-recursive approach.
pub fn sort<T: Ord>(array: &mut [T], compare: impl Fn(&T, &T) -> bool) {
    if array.len() < 2 {
        return;
    }

    let mut stack = Vec::new();
    stack.push((0, array.len()));

    while let Some((low, high)) = stack.pop() {
        if high - low > 1 {
            let pivot = partition(array, low, high, &compare);
            stack.push((low, pivot));
            stack.push((pivot + 1, high));
        }
    }
}

fn partition<T: Ord>(
    array: &mut [T],
    low: usize,
    high: usize,
    compare: impl Fn(&T, &T) -> bool,
) -> usize {
    let pivot = low + (high - low) / 2;
    array.swap(pivot, high - 1);

    let mut i = low;
    for j in low..high - 1 {
        if compare(&array[j], &array[high - 1]) {
            array.swap(i, j);
            i += 1;
        }
    }
    array.swap(i, high - 1);
    i
}
