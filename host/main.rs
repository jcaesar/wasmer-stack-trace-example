use anyhow::Result;
use wasmer::*;

fn main() -> Result<()> {
    let wasm_bytes = include_bytes!("stackpanic.wasm");
    let store = Store::new(&Universal::new(Cranelift::new()).engine());
    let module = Module::new(&store, wasm_bytes)?;
    let instance = Instance::new(&module, &imports! {})?;
    let res = instance
        .exports
        .get_function("a")?
        .native::<(), ()>()?
        .call();
    match res {
        Ok(()) => (),
        Err(e @ RuntimeError { .. }) => {
            println!("RuntimeError: {}", e.message());
            for frame in e.trace() {
                println!(
                    "\t{}::{}",
                    frame.module_name(),
                    frame
                        .function_name()
                        .map(symbolic_demangle::demangle)
                        .map(Into::into)
                        .unwrap_or(format!("$;{};", frame.func_index()))
                )
            }
        }
    }
    Ok(())
}
