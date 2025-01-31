// custom LRU Cache implementation
// what is LRU Cache Implementation?
//
// LRU : Least Recently Used
// It refers to a cache replacement algorithm that removes the data item that has been accesses the least recently when the cachce reaches it's capacity
// operates on the principle that the data most recently accessed is likely to be accesses again in the near future.

use custom_hashmap;
use std::hash::Hash;
use std::mem;
use std::marker::Copy;

// struct of double linked list
// since LRU Cache uses Double Linked List under the hood
struct Node<K,V> {
    key : K,
    value : V,
    prev : Option<usize>,            // prev pointer
    next : Option<usize>,            // next pointer
}

// LRU Cache implementation using custom_hashmap and double linked list
pub struct CustomLruCache<K, V> {

    // Hashmap stores key to node index mapping
    map : custom_hashmap::CustomHashMap<K, usize>,

    // vector stores nodes based on index
    nodes : Vec<Option<Node<K,V>>>,

    // Free list for node reuse
    free : Vec<usize>,

    // Head node of Double Linked List
    head : Option<usize>,

    // Tail node of Double Linked List
    tail : Option<usize>

    // maximum capacity
    capacity : usize,
}

// Attach traits to generic types
impl<K : Hash + Eq + Clone, V : Copy> CustomLruCache<K,V> {
    // create an LRU Cache based on the capacity provided within the parameter
    // constructor
    pub fn new(capacity : usize) -> Self {
        CustomLruCache {

            // map can use the original hashmap as well
            // std::collections::HashMap::with_capacity(capacity) is also another possible choice
            map : CustomHashMap::new(),         // since I did not implement a prebuilt capacity based hashmap
            nodes : Vec::with_capacity(capacity),
            free : Vec::new(),
            head : None,
            tail : None,
            capacity
        }
    }

    // get value by, moving accessed item by front
    // returns an immutable reference, meaning we simply want the value 
    // without the intention of making any in-place modification to the cache
    pub fn get(&self, key : &K) -> Option<&V> {

        // get node index from map
        let node_idx = *self.map.get(key)?;     // using ? could cause error

        // Move node to the front of the list (since it was just accessed and likely to be accessed again)
        // logic of prepending a node to a double linked list
        // move_to_front is an helper to implement this logic
        self.move_to_front(node_idx);

        // return reference to value
        self.nodes[node_idx].as_ref().map(|node| &node.value)
    }

    // get mutable reference to value
    // this method should execute with the intention of modifying the reference
    pub fn get_mut(&mut self, key : &K) -> Option<&mut V> {
        let node_idx = *self.map.get(key)?;
        self.move_to_front(node_idx);
        self.nodes[node_idx].as_mut().map(|node| &mut node.value)
    }

    // insert key-value pair
    // evict least recently used if at capacity
    pub fn insert(&mut self, key : K, value : V) {

        // If key exists, update value and move to front
        if let Some(&node_idx) = self.map.get(&key) {
            if let Some(node) = self.nodes[node_idx].as_mut() {
                node.value = value;         // update the value wtihin the existing node
                self.move_to_front(node_idx);           // remove and prepend the node
                return;
            }
        }

        // get node index - either from free list or by adding new
        // node_idx is a value if a linked list exists
        let node_idx = if let Some(idx) = self.free.pop() {
            idx
        } else {
            let idx = self.nodes.len();
            self.nodes.push(None);      // otherwise, push none if no nodes remain  
            idx
        };


        // create new node
        let new_node = Node {
            key : key.clone(),           // to avoid consuming the parameter
            value,
            prev : None,            // because this would be the first node
            next : self.head,       // beause the node will be prepended, the next node will be come the old head node
        };

        // update head and tail element
        if self.tail.is_none() {
            self.tail = Some(node_idx);
        }
        if let Some(head) = self.head {
            self.nodes[head].as_mut().unwrap().prev = Some(node_idx);
        }

        self.head = Some(node_idx);

        // insert node to the existing hashmap
        // nodes array stores double linked list as elements
        self.nodes[node_idx] = Some(new_node);
        self.map.insert(key, node_idx);         // hashmap stores the key and the index of nodes array where the double linked list node can be found 

        // check capacity and evict as needed
        if self.map.len() > self.capacity {
            self.evict_tail();          // evict_tail() : private helper method
        }
    }

