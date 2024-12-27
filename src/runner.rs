use crate::solver::Solver;
use std::cmp::max;

const SOLUTION_TITLE: &str = "Solution";
const TIME_TITLE: &str = "Time (ms)";

pub struct MaxLength {
    labels: Vec<usize>,
    solution: usize,
    time: usize,
}

pub struct RowFormat {
    labels: Vec<String>,
    solution: String,
    time: String,
}

pub fn run_solvers<T: Solver, F: Fn(&T) -> bool>(
    label_headers: &[&str],
    solvers: &[T],
    solver_predicate: F,
) -> (Vec<RowFormat>, MaxLength) {
    let mut row_formats = Vec::with_capacity(solvers.len() * 3);
    let mut max_length = MaxLength {
        labels: label_headers
            .iter()
            .map(|label| label.chars().count())
            .collect(),
        solution: SOLUTION_TITLE.chars().count(),
        time: TIME_TITLE.chars().count(),
    };
    let mut is_first_row = true;

    for solver in solvers {
        // Do not run a solver that doesn't pass the provided predicate.
        if !solver_predicate(solver) {
            continue;
        }

        // Put a blank row between each solver for clarity. Do not put a blank row before the
        // first solver.
        if is_first_row {
            is_first_row = false;
        } else {
            row_formats.push(RowFormat {
                labels: vec![String::new(); label_headers.len()],
                solution: String::new(),
                time: String::new(),
            });
        }

        for row in 0..solver.get_row_count() {
            // Get the labels for this row.
            let labels = solver.get_labels(row);
            assert!(
                label_headers.len() == labels.len(),
                "Solvers must provide exactly the same number of labels as the label headers."
            );

            // Update max_lengths.labels if any label is longer than all previous labels in the same
            // column.
            for (label, max_label_length) in labels.iter().zip(&mut max_length.labels) {
                *max_label_length = max(*max_label_length, label.chars().count());
            }

            let result = solver.execute(row);

            // Convert the solution and duration from the solver into strings that can be inserted
            // in the final two columns of the table.
            let solution = result.solution.to_string();
            // Pad time strings with zeroes until they are at least four characters long, then
            // insert a decimal point three characters from the end of the string. This way the
            // number of microseconds is converted to a display of milliseconds with a
            // fractional part.
            let mut time = format!("{:04}", result.duration.as_micros());
            time.insert(time.len() - 3, '.');

            // Update the maximum lengths of the solutions and times if necessary.
            max_length.solution = max(max_length.solution, solution.chars().count());
            max_length.time = max(max_length.time, time.chars().count());

            row_formats.push(RowFormat {
                labels,
                solution,
                time,
            });
        }
    }

    (row_formats, max_length)
}

pub fn print_results_table(
    header_labels: &[&str],
    row_formats: &[RowFormat],
    max_length: &MaxLength,
) {
    print_table_header(header_labels, max_length);
    print_table_data(row_formats, max_length);
    print_table_footer(header_labels, max_length);
}

fn print_table_header(header_labels: &[&str], max_length: &MaxLength) {
    let empty_labels = vec![""; header_labels.len()];
    print_row(&empty_labels, "", "", max_length, "╔═", "═╤═", "═╗", "═");
    print_row(
        header_labels,
        SOLUTION_TITLE,
        TIME_TITLE,
        max_length,
        "║ ",
        " │ ",
        " ║",
        " ",
    );
    print_row(&empty_labels, "", "", max_length, "╟─", "─┼─", "─╢", "─");
}

fn print_table_footer(header_labels: &[&str], max_length: &MaxLength) {
    let empty_labels = vec![""; header_labels.len()];
    print_row(&empty_labels, "", "", max_length, "╚═", "═╧═", "═╝", "═");
}

fn print_table_data(row_formats: &[RowFormat], max_length: &MaxLength) {
    for row_format in row_formats {
        print_row(
            &row_format
                .labels
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            &row_format.solution,
            &row_format.time,
            max_length,
            "║ ",
            " │ ",
            " ║",
            " ",
        );
    }
}

fn print_row(
    labels: &[&str],
    solution: &str,
    time: &str,
    max_length: &MaxLength,
    start: &str,
    separator: &str,
    end: &str,
    padding_char: &str,
) {
    let mut row_string = String::new();
    row_string += start;
    for (label, max_label_length) in labels.iter().zip(&max_length.labels) {
        let label_padding = max_label_length - label.chars().count();
        // Left-pad labels that consist purely of digits, otherwise right-pad the label.
        if label.chars().all(|char| char.is_ascii_digit()) {
            row_string += &padding_char.repeat(label_padding);
            row_string += label;
        } else {
            row_string += label;
            row_string += &padding_char.repeat(label_padding);
        }
        row_string += separator;
    }
    let solution_padding = max_length.solution - solution.chars().count();
    row_string += &padding_char.repeat(solution_padding);
    row_string += solution;
    row_string += separator;
    let time_padding = max_length.time - time.chars().count();
    row_string += &padding_char.repeat(time_padding);
    row_string += time;
    row_string += end;
    println!("{row_string}");
}
