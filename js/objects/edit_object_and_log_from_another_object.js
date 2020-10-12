let b = {
    a: 1
}

let a = {
    run: function() {
        for (let a = 0; a < 10; a++) {
            b.a = a * 2;
            console.log(b.a);
        }
    }
}

a.run();