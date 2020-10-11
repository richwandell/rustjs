let foo = {
    bar: {
        baz: function() {
            return {
                foo: 1
            }
        }
    }
}

foo.bar.baz().foo
