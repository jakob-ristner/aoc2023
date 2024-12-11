mod circuitry;
mod parsing;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::u128;

use num::integer::lcm;

use crate::circuitry::*;
use crate::parsing::*;

#[derive(Debug, Clone, PartialEq, Eq)]
struct BHeapState {
    time: u128,
    signal: Signal,
}

impl Ord for BHeapState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for BHeapState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type CycleCtr = HashMap<String, (Vec<u128>, Vec<u128>)>;
const TRACK: u128 = 1;
fn find_lcm(circ: &Circuit) -> u128 {
    let mut circ = circ.clone();
    let mut ccl_ctr = CycleCtr::new();

    for (_, module) in circ.iter() {
        if module.get_outs().contains(&"rx".into()) {
            let ins = module.get_ins();
            for name in ins {
                ccl_ctr.insert(name.clone(), (vec![], vec![]));
            }
        }
    }

    for i in 1.. {
        let (_, _, new_circ) = press(circ, &mut ccl_ctr, i);
        circ = new_circ;

        if ccl_ctr
            .values()
            .all(|(lo, hi)| lo.len() >= TRACK as usize && hi.len() >= TRACK as usize)
        {
            break;
        }
        if i % 100000 == 0 {
            dbg!(&ccl_ctr);
        }
    }

    ccl_ctr.values().map(|(_, hi)| hi[0]).fold(1, lcm)
}

fn press_many(circ: &Circuit, n: u128) -> u128 {
    let mut circ = circ.clone();
    let mut ccl_ctr = CycleCtr::new();
    let mut lo = 0;
    let mut hi = 0;
    for i in 0..n {
        let (n_lo, n_hi, new_circ) = press(circ, &mut ccl_ctr, i);
        lo += n_lo;
        hi += n_hi;
        circ = new_circ;
    }
    lo * hi
}

fn press(circ: Circuit, ccl_ctr: &mut CycleCtr, press_n: u128) -> (u128, u128, Circuit) {
    use Level::*;
    let mut circ = circ;
    let signal = Signal {
        from: "start".into(),
        to: "broadcaster".into(),
        level: Lo,
    };

    let mut heap: BinaryHeap<BHeapState> = circ
        .get_mut("broadcaster")
        .map(|broad| broad.apply(&signal))
        .expect("broadcaster not found")
        .iter()
        .map(|sig| BHeapState {
            time: 0,
            signal: sig.clone(),
        })
        .collect();

    let mut lo = 1;
    let mut hi = 0;

    while let Some(BHeapState { time, signal }) = heap.pop() {
        if signal.level == Hi {
            hi += 1;
            if let Some((_, hi_ccl)) = ccl_ctr.get_mut(&signal.from) {
                if hi_ccl.len() < TRACK as usize {
                    hi_ccl.push(press_n);
                }
            }
        } else {
            lo += 1;
            if let Some((lo_ccl, _)) = ccl_ctr.get_mut(&signal.from) {
                if lo_ccl.len() < TRACK as usize {
                    lo_ccl.push(press_n);
                }
            }
        }

        if signal.to == "rx" && signal.level == Lo {
            return (lo, hi, circ);
        }

        if let Some(module) = circ.get_mut(&signal.to) {
            let new_signals = module.apply(&signal);
            for new_signal in new_signals {
                heap.push(BHeapState {
                    time: time + 1,
                    signal: new_signal,
                });
            }
        }
    }

    (lo, hi, circ)
}

fn main() {
    let content = include_str!("input.txt");
    let (_, circ) = circuit(content).unwrap();
    let p1 = press_many(&circ, 1000);
    let p2 = find_lcm(&circ);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
