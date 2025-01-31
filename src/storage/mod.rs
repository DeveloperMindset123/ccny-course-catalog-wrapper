//! Course storage implementations with minor optimizations
use std::collections::{HashMap};
use crate::models::{CourseInfo, CompactCourseInfo};     // absolute import from models dir
use crate::utils::StringInterner;                       // absolute import from utils dir
// use lru::LruCache;
use crate::dataStructure::custom_hashmap::CustomHashMap;
use crate::dataStructure::custom_lru_cache::CustomLruCache;
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
    pub code_index : HashMap<u32, usize>,
    pub id_index : HashMap<String, usize>,

    // cache for frequently accessed items
    pub cache : LruCache<usize, CourseInfo>,
}

#[derive(Clone,Debug)]
pub struct Block<T> {
    data : Box<[T; BLOCK_SIZE]>,
    modified : bool,
    access_count : u32,
}

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

        // store in block
        // TODO : continue this implementation tommorow.
        
    }

    // Additional methods...
}

