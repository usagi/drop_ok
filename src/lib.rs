/// Result<????, E> -> Result<(), E>
pub trait DropOk<E>
{
 /// Only drop Ok value, no drop Err value.
 /// # Example
 /// ```
 /// use drop_ok::DropOk;
 ///
 /// struct MyError{}
 ///
 /// fn some_function() -> Result<(), MyError>
 /// {
 ///  match "something"
 ///  {
 ///   "pattern-x" => some_task1().drop_ok()?,
 ///   "pattern-y" => some_task2().drop_ok()?,
 ///   "pattern-z" => some_task3()?,
 ///   "pattern-w" => some_task4(),
 ///   _ => ()
 ///  }
 ///  Ok(())
 /// }
 ///
 /// fn some_task1()     -> Result<i8     , MyError> { Ok(1) }
 /// fn some_task2<'a>() -> Result<&'a str, MyError> { Ok("abc") }
 /// fn some_task3()     -> Result<()     , MyError> { Ok(()) }
 /// fn some_task4() { }
 /// ```
 /// See also: <https://github.com/usagi/drop_ok/tests/test.rs>
 fn drop_ok(self) -> Result<(), E>;
}

impl<V, E> DropOk<E> for Result<V, E>
{
 fn drop_ok(self) -> Result<(), E>
 {
  self.map(|_| ())
 }
}
