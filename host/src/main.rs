use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::preview2::{command, Table, WasiCtx, WasiCtxBuilder, WasiView};

wasmtime::component::bindgen!({
    inline: "
        package component:guest;

        world example {
            export example: interface {
                hello-world: func() -> string;
            }
        }
    ",
    async: true
});

#[tokio::main]
async fn main() {
    let mut config = Config::new();
    config.async_support(true).wasm_component_model(true);
    let engine = Engine::new(&config).unwrap();

    let table = Table::new();
    let wasi = WasiCtxBuilder::new().build();
    let mut store = Store::new(&engine, Ctx { table, wasi });
    let component = Component::from_file(&engine, "./target/wasm32-wasi/debug/guest.wasm").unwrap();
    let mut linker = Linker::new(&engine);
    command::add_to_linker(&mut linker).unwrap();

    let (example, _instance) = Example::instantiate_async(&mut store, &component, &linker)
        .await
        .unwrap();
    let msg = example
        .example()
        .call_hello_world(&mut store)
        .await
        .unwrap();
    println!("{msg}");
}

pub struct Ctx {
    table: Table,
    wasi: WasiCtx,
}

impl WasiView for Ctx {
    fn table(&self) -> &Table {
        &self.table
    }

    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }

    fn ctx(&self) -> &WasiCtx {
        &self.wasi
    }

    fn ctx_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}
