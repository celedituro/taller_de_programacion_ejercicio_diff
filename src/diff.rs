use std::cmp;

/// Returns a ```Vec<i32>``` with the lenghts of the subsecuencies.
///
/// Receives two sequences of lines from two files (from which we want
/// to determine which lines need to be removed from or added to each
/// other to be equal) and find the largest subsequence of lines that
/// appear in both files.

pub fn diff(x: &[String], y: &[String], m: usize, n: usize) -> Vec<i32> {
    let mut grid = vec![0; (m + 1) * (n + 1)];

    for i in 0..m {
        for j in 0..n {
            if x[i] == y[j] {
                grid[(j + 1) * (m + 1) + (i + 1)] = grid[j * (m + 1) + i] + 1;
            } else {
                grid[(j + 1) * (m + 1) + (i + 1)] =
                    cmp::max(grid[j * (m + 1) + (i + 1)], grid[(j + 1) * (m + 1) + i]);
            }
        }
    }

    grid
}

/// Prints the sequences to change from the first file
/// so that the two files are the same.
///
/// # Example
///
/// File1:
/// ```
/// # Generated by NetworkManager
/// search midominio
/// nameserver 192.168.8.1
/// nameserver 8.8.8.8
/// ```
///
/// File2:
/// ```
/// # Generated by NetworkManager
/// search midominio
/// nameserver 192.168.8.1
/// nameserver 4.4.4.4
/// ```
///
/// Output:
/// ```
/// # Generated by NetworkManager
/// search midominio
/// nameserver 192.168.8.1
/// < nameserver 8.8.8.8
/// > nameserver 4.4.4.4
/// ```
///
/// Means that the first three lines of the files are the same.
/// If you modify the fourth line of file 1 ("nameserver 8.8.8.8")
/// and change it to the fourth line of file 2 ("nameserver 4.4.4.4"),
/// the files are the same.

pub fn print_diff(c: Vec<i32>, x: &[String], y: &[String], i: usize, j: usize) {
    if (i > 0) && (j > 0) && (x[i - 1] == y[j - 1]) {
        print_diff(c, x, y, i - 1, j - 1);
        println!(" {}", x[i - 1]);
    } else if (j > 0)
        && (i == 0 || c[(j - 1) * (x.len() + 1) + i] >= c[j * (x.len() + 1) + (i - 1)])
    {
        print_diff(c, x, y, i, j - 1);
        println!("> {}", y[j - 1]);
    } else if (i > 0) && (j == 0 || c[(j - 1) * (x.len() + 1) + i] < c[j * (x.len() + 1) + (i - 1)])
    {
        print_diff(c, x, y, i - 1, j);
        println!("< {}", x[i - 1]);
    } else {
        println!();
    }
}