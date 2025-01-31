use std::collections::hash_map::DefaultHasher;

// Hash : a hashable type trait
// Hasher : A trait for hashing an arbitary stream of bytes
use std::hash::{Hash,Hasher};
use std::cmp::PartialEq;        // PartialEq trait
use std::marker::Copy;          // Copy trait that is attached to generic V
// default max size of the underlying array
const DEFAULT_MAX_SIZE : u64 = 256;

// maintains a current size, and it's underlying key value pairs.
#[derive(Debug, Clone)]
pub struct CustomHashMap<T,V> {
    curr_size : usize,

    // [Type of data for the elements; size of the overall array]
    // originally, the elements within the array will all be set as None (refer to the definition of the constructor)
    arr : [Option<KeyValue<T,V>>; DEFAULT_MAX_SIZE as usize],       
}

// Box is a simple form of heap allocation in rust.
// stores the key and value for the hashmap.
#[derive(Clone, Debug)]
pub struct KeyValue<T,V> {
    key : T,
    value : V,

    // can use this to check if an element in the array has been initialised or not
    // refers to an optional next keyValue in a linked list, in the case of collisions
    next : Option<Box<KeyValue<T,V>>>       
}

impl<T,V> KeyValue<T,V> {

    // constructor definition
    pub fn new(key : T, value : V) -> KeyValue<T,V> {
        KeyValue {
            key,
            value,
            next : None         // originally, linked list should point to None
        }
    }
}

// four primary public functions
// generics are similar to templates in C++
// a generic struct or trait that takes type parameters is just like a template
// however, the type can be enforced by saying that the traits that it must implement.
//
// all methods should be wrapped around Option<> to handle None Cases in the event that the value doesn't exist.
// underneath an hashmap, note the following : the keys are the indexes of the array, the values are LinkedLists representation (and thus can handle insertion of multiple values)
//
// T: Clone + Hash + PartialEq are all traits that are being attached to the generic T
// V : Copy
// Meaning, T and V will be the placeholder types
// and those types will inherit the traits listed automatically
impl<T : Clone + Hash + PartialEq, V : Copy> CustomHashMap<T,V> {
    const INIT : Option<KeyValue<T,V>> = None;      // we use const to avoid running into borrow/ownership related errors that may arise

    pub fn new() -> CustomHashMap<T,V> {
        CustomHashMap {
            curr_size : 0,
            arr : [Self::INIT; DEFAULT_MAX_SIZE as usize],
        }
    }

    // inserts key and value pair
    pub fn insert(&mut self, key : T, val : V) -> Option<V> {
        // commented out for future reference
        // todo!()         // macro to state it has not yet been implemented

        let hash_val : u64 = hash_key(key.clone());
        let position = hash_val % DEFAULT_MAX_SIZE;

        match &self.arr[position as usize] {

            // update_or_link_new_val is a private method wtihin the implementation
            // what do we do if a value is already present at our index?
            // consider the 2 following scenarios:
            //
            // traverse the linked list until we find a keyvalue with a matching key
            // or we insert our new KeyValue pair onto the end of the Linked List
            // we continue traversing within the current index until we reach the end of the Linked List
            Some(_) => self.update_or_link_new_val(key,val, position as usize),
            None => {
                // insert_new_value is another private method within the implementation
                // append a new node to the Linked List to the end of the LL once we have arrived at the end
                // holds three information, the key, the value and the position within which the node is located
                self.insert_new_value(key,val, position as usize);
                None
            }
        }
    }


    // function definition for inserting a new value to the linked list
    fn insert_new_value(&mut self, key : T, val : V, position : usize) {

        // calls upon the KeyValue's constructor to create a new key and value pair
        // which is then set to the current position of the array (essentially an array of linked list)
        let new_entry = KeyValue::new(key,val);

        self.arr[position] = Some(new_entry);
        self.curr_size += 1;
    }

