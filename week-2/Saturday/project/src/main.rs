fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // Goes up to the parent module to find deliver_order, (super keyword)
    }

    fn cook_order() {}
}