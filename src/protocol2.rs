/// https://disco.process.io/dummy
/// f0: () -> T0 T1.
/// f1: T1 -> T2.
/// f2: T0 -> T3 T4.
/// f3: T3 T2 -> T5.
/// f4: T5 T4 -> ().

#[derive(Default, Debug)]
struct Empty;

trait Produce<T>: Sized {
    fn produce(_: T) -> Self;
}
trait Consume<T>: Sized {
    fn consume(self) -> T;
}
impl<T, U> Consume<U> for T
where
    U: Produce<T>,
{
    fn consume(self) -> U {
        U::produce(self)
    }
}
// impl<T, U> Produce<T> for U
// where
//     U: Consume<T>,
// {
//     fn produce(t: T) -> U {
//         T::consume(t)
//     }
// }
// impl<T, U> Produce<U> for T
// where
//     T: Consume<U>,
// {
//     fn produce(self) -> U {
//         T::consume(self)
//     }
// }
impl<T> Produce<T> for T {
    fn produce(t: T) -> T {
        t
    }
}

// petrinet definition
trait F0<T0, T1, Tuple: Produce<Empty>, Tuple2: Consume<(T0, T1)>> {}
trait F1<T1, T2: Produce<T1>> {}
trait F2<T0: Consume<(T3, T4)>, T3, T4> {}
trait F3<T3, T2, T5: Produce<(T3, T2)>> {}
trait F4<T4, T5, Tuple: Produce<(T4, T5)> + Consume<Empty>> {}
trait F5<T3, T1, T2, T6, Tuple: Produce<(T3, T1)> + Consume<(T2, T6)>> {}

fn main() {
    #[derive(Default, Debug)]
    struct A0;
    #[derive(Default, Debug)]
    struct A1;
    #[derive(Default, Debug)]
    struct A2;
    #[derive(Default, Debug)]
    struct A3;
    #[derive(Default, Debug)]
    struct A4;
    #[derive(Default, Debug)]
    struct A5;
    #[derive(Default, Debug)]
    struct A6;

    #[derive(Default, Debug)]
    struct PetriState;

    impl F0<A0, A1, (A0, A1), (A0, A1)> for PetriState {}

    impl Produce<Empty> for (A0, A1) {
        fn produce(_: Empty) -> (A0, A1) {
            dbg!(Default::default())
        }
    }

    // impl F1<A1, A2> for PetriState {}

    impl Produce<A1> for A2 {
        fn produce(_: A1) -> A2 {
            dbg!(Default::default())
        }
    }

    // impl F2<A0, A3, A4> for PetriState {}

    impl Produce<A0> for (A3, A4) {
        fn produce(_: A0) -> (A3, A4) {
            dbg!(Default::default())
        }
    }

    // impl F3<A3, A2, A5> for PetriState {}

    impl Produce<(A3, A2)> for A5 {
        fn produce(_: (A3, A2)) -> A5 {
            dbg!(Default::default())
        }
    }

    // impl F4<A4, A5, (A4, A5)> for PetriState {}

    impl Produce<(A4, A5)> for Empty {
        fn produce(_: (A4, A5)) -> Empty {
            dbg!(Default::default())
        }
    }


    // impl F5<A3, A1, A2, A6, (A3, A1)> for PetriState {}
    impl Produce<(A3, A1)> for (A2, A6) {
        fn produce(_: (A3, A1)) -> (A2, A6) {
            dbg!(Default::default())
        }
    }

    // impl Consume<(A2, A6)> for (A3, A1) {
    //     fn into(self) -> (A2, A6) {
    //         dbg!(Default::default())
    //     }
    // }

    let init: Empty = Default::default();
    let (a0, a1): (A0, A1) = init.consume() ;
    // let a2: A2 = a1.consume();
    let (a3, a4): (A3, A4) = a0.consume();
    let (a2, a6): (A2, A6) = (a3, a1).consume();
    let init: Empty = Default::default();
    let (a0, a1): (A0, A1) = init.consume() ;
    let (a3, a4): (A3, A4) = a0.consume();
    // let a5 = (a3, a2).consume();
    let a5: A5 = A5::produce((a3, a2));
    let _: Empty = (a4, a5).consume();

}
