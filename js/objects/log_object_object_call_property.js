let foo = {
    bar: {
        baz: function() {
            return {
                foo: 1
            }
        }
    }
}

console.log(foo.bar.baz().foo)
