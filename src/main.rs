use userstd::linkedlist::LinkedList;

mod userstd;

fn main() {
    let mut list: LinkedList = LinkedList::new();
    println!("{:?}", list);
    list.push(2);
    println!("{:?}", list);
    list.push(3);
    println!("{:?}", list);

    println!("{:?}", list.get());
    list.pop();
    println!("{:?}", list.get());
    list.pop();
    println!("{:?}", list.get());
}
