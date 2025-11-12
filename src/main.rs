use crate::userstd::heap;

mod userstd;

fn main() {
    let myvec = [7, 3, 1, 5, 2, 4, 6];
    println!("{:?}", heap::heap_sort(&myvec, heap::HeapSortType::Min));
}
