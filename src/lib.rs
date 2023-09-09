
// object.rs
mod object;

pub use object::*;



// Test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = obj!("Hello, World!");
        assert_eq!(result.get::<&str>(), "Hello, World!");

        let mut vec: Vec<Object> = vec![obj!(255_u8)];
        vec.push(obj!(obj!("&Str In Object In Object")));
        assert_eq!(vec[0].get::<u8>(), 255_u8);
        assert_eq!(vec[1].get::<Object>().get::<&str>(), "&Str In Object In Object");
    }
}

