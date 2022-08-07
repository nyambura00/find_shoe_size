use filter_shoesize_closure::Shoe;
use filter_shoesize_closure::shoes_in_size;

fn main() {
    //println! sizes of shoes available from pb fn shoes_in_size
    println!("{:?}", shoes_in_size(vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ], 10));
}