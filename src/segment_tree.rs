use crate::monoid::Monoid;
use std::cmp::{max, min};

/// Segment tree
#[derive(Debug)]
pub struct SegmentTree<T> {
    len: usize,
    v: Vec<T>,
}

impl<T: Clone + Monoid> SegmentTree<T> {
    /// Construct segment tree for given size.
    pub fn new(n: usize) -> Self {
        let s: &[T] = &[];
        Self::init(n, s)
    }

    /// Construct segment tree from slice.
    pub fn from_slice(s: &[impl Into<T> + Clone]) -> Self {
        Self::init(s.len(), s)
    }

    fn init(len: usize, s: &[impl Into<T> + Clone]) -> Self {
        let n = len.next_power_of_two();
        let mut v = vec![T::mempty(); n * 2 - 1];
        for i in 0..s.len() {
            v[n - 1 + i] = s[i].clone().into();
        }

        let mut l = n / 2;
        let mut ofs = n - 1 - l;

        while l > 0 {
            for i in 0..l {
                let ix = ofs + i;
                v[ix] = T::mappend(&v[ix * 2 + 1], &v[ix * 2 + 2]);
            }
            l /= 2;
            ofs -= l;
        }

        Self { len, v }
    }

    /// Length of sequence
    pub fn len(&self) -> usize {
        self.len
    }

    /// Set v to `i`-th element
    /// `s[i] = v`
    pub fn set(&mut self, i: usize, v: impl Into<T>) {
        let n = (self.v.len() + 1) / 2;
        let mut cur = n - 1 + i;
        self.v[cur] = v.into();
        while cur > 0 {
            cur = (cur - 1) / 2;
            self.v[cur] = T::mappend(&self.v[cur * 2 + 1], &self.v[cur * 2 + 2]);
        }
    }

    /// mappend v to `i`-th element
    /// `s[i] = mappend(s[i], v)`
    pub fn mappend(&mut self, i: usize, v: impl Into<T>) {
        self.set(i, T::mappend(&self.get(i), &v.into()));
    }

    /// Get i-th element
    /// Equals to `query(i, i + 1)`
    pub fn get(&self, i: usize) -> T {
        let n = (self.v.len() + 1) / 2;
        self.v[n - 1 + i].clone()
    }

    /// Query for `[l, r)`.
    ///
    /// # Constraints
    ///
    /// * `l <= r`
    /// * `r <= self.len()`
    ///
    /// # Returns
    ///
    /// `Monoid::mconcat(&s[l..r])`
    ///
    pub fn query(&self, l: usize, r: usize) -> T {
        assert!(l <= r);
        assert!(r <= self.len);
        let n = (self.v.len() + 1) / 2;
        self.q(0, n, l, r)
    }

    fn q(&self, ix: usize, span: usize, l: usize, r: usize) -> T {
        if l == r {
            T::mempty()
        } else if r - l == span {
            self.v[ix].clone()
        } else {
            let m = span / 2;
            T::mappend(
                &self.q(ix * 2 + 1, m, min(l, m), min(r, m)),
                &self.q(ix * 2 + 2, m, max(l, m) - m, max(r, m) - m),
            )
        }
    }
}

#[test]
fn test() {
    use crate::monoid::Sum;

    {
        let st = SegmentTree::<Sum<i64>>::new(5);
        assert_eq!(st.v.iter().map(|r| r.0).collect::<Vec<_>>(), vec![0; 15]);
    }

    {
        let st = SegmentTree::<Sum<i64>>::from_slice(&[1, 2, 3, 4, 5]);
        assert_eq!(
            st.v.iter().map(|r| r.0).collect::<Vec<_>>(),
            vec![
                15, //
                10, 5, //
                3, 7, 5, 0, //
                1, 2, 3, 4, 5, 0, 0, 0
            ]
        );
    }

    let mut st = SegmentTree::<Sum<i64>>::new(5);
    st.set(2, 1);
    assert_eq!(st.query(0, 5).0, 1);
    assert_eq!(st.query(0, 2).0, 0);
    assert_eq!(st.query(3, 5).0, 0);
    assert_eq!(st.query(2, 3).0, 1);
    assert_eq!(st.query(2, 2).0, 0);
    st.mappend(2, 2);
    assert_eq!(st.query(0, 5).0, 3);
    assert_eq!(st.query(0, 2).0, 0);
    assert_eq!(st.query(3, 5).0, 0);
    assert_eq!(st.query(2, 3).0, 3);
    assert_eq!(st.query(2, 2).0, 0);
    st.set(0, 1);
    st.set(1, 2);
    st.set(3, 4);
    st.set(4, 5);
    assert_eq!(st.query(0, 5).0, 15);
    assert_eq!(st.query(0, 2).0, 3);
    assert_eq!(st.query(3, 5).0, 9);
    assert_eq!(st.query(2, 3).0, 3);
    assert_eq!(st.query(2, 2).0, 0);
}
