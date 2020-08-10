# drop_ok

- This crate defines `DropOk<E>` trait and `impl` for `Result`.
- `DropOk<E>` trait has `.drop_ok(self) -> Result<(), E>` function.
- This is a syntax sugar for `.map(|_|())`.

## Useful scene

```rust
fn some_function() -> Result<(), MyError>
{
 match switcher
 {
  pattern_a => some_task1().drop_ok()?
  pattern_b => some_task2().drop_ok()?
  pattern_c => some_task3()?
  pattern_d => some_task4()
  _ => ()
 }
}

fn some_task1()     -> Result<i8     , MyError> { /* abbr */ }
fn some_task2<'a>() -> Result<&'a str, MyError> { /* abbr */ }
fn some_task3()     -> Result<()     , MyError> { /* abbr */ }
fn some_task4() { /* abbr */ }
```

- See also: <tests/test.rs>

## Note

> "I don't need it, because ..."

- Yes, your are right to your world.
    - But, I tired to type/see `.map(|_|())`. So I'm happy with`.drop_ok()`.

## LICENSE

- [MIT](LICENSE.md)

## Author

- USAGI.NETWORK / Usagi Ito <https://github.com/usagi/>