    fn update_or_link_new_val(&mut self, key : T, val : V, position : usize) -> Option<V> {
        // Traverse linked list until either find value and update it (meaning the node already exists)
        // or insert a new value at the end of the linked list
        //
        // can safely unwrap because we have verified that this position exists
        // grab the key value at a particular position in order to check if they matches
        //
        // since the nodes stores the key and values as pairs
        // meaning key_val.key and key_val.value are both feasible as they are part of the KeyVal struct
        let key_val = self.arr[position].as_mut().unwrap();

        // meaning the key already exists within the head node, and we need to update the existing value corresponding to the key
        if key_val.key == key {

            // store the old value within this variable
            let old_val = key_val.value;

            // update the current key's corresponding value
            // with the new value with the value that is passed onto the parameter
            key_val.value = val;

            return Some(old_val);     // return the old value
        }

        // otherwise, if the key doesn't exist in the first node
        // it may exists within the middle of the Linked list instead
        // we need to create a new linked list and attach it to the end
        //
        // is_some() : returns True if the option is a Some value.
        // iterates through the linked list using is_some() to check if we've reached the end of the linked list
        //
        // for every existing node, the following checks needs to be performed 
        // 1. if that node's key matches ours
        // 2. if it does, we update the value and return the old value held there
        // 3. if not, move to the next node
        let mut current = key_val;
        while current.next.is_some() {          // linked list traversal logic
            let node = current.next.as_mut().unwrap();

            if node.key == key {
                // update the value 
                let old_val = node.value;
                node.value = val;
                return Some(old_val);
            }

            current = node;    
        }

        // append the new value to the end of the linked list
        let new_key_val = KeyValue::new(key,val);
        current.next = Some(Box::new(new_key_val));

        self.curr_size += 1;        // since linked list has been increased by 1, we also need to increment the current size

        None

    }



    // retrieves a value given a key
    // grab values from the hashmap
    //
    // same approach is applied, hashing our key to find an index of the array to use (this is the core)
    pub fn get(&self, key : T) -> Option<V> {
        let hash_val : u64 = hash_key(key.clone());
        let position = hash_val % DEFAULT_MAX_SIZE;
        
        // after obtaining the position, the goal is to check the value at the particular index
        // check_list_for_key() is a private method within the hashmap (implemented below)
        // the method takes in the key that we are looking for within a particular position to check if it exists
        // _ serves as a placeholder for return value that isn't None
        match &self.arr[position as usize] {
            Some(_) => self.check_list_for_key(key, position as usize),
            None => None,       // if there's no value, simply return None
        }  
    }

    fn check_list_for_key(&self, key : T, position : usize) -> Option<V> {
        let mut current = self.arr[position].as_ref().unwrap();
        if current.key == key {
            return Some(current.value);
        }

        // we are using as_ref() instead of using as_mut()
        // since we don't need to mutate any of the values
        while let Some(node) = current.next.as_ref() {
            if node.key == key {
                return Some(node.value);
            }

            current = node;
        }

        None
    }

    // removes the key-value pair from the map for a given key, return the value if key existed
    // None otherwise
    //
    // again, we hash the key and retrieve the remainder of DEFAULT_MAX_SIZE to retrieve the position
    // if that position is empty, we return None, otherwise we traverse through the linked list looking for the particular value
    //
    // the logic of removing from a hashmap underneath is the same as removing from a linked list
    // in order to remove a value from a Linked List the following steps must be followed:
    // 1. check if the KeyValue being removed has a next value
    // 2. if it does, we set the next value of the "previous" KeyValue to point to this KeyValue
    // 3. If the KeyValue is being removed is the first value, we set the value in the array to be the next value, if it exists
    pub fn remove(&mut self, key : T) -> Option<V> {
        let hash_val : u64 = hash_key(key.clone());
        let position : u64 = hash_val % DEFAULT_MAX_SIZE;

        // fairly straightforward logic, we once again check within the position of the array
        // check if the node exists and call on the helper function
        // the helper in this case being check_item_in_list_and_remove
        match &self.arr[position as usize] {
            Some(_) => self.check_item_in_list_and_remove(key, position as usize),
            None => None,           // same as not doing anything since the key doesn't exist
        }
    }

