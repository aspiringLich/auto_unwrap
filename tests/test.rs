#[macro_use]
extern crate auto_unwrap;

#[test]
#[auto_unwrap]
fn test() {
    let opt: Option<f32> = Some(2.0);
    assert!(opt? == 2.0);
}

#[test]
#[should_panic]
#[auto_unwrap]
fn panic_test() {
    let opt: Option<()> = None;
    let _panic = opt?;
}

#[test]
#[auto_unwrap]
fn ignore_test() {
    #[skip_auto_default]
    let closure = || -> Result<u32, f32> {
        let ok: Result<u32, f32> = Ok(1);
        assert_eq!(ok?, ok.unwrap());

        let err: Result<u32, f32> = Err(2.0);
        assert_eq!(err?, err.unwrap());

        Ok(2)
    };
    assert_eq!(closure(), Err(2.0));
}

#[test]
#[auto_unwrap]
fn skip_test() {
    #[skip_auto_default]
    let closure = || -> Result<u32, f32> {
        let ok: Result<u32, f32> = Ok(1);
        assert_eq!(ok?, ok.unwrap());

        let err: Result<u32, f32> = Err(2.0);
        assert_eq!(err?, err.unwrap());

        Ok(2)
    };
    assert_eq!(closure(), Err(2.0));
}
