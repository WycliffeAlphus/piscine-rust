pub fn edit_distance(source: &str, target: &str) -> usize {

    /*
      dp[i][j] = min(
        dp[i-1][j] + 1, //deletion
        dp[i][j-1] + 1, // insertion
        dp[i-1][j-1]+cost // substitution
    )
    
     */
  

  let m = source.len();
  let n = target.len();


  let mut dp = vec![vec![0;n+1];m+1];

  for i in 0..=m {
    dp[i][0] = i
  }

  for j in 0..=n {
    dp[0][j] = j;
  }

  for i in 1..=m {
    for j in 1..=n {
        let cost = if source.as_bytes()[i -1] == target.as_bytes()[j-1] {0} else {1};

        dp[i][j] = std::cmp::min(
            std::cmp::min(dp[i-1][j]+1, dp[i][j-1]+1),
            dp[i-1][j-1]+ cost
        );
    
    }
  }
dp[m][n]
}