    // remove item from cache
    pub fn remove(&mut self, key : &K) -> Option<V> {
        let node_idx = self.map.remove(key)?;
        self.remove_node(node_idx);     // remove_node() : private helper method
    }

    // clear the cache, removing all items
    // similiar to the clear method within the custom hashmap
    // which is similar to the drain() method within the std::collections::HashMap
    // hard reset everything essentially
    pub fn clear(&mut self) {
        self.map.clear();           
        self.nodes.clear();
        self.free.clear();
        self.head = None;
        self.tail = None;
    }

    // get current number of items within cache
    pub fn len(&self) -> usize {
        self.map.len()          
    }

    // check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()         // TODO : self.map[0] == None return True (somethign along that logic)
    }

    // below are the set of private helper methods that are being used by the above methods
    
    // removes the node from the current position of the linked list
    // and move the node to the front of the linked list
    fn move_to_front(&mut self, node_idx : usize) {

        // check if the node is already in front
        // if so we terminate the function immediately
        if Some(node_idx) == self.head {
            return;
        }

        let node = self.nodes[node_idx].as_mut().unwrap();

        // update adjacent nodes
        if let Some(prev) = node.prev {
            self.nodes[prev].as_mut().unwrap().next = node.next;
        }

        if let Some(next) = node.next {
            self.nodes[next].as_mut().unwrap.prev = node.prev;
        }

        // update tail if moving tail
        if Some(node_idx) == self.tail {
            self.tail = node.prev;
        }

        // move to front
        node.prev = None;
        node.next = self.head;
        if let Some(head) = self.head {
            self.nodes[head].as_mut().unwrap().prev = Some(node_idx);
        }

        self.head = Some(node_idx);
    }

    // remove node and return it's value
    // Option::take() : to move ownership out and leave None on the place
    fn remove_node(&mut self, node_idx : usize) -> Option<V> {
        let node = self.nodes[node_idx].take()?;        

        // update adjacent nodes
        if let Some(prev) = node.prev {
            self.nodes[prev].as_mut().unwrap().next = node.next;
        }

        if let Some(next) = node.next {
            self.nodes[next].as_mut().unwrap().prev = node.prev;
        }

        // update head and tail
        if Some(node_idx) == self.head {
            self.head = node.next;      // new head becomes the next node
        }

        if Some(node_idx) == self.tail {
            self.tail = node.prev;      // new tail becomes the previous node
        }

        // add index to free list
        self.free.push(node_idx);

        Some(node.value)
    }

    // remove least recently used tail item
    fn evict_tail(&mut self) {
        if let Some(tail) = self.tail {
            if let Some(node) = &self.nodes[tail] {
                self.map.remove(&node.key);
                self.remove_node(tail);
            }
        }
    }
}


// test the cache to make sure that it works properly as intended.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operations() {
        let mut cache = LruCache::new(2);
        
        // Test insertion
        cache.put(1, "one");
        cache.put(2, "two");
        assert_eq!(cache.get(&1), Some(&"one"));
        assert_eq!(cache.get(&2), Some(&"two"));
        
        // Test eviction
        cache.put(3, "three");
        assert_eq!(cache.get(&1), None); // Should be evicted
        assert_eq!(cache.get(&2), Some(&"two"));
        assert_eq!(cache.get(&3), Some(&"three"));
    }

    #[test]
    fn test_update_existing() {
        let mut cache = LruCache::new(2);
        cache.put(1, "one");
        cache.put(1, "ONE");
        assert_eq!(cache.get(&1), Some(&"ONE"));
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_clear() {
        let mut cache = LruCache::new(2);
        cache.put(1, "one");
        cache.put(2, "two");
        cache.clear();
        assert!(cache.is_empty());
        assert_eq!(cache.get(&1), None);
    }
}