use num::{CheckedAdd, CheckedSub, One};

pub trait Coord {
    fn adjacent_cardinal(&self) -> Vec<Self>
    where
        Self: Sized;
    fn adjacent_diagonal(&self) -> Vec<Self>
    where
        Self: Sized;
    fn adjacent_all(&self) -> Vec<Self>
    where
        Self: Sized,
    {
        let mut v = self.adjacent_cardinal();
        v.append(&mut self.adjacent_diagonal());
        v
    }
}

pub type ICoord = (isize, isize);
pub type UCoord = (usize, usize);

impl<T: CheckedAdd + CheckedSub + One + Copy> Coord for (T, T) {
    fn adjacent_cardinal(&self) -> Vec<(T, T)> {
        [
            (self.0.checked_add(&T::one()), Some(self.1)),
            (Some(self.0), self.1.checked_add(&T::one())),
            (self.0.checked_sub(&T::one()), Some(self.1)),
            (Some(self.0), self.1.checked_sub(&T::one())),
        ]
        .iter()
        .filter_map(|a| a.0.map(|a0| (a0, a.1)))
        .filter_map(|a| a.1.map(|a1| (a.0, a1)))
        .collect()
    }
    fn adjacent_diagonal(&self) -> Vec<(T, T)> {
        [
            (self.0.checked_add(&T::one()), self.1.checked_add(&T::one())),
            (self.0.checked_add(&T::one()), self.1.checked_sub(&T::one())),
            (self.0.checked_sub(&T::one()), self.1.checked_add(&T::one())),
            (self.0.checked_sub(&T::one()), self.1.checked_sub(&T::one())),
        ]
        .iter()
        .filter_map(|a| a.0.map(|a0| (a0, a.1)))
        .filter_map(|a| a.1.map(|a1| (a.0, a1)))
        .collect()
    }
}
