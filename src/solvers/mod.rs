// pub mod p1;
// pub use p1::p1;

// pub mod p2;
// pub use p2::p2;

// pub mod p3;
// pub use p3::p3;

// pub mod p4;
// pub use p4::p4;

// pub mod p5;
// pub use p5::p5;

////

// macro_rules! export_solver {
//     ($name:ident) => {
//         pub mod $name;
//         pub use $name::$name;
//     };
// }

// export_solver!(p1);
// export_solver!(p2);
// export_solver!(p3);
// export_solver!(p4);
// export_solver!(p5);

////

macro_rules! export_solvers {
    ($($name:ident),* $(,)?) => {
        $(
            pub mod $name;
            pub use $name::$name;
        )*
    };
}

export_solvers![p1, p2, p3, p4, p5, p6, p7, p8, p9];
