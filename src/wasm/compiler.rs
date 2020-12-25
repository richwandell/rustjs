pub(crate) struct WasmCompiler {
    pub(crate) output: Vec<u8>,
    func_type_sec: Vec<Vec<u8>>,
    import_sec: Vec<Vec<u8>>,
    type_idx: Vec<Vec<u8>>,
    table_sec: Vec<Vec<u8>>,
    mem_sec: Vec<Vec<u8>>,
    global_sec: Vec<Vec<u8>>,
    export_sec: Vec<Vec<u8>>,
    start_sec: Vec<Vec<u8>>,
    elem_sec: Vec<Vec<u8>>,
    code_sec: Vec<Vec<u8>>,
    data_sec: Vec<Vec<u8>>
}

impl WasmCompiler {

    pub(crate) fn new() -> WasmCompiler {
        WasmCompiler{
            output: vec![],
            func_type_sec: vec![],
            import_sec: vec![],
            type_idx: vec![],
            table_sec: vec![],
            mem_sec: vec![],
            global_sec: vec![],
            export_sec: vec![],
            start_sec: vec![],
            elem_sec: vec![],
            code_sec: vec![],
            data_sec: vec![]
        }
    }

    pub(crate) fn get_bytes(&mut self) -> Vec<u8> {
        self.write_module();
        self.write_func_type_sec();
        self.write_func_sec();
        self.write_export_sec();
        self.write_code_sec();
        return self.output.clone();
    }

    fn write_module(&mut self) {
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

    fn write_func_type_sec(&mut self) {
        let mut func_type_len = 1;
        for ft in &self.func_type_sec {
            func_type_len += ft.len();
        }
        self.output.push(0x01);
        self.output.push(func_type_len as u8);
        self.output.push(self.func_type_sec.len() as u8);

        for ft in self.func_type_sec.iter_mut() {
            self.output.append(ft);
        }
    }

    fn write_func_sec(&mut self) {
        self.output.push(0x03);
        let mut n_elements = self.func_type_sec.len();
        self.output.push((n_elements + 1) as u8);
        self.output.push(n_elements as u8);
        for i in 0..n_elements {
            self.output.push(i as u8);
        }
    }

    /**
    07 = start of export section
    07 = 7 bytes
    01 = 1 export
    03 = name is 3 bytes (add)
    61 = a
    64 = d
    64 = d
    00 = ?
    00 = ?
     the 00 directly after the name marks it as a function export, the 00 after that is the index of the exported function
    */
    fn write_export_sec(&mut self) {

    }

    fn write_code_sec(&mut self) {
        let mut num_total_bytes = 0;
        for cs in &self.code_sec {
            num_total_bytes += cs.len();
        }
        let num_code_secs = self.code_sec.len() as u8;
        self.output.push(num_total_bytes as u8);
        self.output.push(num_code_secs as u8);
        for cs in self.code_sec.iter_mut() {
            self.output.append(cs);
        }
    }

    pub(crate) fn add_func(&mut self, params: Vec<u8>, returns: Vec<u8>, code: Vec<u8>, export: bool, name: &str) {
        let mut func = vec![];

        // function
        func.push(0x60);

        if params.len() > 0 {
            // function params
            func.push(params.len() as u8);
            for p in params {
                func.push(p);
            }
        } else {
            func.push(0);
        }
        if returns.len() > 0 {
            // function returns
            func.push(returns.len() as u8);
            for r in returns {
                func.push(r);
            }
        } else {
            func.push(0);
        }
        self.func_type_sec.push(func);
        self.code_sec.push(code);

        if export {
            let func_id = self.func_type_sec.len() as u8;
            let mut name_bytes = String::from(name).into_bytes();
            let mut ex = vec![name_bytes.len() as u8];
            ex.append(&mut name_bytes);
            ex.push(0x00 as u8);
            ex.push(func_id);
            self.export_sec.push(ex);
        }
    }
}