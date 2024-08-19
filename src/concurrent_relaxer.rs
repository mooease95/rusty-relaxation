use std::sync::{Arc, Barrier};
use threadpool::ThreadPool;
use crate::relaxation_context::RelaxationContext;

pub fn relax(context: &RelaxationContext) -> (bool, isize) {

    let mut assignable_rows: Vec<usize> = initialise_row_number_stack(context.array_size - 2);
    let rows_per_thread: usize = context.array_size / context.no_of_threads;
    let mut remainders: usize = assignable_rows.len() % context.no_of_threads;
    let thread_pool = ThreadPool::new(context.no_of_threads);


    // TODO: This doesn't seem to work. Check why. Google the compiler's error.
    // thread_pool.execute(|| thread_relax(context, assign_rows_to_thread(assignable_rows, &context.no_of_threads, rows_per_thread, remainders).0));
    let barrier: Arc<Barrier> = Arc::new(Barrier::new(context.no_of_threads + 1));
    barrier.wait();
    (true, 5)
}

fn thread_relax(context: &RelaxationContext, assigned_row: Vec<usize>) {

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

// This function is executed by the main thread.
fn assign_rows_to_thread(mut assignable_rows: Vec<usize>, no_of_threads: &usize, rows_per_thread: usize, mut remaining: usize) -> (Vec<usize>, Vec<usize>, usize) {
    let mut assigned_rows: Vec<usize> = Vec::with_capacity(rows_per_thread + 1);

    for _ in 1..rows_per_thread+1 {
        assigned_rows.push(assignable_rows.pop().unwrap());
    }

    if remaining > 0 {
        assigned_rows.push(assignable_rows.pop().unwrap());
        remaining -= 1;
    }

    (assignable_rows, assigned_rows, remaining)
}


// ============================================================================================= //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assign_rows_to_thread_no_remainder() {

        // Create test data
        let no_of_assignable_rows: usize = 4;
        let no_of_threads: usize = 2;
        let rows_per_thread = no_of_assignable_rows / no_of_threads;

        let mut assignable_rows: Vec<usize> = Vec::with_capacity(no_of_assignable_rows);
        for i in 1..no_of_assignable_rows+1 {
            assignable_rows.push(i);
        }

        let mut remaining: usize = assignable_rows.len() % no_of_threads;
        let (assignable_rows, assigned_rows, remaining): (Vec<usize>, Vec<usize>, usize) = assign_rows_to_thread(assignable_rows, &no_of_threads, rows_per_thread, remaining);

        assert_eq!(assigned_rows.len(), 2);
    }

    #[test]
    fn test_assign_rows_to_thread_with_remainder() {

        // Create test data
        let no_of_assignable_rows: usize = 19;
        let no_of_threads: usize = 5;
        let rows_per_thread = no_of_assignable_rows / no_of_threads;

        let mut assignable_rows: Vec<usize> = Vec::with_capacity(no_of_assignable_rows);
        for i in 1..no_of_assignable_rows+1 {
            assignable_rows.push(i);
        }

        let mut remaining: usize = assignable_rows.len() % no_of_threads;
        // 19 / 5 = 3, with 4 remaining, which will be distributed. So should be 4 rows for first 4 threads, then 3 for the final thread.
        let (assignable_rows, assigned_rows, remaining): (Vec<usize>, Vec<usize>, usize) = assign_rows_to_thread(assignable_rows, &no_of_threads, rows_per_thread, remaining);
        assert_eq!(assigned_rows.len(), 4);
        let (assignable_rows, assigned_rows, remaining): (Vec<usize>, Vec<usize>, usize) = assign_rows_to_thread(assignable_rows, &no_of_threads, rows_per_thread, remaining);
        assert_eq!(assigned_rows.len(), 4);
        let (assignable_rows, assigned_rows, remaining): (Vec<usize>, Vec<usize>, usize) = assign_rows_to_thread(assignable_rows, &no_of_threads, rows_per_thread, remaining);
        assert_eq!(assigned_rows.len(), 4);
        let (assignable_rows, assigned_rows, remaining): (Vec<usize>, Vec<usize>, usize) = assign_rows_to_thread(assignable_rows, &no_of_threads, rows_per_thread, remaining);
        assert_eq!(assigned_rows.len(), 4);println!("assigned_rows.len()={}, remaining={remaining}", assigned_rows.len());
        let (assignable_rows, assigned_rows, remaining): (Vec<usize>, Vec<usize>, usize) = assign_rows_to_thread(assignable_rows, &no_of_threads, rows_per_thread, remaining);
        assert_eq!(assigned_rows.len(), 3);
    }
}