extern crate rand;

mod array_fun;
mod closure_fun;
mod core_data_types;
mod dynamic_dispatch_fun;
mod enumfun;
mod functions;
mod generics_fun;
mod high_order_functions;
mod if_stmt;
mod listsfun;
mod loops;
mod match_stmt;
mod option_t;
mod pattern_matcher_fun;
mod slice_fun;
mod strings_fun;
mod struct_fun;
mod traits_fun;
mod tuples_fun;
mod union_data;
mod unsafe_global;
mod vars;
mod vector_fun;

fn main() {
    array_fun::execute();
    closure_fun::execute();
    core_data_types::execute();
    dynamic_dispatch_fun::execute();
    enumfun::execute();
    functions::execute();
    generics_fun::execute();
    high_order_functions::execute();
    if_stmt::execute();
    listsfun::execute();
    loops::execute();
    match_stmt::execute();
    option_t::execute();
    pattern_matcher_fun::execute();
    slice_fun::execute();
    strings_fun::execute();
    struct_fun::print_struct();
    traits_fun::execute();
    tuples_fun::execute();
    union_data::execute();
    unsafe_global::execute();
    vars::execute();
    vector_fun::execute();
}