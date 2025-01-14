fn main() {
    let  arr = [1,2,3,4,6,7,8,15,16];
    println!("Array: {:?}", arr);
    for i in 0..arr.len() {
        if arr[i+1] - arr[i] != 1 {
            println!("Element at index {} is {}", i+1, arr[i+1]);
        }
    
    }

}
