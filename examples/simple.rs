use exit::Exit;

#[derive(Debug)]
enum MyErr {
    OneLessThanZero,
    OneEqualsTwo,
}

impl From<MyErr> for i32 {
    fn from(err: MyErr) -> Self {
        match err {
            MyErr::OneLessThanZero => 2,
            MyErr::OneEqualsTwo => 3,
        }
    }
}

fn main() -> Exit<MyErr> {
    if 1 < 0 {
        return Exit::Err(MyErr::OneLessThanZero)
    } else if 1 == 2 {
        return Exit::Err(MyErr::OneEqualsTwo)
    }

    Exit::Ok
}
