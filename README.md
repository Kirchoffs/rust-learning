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

## MapWhile
```
fn map_while<B, P>(self, predicate: P) -> MapWhile<Self, P>
where
    Self: Sized,
    P: FnMut(Self::Item) -> Option<B>,
```

`map_while()` takes a closure as an argument. It will call this closure on each element of the iterator, and yield __elements__ while it returns `Some(_)`.

## AsRef
```
pub trait AsRef<T>
where
    T: ?Sized,
{
    fn as_ref(&self) -> &T;
}
```
A struct which implemenst AsRef<T> can be used as a reference to T by `as_ref` method.

Notice that as_ref in Result is not for AsRef trait, it is for &Result<T, E> to Result<&T, &E>.

## Sized
Types with a constant size known at compile time.  
All type parameters have __an implicit bound of Sized__. The special syntax ?Sized can be used to remove this bound if it’s not appropriate.  
The one exception is the implicit Self type of a trait. A trait does not have an implicit Sized bound as this is incompatible with trait objects where, by definition, the trait needs to work with all possible implementors, and thus could be any size.

### Unsized Type
Most types have a particular size, in bytes, that is knowable at compile time. For example, an i32 is thirty-two bits big, or four bytes. However, there are some types which are useful to express, but do not have a defined size. These are called ‘unsized’ or ‘dynamically sized’ types. One example is [T]. This type represents a certain number of T in sequence. But we don’t know how many there are, so the size is not known.

Rust understands a few of these types, but they have some restrictions. There are three:

- We can only manipulate an instance of an unsized type via a pointer. An &[T] works fine, but a [T] does not.
- Variables and arguments cannot have dynamically sized types.
- Only the last field in a struct may have a dynamically sized type; the other fields must not. Enum variants must not have dynamically sized types as data.