    // private helper function for the remove method 
    fn check_item_in_list_and_remove(&mut self, key : T, position : usize) -> Option<V> {
        let mut current = self.arr[position].as_mut().unwrap();

        // check if the first node's key matches the value
        // if it does, we save the value it held so that we can return it later
        //
        // we then check if next exists, if it doesn't, we simply update that position to be None
        // if it does exist, we set that position to be the address of that node (hence the use of *)
        if current.key == key {
            let return_val = current.value;

            // check if there is a next value
            // if there is next, update array to point to this particular value
            // this conditional will continue to execute while current.next.to_owned() returns Some(node)
            if let Some(node) = current.next.to_owned() {
                self.arr[position] = Some(*node);
            } else {
                self.arr[position] = None
            }

            // return the value the node held
            self.curr_size -= 1;
            return Some(return_val);
        }

        // iterate through until key is found
        // 
        // The following steps are being performed within the code:
        // 1. we check if the next KeyValue is not None.
        // 2. if it isn't None, we check if it's key matches the key we're looking for.
        // 3. if it does, we store the return value to return later.
        // 4. we then check if the next KeyValue after this is also a value.
        // 5. if it is, we set our current value to point to this "next next" value.
        // 6. if there's no "next next" value, we set our current value to point to None.
        // 7. we then decrement the current size and return the value.
        while current.next.is_some() {
            let next = current.next.as_mut().unwrap();

            // if the key has been found
            if next.key == key {

                // set the value
                let return_val = next.value;

                // check if there's a next val
                // if there is next, update array to point ot this val
                if let Some(next_next) = next.next.to_owned() {
                    current.next = Some(Box::new(*next_next));
                } else {
                    current.next = None         // otherwise, if next doesn't exist, have it point to None
                }

                // return the value the node held
                // shrink the size of the linked list by 1 as well
                self.curr_size -= 1;
                return Some(return_val);
            }

            // set current equal to the next
            current = current.next.as_mut().unwrap();
        }


        // if we reach the end of the Linked list with no value, simply return None
        None

    }

    // clears the hashmap
    // similar to the drain method within the std::collections::CustomHashMap
    //
    // overwrite the existing array to remove everything
    // the definition is essentially similar to the constructor method
    pub fn clear(&mut self) {
        self.curr_size = 0;        
        self.arr = [Self::INIT; DEFAULT_MAX_SIZE as usize];        
    }

    pub fn len(&self) -> usize {
        self.curr_size
    }

    pub fn is_empty(&self) -> bool {
        if self.len() == 0 {
            return true;
        }
        false
    }
}

// hash_key implementation
// .hash() method comes from the Hash trait that has been attached to generic type T
fn hash_key<T : Hash>(key : T) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    let hash_val = hasher.finish();
    hash_val
}


// we can run tests to check if the hashmap works
// tested within rust playground
#[test]
fn test_can_get_item() {
    // Function to check if the .get() method works
    let key = "hello".to_string();
    let value : i32 = 1;

    // instantiate a hashmap of key type string and value of i32
    let mut my_hash : CustomHashMap<String, i32> = CustomHashMap::new();
    my_hash.insert(key.clone(), value);

    let result = my_hash.get(key).unwrap();
    assert_eq!(result,value)
}


// Tested in rust playground
// fn main() {
//     let mut my_hashmap : CustomHashMap<i32,i32> = CustomHashMap::new();
//     let key = 1;
//     let val = 10;
    
//     my_hashmap.insert(key.clone(), val);
//     // println!("{:?}", my_hashmap.get(key).unwrap());
    
//     my_hashmap.clear();
//     println!("{:?}", my_hashmap);
    
// }