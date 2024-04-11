use std::borrow::{Borrow, Cow};

// examples:
// &'a str <-> Cow<'a, str> <-> String
// (&'a str, u32) <-> (Cow<'a, str>, u32) <-> (String, u32)
// u32 <-> u32 <-> u32
// String <-> String <-> String

pub trait ToOwnable {
    type Borrowed<'a>: 'a;
    type Ownable<'a>: ToStatic<'a> + 'a;

    fn to_ownable<'a>(borrowed: Self::Borrowed) -> Self::Ownable<'a>;

    fn from_ownable<'a>(ownable: Self::Ownable<'a>) -> Self;
}

pub trait ToStatic<'a>: 'a {
    fn to_static(self) -> Self::Static;

    fn from_static(s: &Self::Static) -> Self;
}

// impl <T: 'static> ToOwnable for T {
//     type Borrowed<'a> = T;
//     type Ownable<'a> = T;
//
//     fn to_ownable<'a>(borrowed: Self::Borrowed<'static>) -> Self::Ownable<'a> {
//         borrowed
//     }
//
//     fn from_ownable<'a>(ownable: Self::Ownable<'a>) -> Self {
//         ownable
//     }
// }

impl <'a, T: ToOwned> ToOwnable for T {
    type Borrowed<'a> = T;
    type Ownable<'a> = Cow<'a, T>;

    fn to_ownable<'a>(borrowed: Self::Borrowed<'static>) -> Self::Ownable<'a> {
        borrowed
    }

    fn from_ownable<'a>(ownable: Self::Ownable<'a>) -> Self {
        ownable
    }
}

// pub trait Ownable {
//     type Borrowed<'a>: 'a;
//     type Ownable<'a>: ToStatic + 'a;
//
//     fn ownable<'a>(borrowed: Self::Borrowed<'a>) -> Self::Ownable<'a>;
//
//     fn borrowed<'a>(ownable: Self::Ownable<'a>) -> Self::Borrowed<'a>;
// }

//
// impl <T: 'static> ToStatic for T {
//     type Static = T;
//
//     fn to_static(self) -> Self::Static {
//         self
//     }
// }
//
// impl <'a, T: ToStatic + ToOwned> ToStatic for Cow<'a, T> {
//     type Static = ToOwned::Owned;
//
//     fn to_static(self) -> Self::Static {
//         self.into_owned()
//     }
// }
//
// impl <T: ToOwned> Ownable for T {
//     type Borrowed<'a> = &'a T;
//     type Ownable<'a> = Cow<'a, T>;
//
//     fn ownable<'a>(borrowed: Self::Borrowed<'a>) -> Self::Ownable<'a> {
//         Cow::Borrowed(borrowed)
//     }
// }
//
// impl <A: ToOwned, B: ToOwned> Ownable for (A, B) {
//     type Borrowed<'a> = (&'a A, &'a B);
//     type Ownable<'a> = (Cow<'a, A>, Cow<'a, B>);
//
//     fn ownable<'a>(borrowed: Self::Borrowed<'a>) -> Self::Ownable<'a> {
//         (Cow::Borrowed(borrowed.0), Cow::Borrowed(borrowed.1))
//     }
// }
//
// impl <A: ToOwned, B: ToOwned, C: ToOwned> Ownable for (A, B, C) {
//     type Borrowed<'a> = (&'a A, &'a B, &'a C);
//     type Ownable<'a> = (Cow<'a, A>, Cow<'a, B>, Cow<'a, C>);
//
//     fn ownable<'a>(borrowed: Self::Borrowed<'a>) -> Self::Ownable<'a> {
//         (Cow::Borrowed(borrowed.0), Cow::Borrowed(borrowed.1), Cow::Borrowed(borrowed.2))
//     }
// }

// impl <A: ToOwned, B: ToOwned, C: ToOwned, D: ToOwned> Ownable for (A, B, C, D) {
//     type Borrowed<'a> = (&'a A, &'a B, &'a C, &'a D);
//     type Ownable<'a> = (Cow<'a, A>, Cow<'a, B>, Cow<'a, C>, Cow<'a, D>);
//
//     fn ownable<'a>(borrowed: Self::Borrowed<'a>) -> Self::Ownable<'a> {
//         (Cow::Borrowed(borrowed.0), Cow::Borrowed(borrowed.1), Cow::Borrowed(borrowed.2), Cow::Borrowed(borrowed.3))
//     }
// }

fn cow_to_static<'a, T: ToOwned + 'static>(cow: Cow<'a, T>) -> Cow<'static, T> {
    match cow {
        Cow::Borrowed(b) => Cow::Owned(b.to_owned()),
        Cow::Owned(o) => Cow::Owned(o),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cow_to_static() {
        let cow = Cow::Borrowed("hello");
        let static_cow = cow_to_static(cow);
        assert_eq!(static_cow, Cow::Owned("hello".to_string()));

        let cow = Cow::Owned("hello".to_string());
        let static_cow = cow_to_static(cow);
        assert_eq!(static_cow, Cow::Owned("hello".to_string()));
    }
}