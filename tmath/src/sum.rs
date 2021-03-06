use trytes::*;

#[inline]
pub fn bct_sum(tuple: (Trit, Trit)) -> Trit {
    let s = tuple.0 + tuple.1;
    match s {
        2 => -1,
        -2 => 1,
        _ => s,
    }
}
pub trait Sum<T> {
    fn sum(&self) -> T;
}

impl Sum<Trit> for (Trit, Trit) {
    fn sum(&self) -> Trit {
        bct_sum(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum_works() {
        assert_eq!(-1, (1, 1).sum());
        assert_eq!(-1, (0, -1).sum());
        assert_eq!(0, (1, -1).sum());
        assert_eq!(1, (1, 0).sum());
        assert_eq!(1, (-1, -1).sum());
    }
}
