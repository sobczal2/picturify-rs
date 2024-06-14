pub fn rotate_right_2d<T: Copy + Clone>(input: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if input.is_empty() {
        return vec![];
    }

    let width = input[0].len();
    let height = input.len();

    let mut output = vec![vec![input[0][0]; height]; width];

    output.iter_mut().enumerate().for_each(|(i, row)| {
        row.iter_mut().enumerate().for_each(|(j, elem)| {
            *elem = input[height - j - 1][i];
        });
    });

    output
}

pub fn rotate_left_2d<T: Copy + Clone>(input: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if input.is_empty() {
        return vec![];
    }

    let width = input[0].len();
    let height = input.len();

    let mut output = vec![vec![input[0][0]; height]; width];

    output.iter_mut().enumerate().for_each(|(i, row)| {
        row.iter_mut().enumerate().for_each(|(j, elem)| {
            *elem = input[height - j - 1][i];
        });
    });

    output
}
