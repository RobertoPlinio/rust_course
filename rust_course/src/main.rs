#![allow(unused_variables)]

mod core_data_types;
mod global_variables;
mod operators;
mod scope_and_shadowing;
mod stack_and_heap;

mod combination_lock;
mod for_example;
mod if_statements;
mod match_statement;
mod while_and_loop;

mod arrays;
mod enumerations;
mod generics;
mod option_of_type;
mod slices;
mod structures;
mod tuples;
mod unions_example;

mod hash_map;
mod hash_set;
mod iterators;
mod vectors;

mod closures;
mod higher_functions;
mod methods;
mod strings;

mod drop_trait;
mod dynamic_static_dispatch;
mod into_trait;
mod operator_overloading;
mod trait_parameters;
mod traits;
mod vec_different_objects;

fn main() {
    println!("Hello, world!");

    // Types and Variables
    core_data_types::example_data_types();
    operators::example_operators();
    scope_and_shadowing::scope_and_shadowing();
    global_variables::global_variables();
    stack_and_heap::stack_and_heap();

    // Control flow
    if_statements::if_statements();
    while_and_loop::while_and_loop();
    for_example::for_example();
    match_statement::match_statement();
    combination_lock::combination_lock();

    // Data Structures
    structures::structures();
    enumerations::enumerations();
    unions_example::unions_example();
    option_of_type::option_of_type();
    arrays::arrays();
    slices::slices();
    tuples::tuples();
    generics::generics();

    // Standard Colllections
    vectors::vectors();
    hash_map::hash_map();
    hash_set::hash_set();
    iterators::iterators();

    // Characters and Strings
    strings::strings();
    methods::methods();
    closures::closures();
    higher_functions::higher_functions();

    // Traits
    traits::traits();
    trait_parameters::trait_parameters();
    into_trait::into_trait();
    drop_trait::drop_trait();
    operator_overloading::operator_overloading();
    dynamic_static_dispatch::dynamic_static_dispatch();
    vec_different_objects::vec_different_objects();
}
