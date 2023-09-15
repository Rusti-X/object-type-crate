# Object type

Object it's type, using a pointer to store any other types  
But, it's not Object<*T*> it's just Object!  
You don't need to specify a type template to create or type an Object.  

And Object is structure, not trait!

You can use:
```
  let vec: Vec<Object> = vec![];  
  vec.push(obj!(472833));            // i32    [0]   
  vec.push(obj!("It's string!"));    // &str   [1]  
  vec.push(obj!(*Your type*));       // Other  [2]  
  // And get value with using get()  
  let string = vec[1].get::<&str>();  
  assert_eq(string, "It's string!");  
```
