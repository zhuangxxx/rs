fn main() {
    let vec = vec![1, 2, 3];
    let mut sum = 0;
    for i in vec {
        sum += i;
        println!("sum + {} = {}", i, sum);
    }
}
