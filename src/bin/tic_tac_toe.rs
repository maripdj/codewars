fn is_solved(board: &[&[u8; 3]; 3]) -> i8 {
    let mut w1 = false;
    let mut w2 = false;

    // Check rows
    for r in 0..3 {
        if board[r][0] == 1 && board[r][1] == 1 && board[r][2] == 1 {
            w1 = true;
        }
        if board[r][0] == 2 && board[r][1] == 2 && board[r][2] == 2 {
            w2 = true;
        }
    }

    // Check columns
    for c in 0..3 {
        if board[0][c] == 1 && board[1][c] == 1 && board[2][c] == 1 {
            w1 = true;
        }
        if board[0][c] == 2 && board[1][c] == 2 && board[2][c] == 2 {
            w2 = true;
        }
    }

    // Check diagonals
    if board[0][0] == 1 && board[1][1] == 1 && board[2][2] == 1 {
        w1 = true;
    }
    if board[0][0] == 2 && board[1][1] == 2 && board[2][2] == 2 {
        w2 = true;
    }
    if board[0][2] == 1 && board[1][1] == 1 && board[2][0] == 1 {
        w1 = true;
    }
    if board[0][2] == 2 && board[1][1] == 2 && board[2][0] == 2 {
        w2 = true;
    }

    if w1 && !w2 {
        1i8
    } else if w2 && !w1 {
        2i8
    } else {
        let mut has_empty = false;
        for r in 0..3 {
            for c in 0..3 {
                if board[r][c] == 0 {
                    has_empty = true;
                    break;
                }
            }
            if has_empty {
                break;
            }
        }
        if has_empty { -1i8 } else { 0i8 }
    }
}

#[cfg(test)]
mod tests {
    use super::is_solved;

    fn dotest(board: &[&[u8; 3]; 3], expected: i8) {
        let actual = is_solved(board);
        assert!(
            actual == expected,
            "With board = {board:?}\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn fixed_tests() {
        for (board, expected) in [
            ([&[0, 0, 1], &[0, 1, 2], &[2, 1, 0]], -1),
            ([&[1, 1, 1], &[0, 2, 2], &[0, 0, 0]], 1),
            ([&[2, 1, 2], &[2, 1, 1], &[1, 1, 2]], 1),
            ([&[2, 1, 2], &[2, 1, 1], &[1, 2, 1]], 0),
        ] {
            dotest(&board, expected);
        }
    }
}

fn main() {}
