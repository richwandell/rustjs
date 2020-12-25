struct WasmCompiler {
    output: Vec<i8>
}

impl WasmCompiler {

    fn module(&mut self) {
        // first write magic
        self.output.push(0x0);
        self.output.push(0x61);
        self.output.push(0x73);
        self.output.push(0x6D);
        // next write version
        self.output.push(0x01);
        self.output.push(0x0);
        self.output.push(0x0);
        self.output.push(0x0);
    }

    fn func(&mut self, params: Vec<i8>, returns: Vec<i8>) {
        let mut len_sig = 2;
        if params.len() > 0 {
            len_sig += 1 + params.len();
        }
        if returns.len() > 0 {
            len_sig += 1 + returns.len();
        }
        // length of function signature
        self.output.push(0x01);
        self.output.push(len_sig as i8);
        // function
        self.output.push(0x01);
        self.output.push(0x60);

        if params.len() > 0 {
            // function params
            self.output.push(params.len() as i8);
            for p in params {
                self.output.push(p);
            }
        }
        if returns.len() > 0 {
            // function returns
            self.output.push(returns.len() as i8);
            for r in returns {
                self.output.push(r);
            }
        }
    }
}