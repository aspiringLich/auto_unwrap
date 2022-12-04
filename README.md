# auto_unwrap

Have you every written a function and were too lazy to have it return `Result<T, E>` but still wanted to use the `?` operator? I present to you:

```rs
use auto_unwrap::auto_unwrap;

#[auto_unwrap]
fn fn_1() -> i32 {
    let s = "does it detect this question mark? (no)";
    println!("{}", s);
    let x: Result<i32, ()> = Ok(23);
    return x?; // gets replaced with x.unwrap();
}

assert_eq!(fn_1(), 23);
```

Is there someplace you would like to keep the `?`?

```rs
use auto_unwrap::auto_unwrap;

#[auto_unwrap]
fn fn_2() {
    #[skip_auto_default] // skips until (and including) the next brace-delimited group or semicolon
    let closure = || -> Result<u32, f32> {
        let ok: Result<u32, f32> = Ok(1);
        assert_eq!(ok?, ok.unwrap());

        let err: Result<u32, f32> = Err(2.0);
        assert_eq!(err?, err.unwrap()); // without the skip this would panic!

        Ok(2)
    };

    assert_eq!(closure(), Err(2.0));
}
```

this is updated from some previous code i wrote: [yauc](https://github.com/aspiringLich/yauc)

Honestly you probably shouldn't use this.
