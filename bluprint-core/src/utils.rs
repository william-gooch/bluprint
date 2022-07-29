use num::PrimInt;
use std::ops::{Range, RangeInclusive};

pub trait IntoIterator3D<T: PrimInt> {
    fn into_3d_iter(self) -> Iterator3D<T>;
}

impl<T: PrimInt> IntoIterator3D<T> for RangeInclusive<(T, T, T)> {
    fn into_3d_iter(self) -> Iterator3D<T> {
        Iterator3D::new(self)
    }
}

impl<T: PrimInt> IntoIterator3D<T> for Range<(T, T, T)> {
    fn into_3d_iter(self) -> Iterator3D<T> {
        let new_range = self.start
            ..=(
                self.end.0 - T::one(),
                self.end.1 - T::one(),
                self.end.2 - T::one(),
            );

        Iterator3D::new(new_range)
    }
}

pub struct Iterator3D<T>
where
    T: PrimInt,
{
    range: RangeInclusive<(T, T, T)>,
    current: Option<(T, T, T)>,
}

impl<T: PrimInt> Iterator3D<T> {
    fn new(range: RangeInclusive<(T, T, T)>) -> Self {
        Self {
            current: Some(range.start().clone()),
            range,
        }
    }
}

impl<T: PrimInt> Iterator for Iterator3D<T> {
    type Item = (T, T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let prev = self.current.clone();
        if let Some(curr) = &mut self.current {
            if curr.0 >= self.range.end().0 {
                if curr.1 >= self.range.end().1 {
                    if curr.2 >= self.range.end().2 {
                        self.current = None
                    } else {
                        curr.0 = self.range.start().0;
                        curr.1 = self.range.start().1;
                        curr.2 = curr.2.add(T::one());
                    }
                } else {
                    curr.0 = self.range.start().0;
                    curr.1 = curr.1.add(T::one());
                }
            } else {
                curr.0 = curr.0.add(T::one());
            }
        }
        prev
    }
}
