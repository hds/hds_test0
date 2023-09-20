#![doc = include_str!("../README.md")]

/// Prints out what Hayden says.
///
/// Make Hayden say whatever you want!
pub fn hayden_says(what: &str) {
    println!("Hayden says '{what}'.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        hayden_says("something");
    }
}
