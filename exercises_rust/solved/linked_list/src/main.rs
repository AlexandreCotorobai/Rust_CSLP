// import the linked_list module
mod linked_list;
use linked_list::LinkedList;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list_pop_left() {
        let mut list: LinkedList<i32> = LinkedList::<i32>::new();

        list.push_left(1);
        list.push_left(2);
        list.push_left(3);
        list.push_left(4);

        assert_eq!(list.pop_left(), Some(4));
        assert_eq!(list.pop_left(), Some(3));
        assert_eq!(list.pop_left(), Some(2));
        assert_eq!(list.pop_left(), Some(1));
    }
}
