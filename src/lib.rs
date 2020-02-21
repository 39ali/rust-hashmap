use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;
use std::borrow::Borrow;

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    items_size: usize,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        HashMap {
            buckets: Vec::new(),
            items_size: 0,
        }
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq,
{
    fn get_bucket_index<Q>(&self, key: &Q) -> usize
    where K:Borrow<Q>, 
    Q:Hash +Eq +?Sized 
    {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % (self.buckets.len() as u64)) as usize
    }
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.buckets.is_empty() || self.items_size > (3 / 4) * self.buckets.len() {
            self.resize();
        }
        let bucket_index = self.get_bucket_index(&key);
        let bucket = &mut self.buckets[bucket_index];
        for &mut (ref ekey, ref mut eval) in bucket.iter_mut() {
            if ekey == &key {
                return Some(mem::replace(eval, value));
            }
        }
        bucket.push((key, value));
        self.items_size+=1;
        None
    }

      pub fn contains_key<Q>(&self, key: &Q) -> bool
        where 
        K:Borrow<Q>,
        Q: Hash +Eq +?Sized
      {
    
        let bucket_index = self.get_bucket_index(key.borrow());
        self.buckets[bucket_index]
            .iter()
            .find(|&(ref ekey, _)| ekey.borrow() == key)
            .is_some()
      }

      pub fn get<Q>(&self, key: &Q) -> Option<&V>
      where
      K:Borrow<Q>, 
      Q:Hash +Eq +?Sized {
        let bucket_index = self.get_bucket_index(key);
        self.buckets[bucket_index]
            .iter()
            .find(|&(ref ekey, _)| ekey.borrow() == key)
            .map(|&(_, ref eval)| eval)
    }

    pub fn remove<Q>(&mut self, key:&Q) -> Option<V>
      where
      K:Borrow<Q>, 
      Q:Hash +Eq +?Sized {
        let bucket_index = self.get_bucket_index(key);
        let bucket=&mut self.buckets[bucket_index]; 
        let element_index = bucket.iter()
        .position(|&(ref ekey,_)| ekey.borrow() ==key)?;
        self.items_size-=1;
        Some(bucket.swap_remove(element_index).1)
        }
    
    pub fn len(&self)-> usize{
        self.items_size
    }

    pub fn is_empty(&self) -> bool{
        self.items_size==0
    }
    fn resize(&mut self) {
        let new_size = match self.buckets.len() {
            0 => 1,
            n => 2 * n,
        };

        let mut new_buckets = Vec::with_capacity(new_size);
        new_buckets.extend((0..new_size).map(|_| Vec::new()));

        for (key, value) in self.buckets.iter_mut().flat_map(|bucket| bucket.drain(..)) {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let bucket_index = (hasher.finish() % new_buckets.len() as u64) as usize;
            new_buckets[bucket_index].push((key, value));
        }
        mem::replace(&mut self.buckets, new_buckets);
    }
}


pub struct Iter<'a,K:'a,V:'a>{
    map:&'a HashMap<K,V>,
    bucket:usize,
    at:usize,
}

impl<'a,K,V> IntoIterator for &'a HashMap<K,V>{
    type Item = (&'a K,&'a V);
    type IntoIter= Iter<'a,K,V>;
    
    fn into_iter(self)-> Self::IntoIter{
        Iter{map:self,bucket:0,at:0}
    }
}

impl<'a,K,V> Iterator for Iter<'a,K,V>{
    type Item = (&'a K, &'a V);
    fn next(&mut self)->Option<Self::Item>{
        loop{
            match self.map.buckets.get(self.bucket){
                Some(bucket)=>{
                    match bucket.get(self.at){
                        Some(&(ref k,ref v))=>{
                            self.at+=1;
                            break Some((k,v));
                        }
                        None =>{
                            self.bucket+=1;
                            self.at=0;
                            continue;
                            }
                    }
                }
                None => break None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insert() {
        let mut hashmap = HashMap::new();
        hashmap.insert("meaning of life", 42);
    }
    #[test]
    fn get() {
        let mut hashmap = HashMap::new();
        hashmap.insert("key", "val");
        let v = hashmap.get(&"key");
        assert_eq!(v, Some(&"val"));
    }
   #[test] 
    fn remove() {
        let mut hashmap = HashMap::new();
        hashmap.insert("meaning of life", 42);
        hashmap.remove(&"meaning of life");
        assert_eq!(hashmap.get(&"meaning of life"),None);
    }

}
