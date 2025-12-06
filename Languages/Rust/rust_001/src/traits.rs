
//trait bound
pub fn print_it<T>(item: T)
where T: std::fmt::Display
{
    println!("{}", item);
}

//using ipl trait
pub fn print_it_impl(item: impl std::fmt::Display) {
    println!("{}", item);
}
