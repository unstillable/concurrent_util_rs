

trait Collection<T: Clone + Copy>{

    // The number of elements in this colleciton
    fn size(&self) -> u32;

    // True if the collecciton contains no Elements
    fn isEmpty(&self) -> bool;

    // the collection has at least one element
    // such that member == t (eq by value)
    fn contains(&self, t: &T) -> bool;

    fn iterator(&self) -> Iterator<T>;

    fn toArray(&self) -> [T];

}

