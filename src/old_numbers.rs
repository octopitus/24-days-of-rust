impl Solution {
  pub fn odd_cells(n: i32, m: i32, indices: Vec<Vec<i32>>) -> i32 {
    let mut matrix = vec![vec![0; m as usize]; n as usize];

    for indice in indices {
      for i in 0..m {
        matrix[indice[0] as usize][i as usize] += 1;
      }

      for j in 0..n {
        matrix[j as usize][indice[1] as usize] += 1;
      }
    }

    let mut odd_numbers = 0;

    for row in matrix {
      for cell in row {
        if cell % 2 != 0 {
          odd_numbers += 1;
        }
      }
    }

    odd_numbers
  }
}
