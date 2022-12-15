mod bubble_sort;
mod traffic_light;
mod sum;
mod area;
use bubble_sort::bubble_sort::bubble_sort;

fn main() {
    let mut test= [1,2,3,6,9,7,1,2,45,3];
    bubble_sort(&mut test);
    for i in 0..test.len(){
        print!("{}\t", test[i]);
    }
}
