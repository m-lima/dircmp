pub struct SortedVec<T>(Vec<T>);

impl<T> std::ops::Deref for SortedVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Ord> SortedVec<T> {
    pub fn new(mut vec: Vec<T>) -> Self {
        vec.sort_unstable();
        Self(vec)
    }

    pub fn merge(self, other: Self) -> Self {
        let mut out = Vec::with_capacity(self.len().max(other.len()));
        let mut left = self.to_iter();
        let mut right = other.to_iter();
        let mut left_next = left.next();
        let mut right_next = right.next();

        loop {
            match (left_next, right_next) {
                (None, None) => break,
                (Some(some_left_next), None) => {
                    out.push(some_left_next);
                    out.extend(left);
                    break;
                }
                (None, Some(some_right_next)) => {
                    out.push(some_right_next);
                    out.extend(right);
                    break;
                }
                (Some(some_left_next), Some(some_right_next)) => {
                    if some_left_next <= some_right_next {
                        out.push(some_left_next);
                        left_next = left.next();
                        right_next = Some(some_right_next);
                    } else {
                        out.push(some_right_next);
                        left_next = Some(some_left_next);
                        right_next = right.next();
                    }
                }
            }
        }
        SortedVec(out)
    }

    pub fn to_iter(self) -> std::vec::IntoIter<T> {
        self.0.into_iter()
    }
}
