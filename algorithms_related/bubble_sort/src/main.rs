
fn bubble_sort(arr: &mut [i32], ascending: bool) {
    let len = arr.len();
    for i in 0..len - 1 {
        for j in 0..len - i - 1 {
            if ascending && arr[j] > arr[j+1] || (!ascending && arr[j] < arr[j+1]) {
                arr.swap(j, j+1)
            }
        }
    }
}



fn main() {
    let mut arr = [22,2,2];
    // bubble_sort(&mut arr);
}
