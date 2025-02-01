//! Course storage implementations with minor optimizations
use std::collections::{HashMap};
use crate::models::{CourseInfo, CompactCourseInfo};     // absolute import from models dir
use crate::utils::StringInterner;                       // absolute import from utils dir
// use lru::LruCache;
// mod dataStructures;
use crate::custom_hashmap::CustomHashMap;
use crate::custom_lru_cache::CustomLruCache;
// use crate::data_structures::{custom_hashmap, custom_lru_cache};
use std::marker::Copy;
// use std::num::NonZeroUsize;

const BLOCK_SIZE : usize = 16;      // global 

/// Main storage structure for course data
pub struct CourseStorage {
    // primary storage in blocks for cache utilization
    // Block is a struct defined below
    pub blocks : Vec<Option<Block<CompactCourseInfo>>>,

    // string interner for de-duplication
    pub string_interner : StringInterner,

    // indices for fast lookups
    pub code_index : CustomHashMap<u32, usize>,
    pub id_index : CustomHashMap<String, usize>,

    // cache for frequently accessed items
    pub cache : CustomLruCache<usize, CourseInfo>,
}

#[derive(Clone,Debug)]
pub struct Block<T> {
    data : Box<[T; BLOCK_SIZE]>,
    modified : bool,
    access_count : u32,
}


// The Default trait in rust allows the process of defining a "default" value for a type
// similar to "defaultdict" in python
impl<T> Block<T> 
where 
    T: Default + Copy,
{
    fn new() -> Self {
        Block {
            data: Box::new([T::default(); BLOCK_SIZE]),
            modified: false,
            access_count: 0,
        }
    }
}

impl CourseStorage {
    // Constructor Definition below
    /// Creates a new CourseStorage instance
    /// 
    /// # Arguments
    /// # `capacity` - Initial capacity for the storage
    /// 
    /// # Examples
    /// ```ruse
    /// let storage = CourseStorage::new(1000);
    /// ```
    pub fn new(capacity: usize) -> Self {
        CourseStorage {
            blocks: Vec::new(),
            string_interner: StringInterner::new(),
            code_index: CustomHashMap::new(),
            id_index: CustomHashMap::new(),
            cache: CustomLruCache::new(100), // Cache last 100 items
        }
    }

    /// Inserts a course into storage
    /// 
    /// # Arguments
    /// * `course` - Course information to store
    /// 
    /// # Examples
    /// ```rust
    /// storage.insert(course_info);
    /// ```
    pub fn insert(&mut self, course : CourseInfo) {
        // string interning is a method of storing only one copy
        // of each distinct string value, which must be immutable
        //
        // nterning strings makes some string processing tasks more time-efficient or space-efficient
        // at the cost of requiring more time when the string is created or interned
        // The distinct values are stored in a string intern pool.
        let name_id = self.string_interner.intern(course.course_name.clone());
        let dept_id = self.string_interner.intern(course.department[0].clone());

        // create compact version
        // TODO : need to implement from_course_info() method for this crate
        let compact = CompactCourseInfo::from_course_info(
            &course,
            name_id,
            dept_id
        );

        // get or create block
        let block_id = self.get_or_create_block();

        // Store in block
        if let Some(block) = &mut self.blocks[block_id] {
            // Find empty slot in block
            for i in 0..BLOCK_SIZE {
                if block.data[i].is_empty() {
                    block.data[i] = compact;
                    block.modified = true;
                    block.access_count += 1;
                    
                    // NOTE : code_index and id_index are both hashmaps here
                    // update indices
                    self.code_index.insert(compact.course_code, block_id);
                    self.id_index.insert(course.unique_id.clone(), block_id);

                    // update cache
                    self.cache.insert(block_id, course);
                    return;
                }
            }
        }
        
    }

    // retrieves information about course based on code ID
    // TODO : fix the implementation of this function (since the code isn't something that's entirely numerical values within string)
    // (i.e MATH 391, CSC 212, etc.)
    //
    // TODO : implement a function to parse the numerical and non-numerical values
    // need to parse and filter out the non-numerical values and numerical values
    pub fn get_by_code(&mut self, code : &str) -> Option<CourseInfo> {
        // convert code into numeric format
        let code_num = self.pack_course_code(code);

        // check cache first
        if let Some(&block_id) = self.code_index.get(code_num) {
            if let Some(course) = self.cache.get(&block_id) {
                return Some(course.clone());
            }

            // not in cache, get from block
            if let Some(block) = &mut self.blocks[block_id] {
                block.access_count += 1;        // increment by 1 since particular data within block has been accessed

                // Find in block
                for item in block.data.iter() {
                    if item.course_code == code_num {
                        // convert back into course info
                        // using the helper function to_course_info
                        let course = item.to_course_info(&self.string_interner);

                        // update cahce
                        self.cache.insert(block_id, course.clone());
                        return Some(course);
                    }
                }
            }
        }
        None
    }

    pub fn get_by_id(&mut self, id : &str) -> Option<CourseInfo> {
        if let Some(&block_id) = self.id_index.get(id.to_string()) {
            if let Some(course) = self.cache.get(&block_id) {
                return Some(course.clone());
            }

            if let Some(block) = &mut self.blocks[block_id] {
                block.access_count += 1;

                // Find in block
                for item in block.data.iter() {
                    if item.unique_id == id {
                        let course = item.to_course_info(&self.string_interner);
                        self.cache.insert(block_id, course.clone());
                        return Some(course);
                    }
                }
            }
        }
        None
    }
}

