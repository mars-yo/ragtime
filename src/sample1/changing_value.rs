
struct ChangingValue<T> {
    value:T,
    change:T,
}

impl<T> ChangingValue<T> {
    fn update(&mut self) {
        self.value += self.change;
    }
    fn set_change(&mut self, change:T) {
        self.change = change;
    }
    fn value(&mut self) {
        self.value
    }
}
