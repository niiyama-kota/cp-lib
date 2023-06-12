pub mod btree {
    #[derive(Clone)]
    struct BTreeNode<T: std::cmp::Ord + std::default::Default + Clone, const B: usize> {
        _keys: Vec<T>,
        _child: Option<Vec<BTreeNode<T, B>>>,
        _size: usize,
        _leaf: bool,
    }

    impl<T: std::cmp::Ord + std::default::Default + Clone, const B: usize> BTreeNode<T, B> {
        pub fn new(leaf: bool) -> Self {
            let keys = vec![Default::default(); 2 * B - 1];
            let child = if leaf {
                None
            } else {
                Some(vec![BTreeNode::<T, B>::new(true); 2 * B])
            };
            let size = 0 as usize;
            Self {
                _keys: keys,
                _child: child,
                _size: size,
                _leaf: leaf,
            }
        }
    }
}
