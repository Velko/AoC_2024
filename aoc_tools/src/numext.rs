pub trait NumExt
    where
        Self: Sized
{
    fn clamped_add_signed(self, rhs: isize, limit: Self) -> Option<Self>;
}

impl NumExt for usize {
    fn clamped_add_signed(self, rhs: isize, limit: Self) -> Option<Self> {
        let res = self.checked_add_signed(rhs)?;
        if res < limit {
            Some(res)
        } else {
            None
        }
    }
}
