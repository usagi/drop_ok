/// Result<????, E> -> Result<(), E>
pub trait DropOk<E>
{
 /// Only drop Ok value, no drop Err value.
 /// # Example

 fn drop_ok(self) -> Result<(), E>;
}

impl<V, E> DropOk<E> for Result<V, E>
{
 fn drop_ok(self) -> Result<(), E>
 {
  self.map(|_| ())
 }
}
