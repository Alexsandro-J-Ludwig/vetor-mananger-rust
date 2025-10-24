struct benchmark_sorts {
    array: Vec<String>,
}

impl benchmark_sorts {
    // N - 1
    fn insertion_sort(array: &mut Vec<String>){
        for i in array.len() {
            let mut j = i;
            while j > 0 && array[j] < array[j - 1] {
                array.swap(j, j - 1);
                j -= 1;
            }
        }
    }
    fn selection_sort(array: &mut Vec<String>){}
    fn quick_sort(array: &mut Vec<String>){}
    fn marge_sort(array: &mut Vec<String>){}
    fn shell_sort(array: &mut Vec<String>){}
    fn bubble_sort(array: &mut Vec<String>){}
    fn heap_sort(array: &mut Vec<String>){}
    fn couting_sort(array: &mut Vec<String>){}
    fn radix_sort(array: &mut Vec<String>){}
    fn bucket_sort(array: &mut Vec<String>){}
}