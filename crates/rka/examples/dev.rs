use rka::*;

fn main() {
    test_dur();
    test_postfix_block();
}

draft! {
    struct Inject {}
}

fn test_dur() {
    let v = Some(123_u32);

    let k: usize = ({
        match v {
            Some(a) => match a.try_into() {
                // if .next() == ';'
                //   b
                // else match
                //   ...
                Ok(b) => b,
                _ => return,
            },
            _ => return,
        }
    });
    dbg!(k);

    {
        let l1 = '__rka_tmp_scope_1: loop {
            '__rka_tmp_scope_2: loop {
                let a = 1;
                dbg!(a);

                '__rka_tmp_scope_3: loop {
                    // return 'fn;
                    //return;
                    break '__rka_tmp_scope_3;
                }

                // return '__rka_tmp_scope_2;
                //     // expand
                break '__rka_tmp_scope_2;
            }

            break '__rka_tmp_scope_1 "l1";
        };
        dbg!(l1);
    }
}

fn test_sleep() {
    sleep!(3s);
}

fn test_postfix_block() {
    fn start() -> usize {
        0
    }
    fn add1(v: &mut usize) {
        *v += 1;
    }
    fn sub(this: usize, other: usize) -> usize {
        this - other
    }

    trait Test: Sized {
        fn test<T>(self, v: T) -> Self {
            self
        }
    }
    impl<T> Test for T {}

    // impl this if possable
    draft! {
        start()
            .{ add(this) }
            .test::<u32>()
            .{
                let b = this;
                sub(this, b)
            }
            .to_owned()
    }

    // .this(...)
    let res = postfix_block! {
        start()
            .this() // 0
            .this() // 0
    };
    assert_eq!(res, 0);

    // .pipe(...)
    let res = postfix_block! {
        start()
            .pipe({ 1 }) // 1
            .pipe({ 2 }) // 2
    };
    assert_eq!(res, 2);

    let res = postfix_block! {
        start()
            .this({
                add1(&mut this); // 1
                add1(&mut this); // 2
            })
            .test::<u8>(123)
            .to_owned()
            .pipe({ 10 })        // 10
            .this({
                add1(&mut this); // 11
                this -= 6;       // 5
            })
            .to_owned()
    };
    assert_eq!(res, 5);
}
