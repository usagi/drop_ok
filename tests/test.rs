use drop_ok::DropOk;

type MyError = i32;
const EXPECTED_ERR_VALUE: MyError = -1;

#[test]
fn test()
{
 let x = ok_value_to_unit();

 // It is NOT drop `Err` value.
 assert_eq!(x.drop_ok(), Err(EXPECTED_ERR_VALUE));
}

fn ok_value_to_unit() -> Result<String, MyError>
{
 // If it would be compilable then `.drop_ok()` is work expectedly.
 // Because, `match` must be return one value that is the same type for any patterns.
 match "something"
 {
  // Result<i8, MyError> -> Result<(), MyError> -> ()
  "pattern-x" => some_task1().drop_ok()?,
  // Result<'a &str, MyError> -> Result<(), MyError> -> ()
  "pattern-y" => some_task2().drop_ok()?,
  // Result<(), MyError> -> ()
  "pattern-z" => some_task3()?,
  // () from the function
  "pattern-w" => some_task4(),
  // () defined directly
  _ => ()
 }

 Err(EXPECTED_ERR_VALUE)
}

fn some_task1() -> Result<i8, MyError>
{
 Ok(1)
}

fn some_task2<'a>() -> Result<&'a str, MyError>
{
 Ok("abc")
}

fn some_task3() -> Result<(), MyError>
{
 Ok(())
}

fn some_task4() -> ()
{
 ()
}
