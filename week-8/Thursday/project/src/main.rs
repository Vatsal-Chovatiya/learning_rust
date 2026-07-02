// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }


// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify(item: &(impl Summary + Display)) {}

pub fn notify<T: Summary + Display>(item: &T) {}