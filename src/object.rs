use std::fmt::{Display, Formatter};



/// Wrapper for anything type
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct Object {
    ptr: *mut (),
}


impl Object {
    pub fn new<T>(val: *mut T) -> Self {
        Self {
            ptr: val as *mut ()
        }
    }


    /// Returns the cloned value from struct
    /// ------------------------------------
    /// *If (T != type of value in struct) => ???*
    /// Example:
    /// ```
    /// let int_object = object_type::obj!(6982_i32);
    /// let _ = int_object.get::<i32>(); // Ok
    /// // let _ = int_object.get::<i64>(); => ???
    /// ```
    pub fn get<T: Clone>(&self) -> T {
        unsafe { self.__get_unsafe() }
    }

    unsafe fn __get_unsafe<T: Clone>(&self) -> T {
        (*(self.ptr as *mut T)).clone()
    }


    /// Returns the value pointer casted to <*T*> type
    /// --------------------------------------------
    /// *If (T != type of value in struct) => ???*
    /// Example:
    /// ```
    /// let int_object = object_type::obj!(6982_i32);
    /// let _ = int_object.get_ptr::<i32>(); // Ok
    /// // let _ = int_object.get_ptr::<i64>(); => ???
    /// ```
    pub fn get_ptr<T>(&self) -> *mut T {
        unsafe { self.__get_ptr_unsafe() }
    }

    unsafe fn __get_ptr_unsafe<T>(&self) -> *mut T {
        self.ptr as *mut T
    }


    /// Returns the raw mutable pointer to value as unit type
    pub fn raw(&self) -> *mut () {
        self.ptr
    }


    /// Unstable? 
    ///
    /// Example:
    /// ```
    /// let int_object = object_type::obj!(2372_i16);
    /// println!("=> {}", int_object.__value_to_string::<i16>());
    /// ```
    pub fn __value_to_string<T: ToString>(&self) -> String {
        unsafe { self.__value_to_string_unsafe::<T>() }
    }

    unsafe fn __value_to_string_unsafe<T: ToString>(&self) -> String {
        (*self.get_ptr::<T>()).to_string()
    }


    pub fn equals<T>(&self, other: T) -> bool
        where T: PartialEq + Clone,
    {
        other.eq(&self.get::<T>())
    }
}


impl Default for Object {
    fn default() -> Self {
        Self { ptr: std::ptr::null_mut(), }
    }
}


impl Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Object({:?})", self.ptr)
    }
}



/// Returns an instance of Object
/// -----------------------------
/// Example:
/// ```
/// let object = object_type::obj!("str test");
/// ```
#[macro_export]
macro_rules! obj {
    ($val:expr) => {
        $crate::Object::new(&mut $val)
    };
    () => {
        $crate::Object::default()
    }
}





