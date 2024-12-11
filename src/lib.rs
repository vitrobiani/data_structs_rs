pub mod graph;
pub mod linked_list;
pub mod node;
pub mod probability;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::probability;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        probability::Urn::new(3);
    }
}
