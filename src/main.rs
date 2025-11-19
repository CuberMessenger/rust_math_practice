
mod utility;
mod solvers;

fn main() {
    let skip: bool = true;

    if !skip {
        solvers::p1();
        solvers::p2();
        solvers::p3();
        solvers::p4();
        solvers::p5();
        solvers::p6();
        solvers::p7();
        solvers::p8();
        solvers::p9();
        solvers::p10();
    }

    solvers::p11();
}
