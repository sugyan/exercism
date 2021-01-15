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
            if first_list.is_empty()
                || second_list
                    .windows(first_list.len())
                    .any(|w| w[0] == first_list[0] && w[1..] == first_list[1..])
            {
                return Comparison::Sublist;
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
