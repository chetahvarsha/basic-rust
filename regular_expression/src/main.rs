fn is_match(s: &str, p: &str) -> bool {
    let s = s.chars().collect::<Vec<char>>();
    let p = p.chars().collect::<Vec<char>>();
    let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];

    dp[0][0] = true;
    for j in 1..=p.len() {
        if p[j - 1] == '*' {
            dp[0][j] = dp[0][j - 2];
        }
    }

    for i in 1..=s.len() {
        for j in 1..=p.len() {
            if p[j - 1] == s[i - 1] || p[j - 1] == '.' {
                dp[i][j] = dp[i][j - 2] || (dp[i - 1][j] && (p[j - 2] == s[i - 1] || p[j - 2] == '.'));
            }
        }
    }

    dp[s.len()][p.len()]
}

fn main() {
    println!("{}", is_match("aab", "c*a*b"));  // prints: true
    println!("{}", is_match("mississippi", "mis*is*p*."));  // prints: false
}