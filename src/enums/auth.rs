#[derive(Clone,Debug,PartialEq)]
pub enum Auth {
    None,
    Bearer(String)
}