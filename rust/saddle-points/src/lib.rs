pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut largestRowVal = 0;
    let mut largestIndex = (0, 0);

    let length = input[0].len();
    if length == 0 {
        return Vec::new();
    }
    let mut saddlePoints = Vec::new();
    
    for (r, row) in input.iter().enumerate() {
        // Find largest item of the current row
        for (i, item) in row.iter().enumerate() {
            if item > &largestRowVal {
                largestIndex = (r, i);
                largestRowVal = *item;
            }
        }

        let mut smallest = true;
        // Check the column if the item is smallest in column
        for j in 0..input.len() {
            if input[j][largestIndex.1] < largestRowVal {
                smallest = false
            }
        }
        
        if smallest {
            saddlePoints.push(largestIndex);
        }

        largestIndex = (0,0);
        largestRowVal = 0;

    }

    saddlePoints
}
