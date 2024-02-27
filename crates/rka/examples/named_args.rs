//! implement named arguments

// cargo run --example named_args

use rka::*;

fn main() {
    expand();

    // test named
    // output: 1-2-3
    named_args! {
        fn f(
            arg: u8,
            color @ {
                r: u8,
                g: u8,
                b: u8,


                // TODO:
                //color2 @ RGB { r, g, b }: RGB,
                //
                // falis
                //RGB { r, g, b }: RGB,

            }: impl Default,

        ) {
            println!("{r}-{g}-{b}");
            dbg!(arg, color);
        }

        f(123, @ {
            r: 1,
            g: 2,
            b: 3,
        })
    };

    // test order
    // output: 1-2-3
    named_args! {
        fn f(
            arg: u8,
            color @ {
                r: u8,
                g: u8,
                b: u8,
            }: impl Default,
        ) {
            println!("{r}-{g}-{b}");
            dbg!(arg, color);
        }

        f(123, @ {
            g: 2,
            r: 1,
            b: 3,
        })
    };

    // test trait
    // output: 0-2-0
    named_args! {
        fn f(
            arg: u8,
            color @ {
                r: u8,
                g: u8,
                b: u8,
            }: impl Default,
        ) {
            println!("{r}-{g}-{b}");
            dbg!(arg, color);
        }

        f(123, @ {
            g: 2,
            ..Default::default()
        })
    };
}

fn expand() {
    fn f(arg: u8, r: u8, g: u8, b: u8) {
        let color = (r, g, b);

        println!("{r}-{g}-{b}");
        dbg!(arg, color);
    }

    f(123, 1, 2, <u8 as Default>::default())
}

// REFS: https://github.com/rust-lang/rfcs/issues/323
//       https://internals.rust-lang.org/t/pre-rfc-named-arguments/16413
draft! {
    fn f<T>(
        color: RGB,
        from_scope: u8,

        opt @ {
            from_scope2: &str,
            other: T,
            color2 @ RGB { r, g, b }: RGB,
        }: impl Default,
        opt2 @ {
            from_scope2: &str,
            other: T,
            color2 @ RGB { r, g, b }: RGB,
        }: impl Default,
    )
        where
            T: Debug,
    {
        // ?tuple vs unnamed(...)
        let opt = (from_scope2, other, color2);
    }

    f(
        RGB { ... },
        { 123 },
        opt1 @ {
            from_scope2: { "hi" },
            color2: RGB { ... },
            ..Default::default()
        },
        opt2 @ {
            from_scope2: { "hi" },
            color2: RGB { ... },
            ..Default::default()
        },
    );

    let rgb @ RGB { r, g, b } = RGB { r, g, b };

    // ?
    let opt = @{ r, g, b };
    //
    // no
    //let opt @ { .. } = rgb;
    //
    // ?
    f2(123, opt);
    // ?
    f2(123, @opt);

    // no
    impl Trait for @{} { ... }

    let f_ptr: fn(
        name: String,
        opt @ { val: usize }
        ) = |name: String, opt @{ val: usize }| {
    };


}

draft! {
    fn f(
        arg: u8,
        color1 @ { ... }: ...,
        color2 @ { ... }: ...,
        color3 @ { ... }: ...,
    ) {
        ...
    }

    // ?named named args
    f(arg,
      color1 @ { ... },
      color3 @ { ... },
      color2 @ { ... },
    );

}
