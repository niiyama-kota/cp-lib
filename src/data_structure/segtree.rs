pub mod segtree
{
    trait _BinOp<T>
    {
        fn apply(lhs: T, rhs: T) -> T;
    }
    trait _Magma<T: _BinOp<Self>>: Sized {}
    impl<T, Op> _Magma<Op> for T where Op: _BinOp<T> {}
    pub struct SegTree<T> where T: Monoid
    {

    }
}