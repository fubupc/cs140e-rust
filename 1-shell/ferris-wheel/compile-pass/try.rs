// FIXME: Make me compile. Diff budget: 12 line additions and 2 characters.
struct ErrorA;
struct ErrorB;

enum Error {
    A(ErrorA),
    B(ErrorB)
}

fn do_a() -> Result<u16, ErrorA> {
    Err(ErrorA)
}

fn do_b() -> Result<u32, ErrorB> {
    Err(ErrorB)
}

fn do_both() -> Result<(u16, u32), Error> {
    let a = match do_a() {
        Err(e) => return Err(Error::A(e)),
        Ok(a) => a,
    };

    let b = match do_b() {
        Err(e) => return Err(Error::B(e)),
        Ok(b) => b,
    };

    Ok((a, b))
}

fn main() { }
