struct User<'a>{
    name: &'a str
}

fn main(){
    let first_name = String::from("Vatsal");
    let user = User {name : &first_name};
    println!("{:?}", user.name)
}
