pub mod btree
{
    struct BTreeNode<'a, T: std::cmp::Ord + std::default::Default + 'a, const B: usize>
    {
        _keys: Vec<T>,
        _child: &'a  Vec::<BTreeNode<'a, T, B>>,
        _size: usize,
        _leaf: bool
    }

    impl<'a, T: std::cmp::Ord + std::default::Default + 'a, const B: usize> BTreeNode<'a, T, B>
    {
        pub fn new(leaf: bool) -> Self
        {
            let mut keys = vec![Default::default();2 * B - 1];
            let mut child = &vec![BTreeNode::<'a, T, B>::new(true);2*B];
            let size = 0 as usize;
            Self
            {
                _keys: keys,
                _child: child,
                _size: size,
                _leaf: leaf
            }
        }
    }
}