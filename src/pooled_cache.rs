
use std::collections::LinkedList;
use dashmap::DashMap;
use std::cmp::Eq;
use std::hash::Hash;
use std::mem::MaybeUninit;
use std::collections::linked_list::IterMut;

pub type PooledCacheIndex = (usize, usize);

struct CachePool<const C: usize, T: Sized> {
    count: usize,
    data: [Option<T>; C]
}
impl<const C: usize, T: Sized> CachePool<C, T> {
    fn new(item: T) -> CachePool<C, T> {
        let mut data: [MaybeUninit<Option<T>>; C] = unsafe { MaybeUninit::uninit().assume_init() };
        
        for p in &mut data[..] {
            p.write(None);
        }
        let elem = &mut data[0];
        *elem = MaybeUninit::new(Some(item));

        CachePool { 
            count: 1, 
            data: unsafe { MaybeUninit::array_assume_init(data) },
        }
    }

    fn add(&mut self, item: T) -> usize {
        let index = self.count;
        self.data[index] = Some(item);
        self.count += 1;
        index
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> { 
        self.data[index].as_mut()
    }
}
pub struct PooledCache<const C: usize, K: Eq + Hash, T> {
    map: DashMap<K, PooledCacheIndex>,
    data: LinkedList<CachePool<C, T>>,
}
impl<const C: usize, K: Eq + Hash, T> PooledCache<C, K, T> {
    pub fn new() -> PooledCache<C, K, T> {
        PooledCache { 
            map: DashMap::new(),
            data: LinkedList::new(),
        }
    }

    pub fn get_mut(&mut self, file: &K) -> Option<&mut T> {
        if let Some(i) = self.map.get_mut(file) {
            let pool = self.data.iter_mut().skip(i.0).next().unwrap();
            return pool.get_mut(i.1);
        }
        None
    }

    pub fn get_index_mut(&mut self, index: PooledCacheIndex) -> Option<&mut T> { 
        let pool = self.data.iter_mut().skip(index.0).next().unwrap();
        pool.get_mut(index.1)
    }

    pub fn index_of(&self, file: &K) -> PooledCacheIndex {
        self.map.get(file).unwrap().clone()
    }

    pub fn insert(&mut self, file: K, data: T) {
        for (i, pool) in self.data.iter_mut().enumerate() {
            if pool.count < C {
                let index = pool.add(data);
                self.map.insert(file, (i, index));
                return;
            }
        }

        let pool = CachePool::new(data);
        self.data.push_back(pool);
        self.map.insert(file, (self.data.len() - 1, 0));
    }

    pub fn indices(&mut self) -> PooledCacheIter<C, T> {
        PooledCacheIter::new(&mut self.data)
    }
}

pub struct PooledCacheIter<'a, const C: usize, T> {
    pools: IterMut<'a, CachePool<C, T>>,
    curr: Option<&'a mut CachePool<C, T>>,
    pool_index: usize,
    index: usize,
}
impl<'a, const C: usize, T> PooledCacheIter<'a, C, T> {
    fn new(data: &'a mut LinkedList<CachePool<C, T>>) -> PooledCacheIter<'a, C, T> {
        PooledCacheIter { 
            pools: data.iter_mut(),
            curr: None, 
            pool_index: 0, 
            index: 0 
        }
    }
}
impl<'a, const C: usize, T> Iterator for PooledCacheIter<'a, C, T> {
    type Item = PooledCacheIndex;
    fn next(&mut self) -> Option<Self::Item> {
        self.index += 1;
        if self.curr.is_none() || self.index >= self.curr.as_ref().unwrap().count {
            self.curr = self.pools.next();
            self.index = 0;
            self.pool_index += 1;
        }

        let index = (self.pool_index - 1, self.index);
        if let Some(p) = &mut self.curr {
            if let Some(_) = p.data[self.index] {
                return Some(index);
            }
        }
        return None;
    }
}