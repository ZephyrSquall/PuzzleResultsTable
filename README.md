# PuzzleResultsTable

A library to help me with displaying the results of my solvers for coding puzzles like Advent of
Code. It runs all solvers provided to it and displays their solution and execution time.

The create_results_table function needs a slice of strings to use as the table headers that uniquely
identify each puzzle, a slice of instances that implement the Solver trait which produce solutions
for the table, and a predicate that determines whether each Solver instance should be included in
the table. It executes every solver matching the predicate, constructs the table with all the
provided labels and solutions, and prints it to the console.

All entries in the table except the final two rows ("Solution" and "Time") are provided by the
caller based on whatever makes sense for them. They are called labels as they are intended to help
identify puzzles (for example, Advent of Code has "Year", "Day", "Title", and "Part" as label
headers, since "Year", "Day", and "Part" are needed to uniquely identify a puzzle and "Title" makes
it easier to identify the puzzle).

Each Solver needs to implement a get_row_count method which returns how many rows that Solver needs
in the results table (e.g. for Advent of Code, this is two rows for most Solvers as most puzzles
have a part 1 and a part 2, though the 25th puzzle of each year only has one part and therefore one
row), a get_labels method which returns each column's label for the indicated row, and a execute
method which executes the Solver and returns the solution and time taken for the indicated row. The
solution should be anything that implements Display and can be turned into a string, but function
pointers cannot use generics, so instead a Solution enum containing a variety of common return types
for a Solver is provided which allows Solvers to return a common type.