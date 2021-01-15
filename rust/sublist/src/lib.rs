#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match first_list.len().cmp(&second_list.len()) {
        std::cmp::Ordering::Less => {
            for i in 0..=second_list.len() - first_list.len() {
                if first_list
                    .iter()
                    .enumerate()
                    .all(|(j, t)| *t == second_list[i + j])
                {
                    return Comparison::Sublist;
                }
            }
        }
        std::cmp::Ordering::Equal => {
            if first_list == second_list {
                return Comparison::Equal;
            }
        }
        std::cmp::Ordering::Greater => {
            if sublist(second_list, first_list) == Comparison::Sublist {
                return Comparison::Superlist;
            }
        }
    }
    Comparison::Unequal
}
