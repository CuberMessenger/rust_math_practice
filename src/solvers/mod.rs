
macro_rules! export_solvers {
    ($($name:ident),* $(,)?) => {
        $(
            pub mod $name;
            pub use $name::$name;
        )*
    };
}

export_solvers![p1, p2, p3, p4, p5, p6, p7, p8, p9, p10];
export_solvers![p11, p12, p13, p14, p15, p16, p17, p18, p19];
export_solvers![p20, p21];
