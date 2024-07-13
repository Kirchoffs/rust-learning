#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use std::hash::Hash;

    /**
     * 
     *  pub trait Iterator {
     *      type Item;
     *      ...
     *  }
     * 
     */

    pub trait IteratorExt: Iterator {
        fn unique(self) -> UniqueIterator<Self>
        where
            Self: Sized,
            Self::Item: Eq + Hash + Clone,
        {
            UniqueIterator {
                iter: self,
                seen: HashSet::new(),
            }
        }
    }
    
    pub struct UniqueIterator<I>
    where
        I: Iterator,
        I::Item: Eq + Hash + Clone,
    {
        iter: I,
        seen: HashSet<I::Item>,
    }
    
    impl<I> Iterator for UniqueIterator<I>
    where
        I: Iterator,
        I::Item: Eq + Hash + Clone,
    {
        type Item = I::Item;
    
        fn next(&mut self) -> Option<Self::Item> {
            self.iter.find(|item| self.seen.insert(item.clone()))
        }
    }

    impl<I> IteratorExt for I where I: Iterator {}

    #[test]
    fn extension_trait_demo() {
        let data = vec![1, 2, 3, 1, 2, 3, 4, 5];
        
        let unique_data: Vec<_> = data.clone().into_iter().unique().collect();
        println!("Unique data: {:?}", unique_data);

        let mut unique_iter = data.clone().into_iter().unique();
        while let Some(value) = unique_iter.next() {
            println!("Value: {:?}", value);
        }
    }
}
