use rka::*;

fn main() {
    test_dur();
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
}

fn test_sleep() {
    sleep!(3s);
}

draft! {
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
