use std::thread;
use crate::relaxation_context::RelaxationContext;

pub fn relax(context: &RelaxationContext) -> (bool, usize) {

    let assignable_rows: Vec<usize> = initialise_row_number_stack(context.array_size - 2);

    for i in 1..context.no_of_threads {
        thread::spawn(|| thread_relax(context, &assignable_rows));
    }

    (true, 5)
}

fn thread_relax(context: &RelaxationContext, assignable_rows: &Vec<usize>) -> (bool, usize) {
    let assigned_rows: Vec<usize> = thread_get_assigned_rows(assignable_rows);

    (true, 5)
}

/*
For 5v5, totalRowsToAssign is 3. Skip the first (0) and last (4) rows as they are boundaries.
Range should be 1,2,3.
 */
fn initialise_row_number_stack(no_of_assignable_rows: usize) -> Vec<usize> {
    let mut assignable_rows: Vec<usize> = Vec::with_capacity(no_of_assignable_rows);
    for i in 1..no_of_assignable_rows+1 {
        assignable_rows.push(i);
    }

    assignable_rows
}

fn thread_get_assigned_rows(assignable_rows: &Vec<usize>) -> Vec<usize> {
    let assigned_rows: Vec<usize> = Vec::new();
    

    assigned_rows
}