
use std::collections::HashMap;

pub type MappedInputs = HashMap<String, Level>;
pub type Modules = Vec<String>;
pub type Circuit = HashMap<String, Module>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    Hi,
    Lo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Module {
    FlipFlop(Modules, bool, Modules),
    Conjunction(MappedInputs, Modules),
    Broadcaster(Modules),
}

pub fn init_conjunctions(circuit: Circuit) -> Circuit {
    let mut new_circuit = circuit.clone();
    for (name, module) in new_circuit.iter_mut() {
        if let Module::Conjunction(inputs, _) = module {
            for (name1, module1) in circuit.iter() {
                let outs = module1.get_outs();
                if outs.contains(name) {
                    inputs.insert(name1.clone(), Level::Lo);
                }
            }
        }
        else if let Module::FlipFlop(inp, _, _) = module {
            for (name1, module1) in circuit.iter() {
                let outs = module1.get_outs();
                if outs.contains(name) {
                    inp.push(name1.clone());
                }
            }
        }
    }
    new_circuit
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Signal {
    pub from: String,
    pub to: String,
    pub level: Level,
}

impl Module {
    pub fn new_flipflop(outputs: Modules) -> Module {
        Module::FlipFlop(vec![], false, outputs)
    }
    pub fn new_conjunction(outputs: Modules) -> Module {
        Module::Conjunction(HashMap::new(), outputs)
    }
    pub fn get_outs(&self) -> Modules {
        use Module::*;
        match self {
            FlipFlop(_, _, outs) => outs.clone(),
            Conjunction(_, outs) => outs.clone(),
            Broadcaster(outs) => outs.clone(),
        }
    }

    pub fn get_ins(&self) -> Modules {
        use Module::*;
        match self {
            FlipFlop(ins, _, _) => ins.clone(),
            Conjunction(ins, _) => ins.keys().cloned().collect(),
            Broadcaster(_) => vec![],
        }
    }

    pub fn apply(&mut self, signal: &Signal) -> Vec<Signal> {
        use Level::*;
        use Module::*;
        match self {
            FlipFlop(_, on, outputs) => {
                let mut out = vec![];
                if signal.level == Lo {
                    let (new_on, sig_level) = if *on { (false, Lo) } else { (true, Hi) };
                    *on = new_on;
                    for output in outputs {
                        out.push(Signal {
                            from: signal.to.clone(),
                            to: output.clone(),
                            level: sig_level,
                        });
                    }
                }
                out
            }
            Conjunction(inputs, outputs) => {
                inputs.insert(signal.from.clone(), signal.level);
                let all_hi = inputs.values().all(|&level| level == Hi);
                let sig_level = if all_hi { Lo } else { Hi };
                let mut out = vec![];
                for output in outputs {
                    out.push(Signal {
                        from: signal.to.clone(),
                        to: output.clone(),
                        level: sig_level,
                    });
                }
                out
            },
            Broadcaster(vec) => {
                let mut out = vec![];
                for output in vec {
                    out.push(Signal {
                        from: signal.to.clone(),
                        to: output.clone(),
                        level: signal.level,
                    });
                }
                out
            },
        }
    }
}
