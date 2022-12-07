pub mod bubble_sort {
    pub fn bubble_sort<T: PartialOrd>(list: &mut [T]) {
        // 获取列表长度避免多次调用method
        let size = list.len();

        // 特殊情况，列表空或仅有一个元素可以直接返回
        if size <= 1 {
            return;
        }

        // 对所有元素遍历，但是因为前一个比较后一个，所以可以排除掉最后一个
        for i in 0..(size - 1) {
            for j in 0..(size - i - 1) {
                // 如果前者大，就交换他们两个。
                if list[j] > list[j + 1] {
                    // 交换，用temp赋值交换会出现cannot move out of type `[T]`, a non-copy slice
                    list.swap(j, j + 1);
                }
            }
        }
    }
}

#[test]
fn test_empty_list() {
    let mut empty_list: Vec<i32> = vec![];
    bubble_sort::bubble_sort(&mut empty_list);
    assert_eq!(empty_list, Vec::<i32>::new());
}

#[test]
fn test_int_list() {
    let mut empty_list: Vec<i32> = vec![31, 25, 9, 6, 3, 5, 11, 4, 77, 32, 7];
    bubble_sort::bubble_sort(&mut empty_list);
    assert_eq!(empty_list, vec![3, 4, 5, 6, 7, 9, 11, 25, 31, 32, 77]);
}

#[test]
fn test_char_list() {
    let mut empty_list= vec!['a', 'e', 'v', 'b', 'w', '5', '6', '4', '2', 'h', 'i'];
    bubble_sort::bubble_sort(&mut empty_list);
    assert_eq!(empty_list, vec!['2', '4', '5', '6', 'a', 'b', 'e', 'h', 'i', 'v', 'w']);
}

#[test]
fn test_string_list() {
    let mut empty_list= vec!["Test", "Test1", "Run", "Apple", "Zoo", "Pen", "Substrate", "Visual", "Web3", "Studio", "Rust"];
    bubble_sort::bubble_sort(&mut empty_list);
    assert_eq!(empty_list, vec!["Apple", "Pen", "Run", "Rust", "Studio", "Substrate", "Test", "Test1", "Visual", "Web3", "Zoo"]);
}