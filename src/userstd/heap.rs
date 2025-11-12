#[derive(PartialEq)]
pub enum HeapSortType {
    Max,
    Min,
}
pub fn heap_sort(data: &[i32], sort_type: HeapSortType) -> Vec<i32> {
    let mut data: Vec<i32> = data.into();
    for i in 0..data.len() {
        let mut p = i;
        while p != 0 {
            if data[p] < data[p / 2] {
                data.swap(p, p / 2);
            }
            p /= 2;
        }
    }
    for i in 0..data.len() {
        let n = data.len();
        data.swap(n - 1 - i, 0);

        let mut p = 0;
        while p < n - 1 - i {
            if 2 * p + 2 < n - 1 - i {
                if data[2 * p + 1] <= data[2 * p + 2] && data[p] > data[2 * p + 1] {
                    data.swap(p, 2 * p + 1);
                    p = 2 * p + 1;
                } else if data[p] > data[2 * p + 2] {
                    data.swap(p, 2 * p + 2);
                    p = 2 * p + 2;
                } else {
                    break;
                }
            } else if 2 * p + 1 < n - 1 - i && data[p] > data[2 * p + 1] {
                data.swap(p, 2 * p + 1);
                p = 2 * p + 1;
            } else {
                break;
            }
        }
    }
    if sort_type == HeapSortType::Min {
        data.reverse();
    }
    data
}
