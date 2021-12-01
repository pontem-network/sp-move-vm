mod common;

#[cfg(feature = "bench")]
mod bench {
    use crate::common::assets::{
        empty_loop, math_loop, read_write_loop, store_module, vector_loop,
    };
    use crate::common::mock::Utils;
    use crate::common::mock::{BankMock, EventHandlerMock, StorageMock};
    use crate::common::vm;
    use move_core_types::vm_status::StatusCode;
    use mvm::io::context::ExecutionContext;
    use mvm::mvm::Mvm;
    use mvm::types::{Gas, ScriptTx};
    use mvm::Vm;
    use std::fmt::{Display, Formatter};
    use std::time::Instant;

    #[test]
    fn gas_bench() {
        let (vm, _, _, _) = vm();
        vm.pub_mod(store_module());

        let mut avg = Avg::default();
        find_gas_per_seconds(&vm, &mut avg, empty_loop, "empty_loop");
        find_gas_per_seconds(&vm, &mut avg, math_loop, "math_loop");
        find_gas_per_seconds(&vm, &mut avg, read_write_loop, "read_write_loop");
        find_gas_per_seconds(&vm, &mut avg, vector_loop, "vector_loop");

        println!("Gas avg: {}", avg);
    }

    fn find_gas_per_seconds<S>(
        vm: &Mvm<StorageMock, EventHandlerMock, BankMock>,
        avg: &mut Avg,
        script_supplier: S,
        name: &str,
    ) where
        S: Fn(u64) -> ScriptTx,
    {
        for _ in 0..20 {
            let gas = calc_gas_per_seconds(vm, &script_supplier);
            println!("Check script {}: gas: {}", name, gas);
            avg.add(gas);
        }
    }

    fn calc_gas_per_seconds<S>(
        vm: &Mvm<StorageMock, EventHandlerMock, BankMock>,
        script_supplier: &S,
    ) -> u64
    where
        S: Fn(u64) -> ScriptTx,
    {
        let mut total_time = 0;
        let mut total_gas = 0;
        while total_time < 10_000 {
            let script = script_supplier(10_000);
            let gas = Gas::new(18446744073709550, 1).unwrap();
            let context = ExecutionContext::new(100, 100);
            let now = Instant::now();
            let res = vm.execute_script(gas, context, script, false);
            let elapsed = now.elapsed().as_millis();
            if res.status_code != StatusCode::EXECUTED {
                panic!("Transaction failed: {:?}", res);
            }
            total_time += elapsed;
            total_gas += res.gas_used;
        }

        total_gas / (total_time as u64 / 1000)
    }

    #[derive(Debug)]
    struct Results {
        time: u128,
        gas: u64,
        iter: u64,
    }

    #[derive(Default)]
    struct Avg {
        vec: Vec<u64>,
    }

    impl Avg {
        pub fn add(&mut self, val: u64) {
            self.vec.push(val)
        }
    }

    impl Display for Avg {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let avg = (self.vec.iter().sum::<u64>() as f64) / self.vec.len() as f64;

            let sum: f64 = self
                .vec
                .iter()
                .map(|val| *val as f64)
                .map(|val| (val - avg).powf(2.0))
                .sum();

            write!(f, "{} ± {}", avg, (sum / avg.powf(2.0)).sqrt())
        }
    }
}
