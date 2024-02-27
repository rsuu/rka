use rka::*;

fn main() {
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

    draft! {
        start()
            .{ add(this) }
            .test::<u32>()
            .{
                this.push(1);

                this
            }
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

    let vec = postfix_block! {
        vec![]
            .this({
                this.push(1);
            })
            .this({
                this.push(2);
            })
            .pipe({
                vec![this[0], this[1], 3, 4]
            })
    };
    dbg!(vec);
}
