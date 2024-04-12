mod ownedcaps;

use std::borrow::Cow;
use std::borrow::Borrow;

// examples:
// &str <-> Cow<str>
// (&str, u32) <-> (Cow<str>, u32)

// pub trait ToOwnable {
//     type Borrowed<'a>: 'a;
//     type Ownable<'a>: 'a;
//
//     fn to_ownable<'a>(borrowed: Self::Borrowed<'static>) -> Self::Ownable<'a>;
//
//     fn to_borrowed<'a>(ownable: &'a Self::Ownable<'a>) -> Self::Borrowed<'a>;
//
//     fn to_static<'a>(ownable: Self::Ownable<'a>) -> Self::Ownable<'static>;
// }
//
// impl ToOwnable for str {
//     type Borrowed<'a> = &'a str;
//     type Ownable<'a> = Cow<'a, str>;
//
//     fn to_ownable<'a>(borrowed: Self::Borrowed<'static>) -> Self::Ownable<'a> {
//         Cow::Borrowed(borrowed)
//     }
//
//     fn to_borrowed<'a>(ownable: &'a Self::Ownable<'a>) -> Self::Borrowed<'a> {
//         match ownable {
//             Cow::Borrowed(b) => b,
//             Cow::Owned(o) => o.borrow(),
//         }
//     }
//
//     fn to_static<'a>(ownable: Self::Ownable<'a>) -> Self::Ownable<'static> {
//         match ownable {
//             Cow::Borrowed(b) => Cow::Owned(b.to_owned()),
//             Cow::Owned(o) => Cow::Owned(o),
//         }
//     }
// }
//
// impl ToOwnable for u32 {
//     type Borrowed<'a> = u32;
//     type Ownable<'a> = u32;
//
//     fn to_ownable<'a>(borrowed: Self::Borrowed<'static>) -> Self::Ownable<'a> {
//         borrowed
//     }
//
//     fn to_borrowed<'a>(ownable: &'a Self::Ownable<'a>) -> Self::Borrowed<'a> {
//         *ownable
//     }
//
//     fn to_static<'a>(ownable: Self::Ownable<'a>) -> Self::Ownable<'static> {
//         ownable
//     }
// }
//
// fn cow_to_static<'a, T: ToOwned + 'static>(cow: Cow<'a, T>) -> Cow<'static, T> {
//     match cow {
//         Cow::Borrowed(b) => Cow::Owned(b.to_owned()),
//         Cow::Owned(o) => Cow::Owned(o),
//     }
// }
//
// impl <A: ToOwnable> ToOwnable for (A,) {
//     type Borrowed<'a> = (A::Borrowed<'a>,);
//     type Ownable<'a> = (A::Ownable<'a>,);
//
//     fn to_ownable<'a>(borrowed: Self::Borrowed<'static>) -> Self::Ownable<'a> {
//         (A::to_ownable(borrowed.0),)
//     }
//
//     fn to_borrowed<'a>(ownable: &'a Self::Ownable<'a>) -> Self::Borrowed<'a> {
//         (A::to_borrowed(&ownable.0),)
//     }
//
//     fn to_static<'a>(ownable: Self::Ownable<'a>) -> Self::Ownable<'static> {
//         (A::to_static(ownable.0),)
//     }
// }
//
// impl <A: ToOwnable, B: ToOwnable> ToOwnable for (A, B) {
//     type Borrowed<'a> = (A::Borrowed<'a>, B::Borrowed<'a>);
//     type Ownable<'a> = (A::Ownable<'a>, B::Ownable<'a>);
//
//     fn to_ownable<'a>(borrowed: Self::Borrowed<'static>) -> Self::Ownable<'a> {
//         (A::to_ownable(borrowed.0), B::to_ownable(borrowed.1))
//     }
//
//     fn to_borrowed<'a>(ownable: &'a Self::Ownable<'a>) -> Self::Borrowed<'a> {
//         (A::to_borrowed(&ownable.0), B::to_borrowed(&ownable.1))
//     }
//
//     fn to_static<'a>(ownable: Self::Ownable<'a>) -> Self::Ownable<'static> {
//         (A::to_static(ownable.0), B::to_static(ownable.1))
//     }
//
// }
//

// trait Ownable {
//     type Ownable<'a>: 'a + Borrowable<Self>;
// }
//
// trait Borrowable<T> where T: ?Sized {
//     fn to_borrowed(&self) -> &T;
// }
//
// impl <'a> Borrowable<str> for Cow<'a, str> {
//     fn to_borrowed(&self) -> &str {
//         self.borrow()
//     }
// }
//
// impl Ownable for str {
//     type Ownable<'a> = Cow<'a, str>;
// }
//
// impl <A: Ownable, B: Ownable> Ownable for (A, B) {
//     type Ownable<'a> = (A::Ownable<'a>, B::Ownable<'a>);
// }

fn main() {
    // let s = <(str,u32) as ToOwnable>::to_ownable(("abc",0));
    // let s: Cow<'static, str> = "abc".into();
}
