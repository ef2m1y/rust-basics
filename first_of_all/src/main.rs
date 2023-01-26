fn main() {
    let arr1: [i32; 3] = [1, 2, 3];
    let arr2: [i32; 1000] = [0; 1000];
    println!("{:?}", arr1);

    println!("{}", arr1[0]);
    
    let [x, y, z] = arr1;
    println!("{x}");

    let arr3: &[i32] = &arr1[..=1];
    println!("{:?}", arr3);
}