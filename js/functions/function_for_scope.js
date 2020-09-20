let a = 5;

function hi(rows, cols) {
    for(let i = a; i < rows + a; i++) {
        for(let j = a; j < cols + a; j++) {
            console.log(i, j);
        }
    }
}

hi(5, 5);