use extend::ext;

trait MyTrait {}

impl MyTrait for String {}

#[ext(supertraits = Default + Clone + MyTrait)]
impl String {
    fn my_len(&self) -> usize {
        self.len()
    }
}

fn main() {
    assert_eq!(String::new().my_len(), 0);
}
