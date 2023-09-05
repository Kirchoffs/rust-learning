# rust-details

## Rc & Arc
Rc is not thread-safe and can only be used in single-threaded scenarios. It cannot be safely shared across multiple threads because it does not implement any synchronization mechanisms to handle concurrent access. As a result, Rc is generally used for single-threaded scenarios where multiple owners of data are required.

Arc, on the other hand, is designed to be thread-safe and can safely be shared among multiple threads. It implements atomic reference counting, which ensures that references can be safely shared across threads. This makes Arc suitable for concurrent programming.

## BufRead::lines & BufRead.read_line
__BufRead::lines__  
Each string returned will not have a newline byte (the 0xA byte) or CRLF (0xD, 0xA bytes) at the end.

__BufRead.read_line__  
It will read bytes from the underlying stream until the newline delimiter (the 0xA byte) or EOF is found. Once found, all bytes up to, and including, the delimiter (if found) will be appended to buf

## Option
### cloned
```
pub fn cloned<T>(&self) -> Option<T> where T: Clone
```
The cloned method for Option is used to create a new Option where the inner value, if present and of a type that implements the Clone trait, is cloned into a new Option. This method is useful when you have an Option<&T> and you want to create an `Option<T>`, where T is a type that implements Clone.


### copied
```
pub fn copied<T>(&self) -> Option<T> where T: Copy
```
The copied method for Option is used to create a new Option where the inner value, if present and of a type that implements the Copy trait, is copied into a new Option. This method is useful when you have an Option<&T> and you want to create an `Option<T>`, where T is a type that implements Copy.

### Copy & Clone
- Copy is implicit, inexpensive, and cannot be re-implemented (memcpy).
- Clone is explicit, may be expensive, and may be re-implement arbitrarily.
