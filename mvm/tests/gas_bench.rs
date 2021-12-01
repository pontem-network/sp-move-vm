mod common;

#[cfg(feature = "bench")]
mod bench {
    use std::fmt::{Display, Formatter};
    use std::time::Instant;
    use move_core_types::vm_status::StatusCode;
    use mvm::io::context::ExecutionContext;
    use mvm::mvm::Mvm;
    use mvm::types::{Gas, ScriptTx};
    use mvm::Vm;
    use crate::common::assets::{empty_loop, math_loop, read_write_loop, store_module, vector_loop};
    use crate::common::mock::{BankMock, EventHandlerMock, StorageMock};
    use crate::common::vm;
    use crate::common::mock::Utils;

    #[test]
    fn gas_bench() {
        let (vm, _, _, _) = vm();
        vm.pub_mod(store_module());

        let mut avg = Avg::default();
        find_gas_per_seconds(5_650_000, 5_000, &vm, &mut avg, empty_loop, "empty_loop");
        find_gas_per_seconds(15_000, 200, &vm, &mut avg, math_loop, "math_loop");
        find_gas_per_seconds(500_000, 5_000, &vm, &mut avg, read_write_loop, "read_write_loop");
        find_gas_per_seconds(1000, 100, &vm, &mut avg, vector_loop, "vector_loop");

        println!("Gas avg: {}", avg);
    }

    fn find_gas_per_seconds<S>(start: u64, step: u64, vm: &Mvm<StorageMock, EventHandlerMock, BankMock>, avg: &mut Avg, script_supplier: S, name: &str) -> u64 where S: Fn(u64) -> ScriptTx {
        let mut iter_count = start;
        let mut last_check = None;

        loop {
            iter_count += step;
            let script = script_supplier(iter_count);

            let gas = Gas::new(18446744073709550, 1).unwrap();
            let context = ExecutionContext::new(100, 100);
            let now = Instant::now();
            let res = vm.execute_script(gas, context, script, false);
            let elapsed = now.elapsed().as_millis();
            if res.status_code != StatusCode::EXECUTED {
                panic!("Transaction failed: {:?}", res);
            }
            let res = Results { time: elapsed, gas: res.gas_used, iter: iter_count };
            if res.time > 1_000 {
                if last_check.is_none() {
                    let back_step = step * 2;
                    if back_step > iter_count {
                        iter_count = 0;
                    } else {
                        iter_count -= back_step;
                    }
                    continue;
                }

                return calc_and_show_stat(vm, avg, last_check.take().unwrap(), res, &script_supplier, name);
            }
            last_check = Some(res);
        }
    }

    fn calc_and_show_stat<S>(vm: &Mvm<StorageMock, EventHandlerMock, BankMock>, avg: &mut Avg, last: Results, res: Results, script_supplier: &S, name: &str) -> u64 where S: Fn(u64) -> ScriptTx {
        println!("Check script {}:", name);
        println!("  Time range: [{}, {}]", last.time, res.time);
        println!("  Gas range: [{}, {}]", last.gas, res.gas);
        println!("  Iter range: [{}, {}]", last.iter, res.iter);

        let iter = (last.iter + res.iter) / 2;
        let mut total_gas = 0;
        let mut total_time = 0;

        for _ in 0..10 {
            let script = script_supplier(iter);
            let gas = Gas::new(18446744073709550, 1).unwrap();
            let context = ExecutionContext::new(100, 100);

            let now = Instant::now();
            let res = vm.execute_script(gas, context, script, false);
            total_time += now.elapsed().as_millis();
            total_gas += res.gas_used;
            avg.add(res.gas_used);
            if res.status_code != StatusCode::EXECUTED {
                panic!("Transaction failed: {:?}", res);
            }
        }
        println!("  Time avg: [{} ms]", total_time / 10);
        println!("  Gas avg: [{}]", total_gas / 10);
        println!("================================================================================");

        total_gas / 10
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

            let sum: f64 = self.vec.iter()
                .map(|val| *val as f64)
                .map(|val| (val - avg).powf(2.0))
                .sum();

            write!(f, "{} ± {}", avg, (sum / avg.powf(2.0)).sqrt())
        }
    }
}