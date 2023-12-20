use anyhow::{anyhow, Result};
use std::collections::{HashMap, VecDeque};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Config(HashMap<String, Module>);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcast(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FlipFlop {
    input_name: String,
    outputs: Vec<String>,
    is_on: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Conjunction {
    input_name: String,
    outputs: Vec<String>,
    input_is_high: HashMap<String, bool>,
}

#[derive(Debug)]
struct SignalIterator<'a> {
    cfg: &'a mut Config,
    signals: VecDeque<(String, bool, String)>,
}

impl<'a> Iterator for SignalIterator<'a> {
    type Item = (String, bool, String);

    /// Yields signals in transit until there are no more signals to process
    fn next(&mut self) -> Option<Self::Item> {
        let rv = self.signals.pop_front()?;
        let (input_source, input_is_high, input_module) = rv.clone();

        // If we can't find the input module we can't process it, but the signal may be interesting
        let Some(m) = self.cfg.0.get_mut(&input_module) else {
            return Some(rv);
        };

        let output_is_high = match m {
            Module::FlipFlop(flip_flop) => {
                if input_is_high {
                    return Some(rv);
                }
                flip_flop.is_on = !flip_flop.is_on;
                flip_flop.is_on
            }
            Module::Conjunction(conjunction) => {
                *conjunction
                    .input_is_high
                    .entry(input_source.clone())
                    .or_default() = input_is_high;
                conjunction.output_is_high()
            }
            Module::Broadcast(_) => input_is_high,
        };

        for output in m.outputs() {
            self.signals
                .push_back((input_module.clone(), output_is_high, output.to_string()));
        }
        Some(rv)
    }
}

impl Config {
    fn iter_signals_from_button_press(&mut self) -> SignalIterator<'_> {
        let signals = [("button".to_string(), false, "broadcaster".to_string())]
            .into_iter()
            .collect();
        SignalIterator { cfg: self, signals }
    }
}

impl Module {
    fn input_name(&self) -> &str {
        match self {
            Module::FlipFlop(m) => &m.input_name,
            Module::Conjunction(m) => &m.input_name,
            Module::Broadcast(_) => "broadcaster",
        }
    }

    fn outputs(&self) -> &[String] {
        match self {
            Module::FlipFlop(m) => &m.outputs,
            Module::Conjunction(m) => &m.outputs,
            Module::Broadcast(outputs) => outputs,
        }
    }
}

impl Conjunction {
    fn output_is_high(&self) -> bool {
        !self.input_is_high.iter().all(|(_, &h)| h)
    }
}

fn parse_input_outputs(s: &str) -> Result<(String, Vec<String>)> {
    let Some((input, outputs_str)) = s.split_once(" -> ") else {
        return Err(anyhow!("Invalid input output specification {:?}", s));
    };
    Ok((
        input.to_owned(),
        outputs_str.split(", ").map(ToOwned::to_owned).collect(),
    ))
}

impl FromStr for Config {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut modules = s
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<Module>, _>>()?;

        // Make a map of module name to inputs
        let mut module_inputs: HashMap<String, Vec<String>> = HashMap::new();
        for m in modules.iter() {
            for output_name in m.outputs() {
                module_inputs
                    .entry(output_name.clone())
                    .or_default()
                    .push(m.input_name().to_string());
            }
        }

        // Update the input list for conjunctions
        for m in modules.iter_mut() {
            let Some(inputs) = module_inputs.remove(m.input_name()) else {
                continue;
            };
            if let Module::Conjunction(c) = m {
                c.input_is_high
                    .extend(inputs.into_iter().map(|n| (n, false)));
            }
        }

        Ok(Self(
            modules
                .into_iter()
                .map(|m| (m.input_name().to_string(), m))
                .collect(),
        ))
    }
}

impl FromStr for Module {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Some(io) = s.strip_prefix('%') {
            let (input_name, outputs) = parse_input_outputs(io)?;
            Ok(Self::FlipFlop(FlipFlop {
                input_name,
                outputs,
                is_on: false,
            }))
        } else if let Some(io) = s.strip_prefix('&') {
            let (input_name, outputs) = parse_input_outputs(io)?;
            Ok(Self::Conjunction(Conjunction {
                input_name,
                outputs,
                input_is_high: HashMap::new(),
            }))
        } else if s.starts_with("broadcaster -> ") {
            let (_, outputs) = parse_input_outputs(s)?;
            Ok(Self::Broadcast(outputs))
        } else {
            Err(anyhow!("Invalid module {:?}", s))
        }
    }
}

fn part_a(cfg: &Config) -> usize {
    let mut state = cfg.clone();
    let mut num_low = 0;
    let mut num_high = 0;

    for _ in 0..1000 {
        for (_, h, _) in state.iter_signals_from_button_press() {
            if h {
                num_high += 1;
            } else {
                num_low += 1;
            }
        }
    }

    num_low * num_high
}

fn part_b(cfg: &Config) -> usize {
    // This solution doesn't fill me with joy. It is very dependent on the particular way the input
    // is laid out. The input basically splits the broadcast signal into 4 "counters" that wrap at
    // different number of button pressed. The RV node basically only switches when all 4 counters
    // wrap during the same button press cycle. The counter wrap outputs are conjunctions with more
    // than 2 inputs, so we look for those, find out when they wrap and use that to calculate when
    // "rv" would eventually trigger. We can just multiply each cycle time together as they are all
    // prime.
    let mut state = cfg.clone();

    // Find counters
    let mut cycle_times: HashMap<String, Option<usize>> = HashMap::new();
    for m in cfg.0.values() {
        if let Module::Conjunction(c) = m {
            if c.outputs.len() > 2 {
                cycle_times.insert(c.input_name.clone(), None);
            }
        }
    }

    for num_presses in 1usize.. {
        for (s, h, _) in state.iter_signals_from_button_press() {
            for (counter_name, cycle_time) in cycle_times.iter_mut() {
                if &s == counter_name && !h {
                    *cycle_time = Some(num_presses);
                }
            }
        }
        if cycle_times.values().all(Option::is_some) {
            return cycle_times.into_values().flatten().product();
        }
    }
    // We can't get here for a very long time
    unreachable!();
}

pub fn main(path: &Path) -> Result<(usize, Option<usize>)> {
    let cfg: Config = std::fs::read_to_string(path)?.parse()?;
    Ok((part_a(&cfg), part_b(&cfg).into()))
}

#[cfg(test)]
mod test {
    use super::*;

    test_real_input!(20, 737_679_780, 227_411_378_431_763);

    const EXAMPLE_INPUT_1: &'static str = concat!(
        "broadcaster -> a, b, c\n",
        "%a -> b\n",
        "%b -> c\n",
        "%c -> inv\n",
        "&inv -> a\n",
    );

    const EXAMPLE_INPUT_2: &'static str = concat!(
        "broadcaster -> a\n",
        "%a -> inv, con\n",
        "&inv -> b\n",
        "%b -> con\n",
        "&con -> output\n",
    );

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(&EXAMPLE_INPUT_1.parse().unwrap()), 32_000_000);
        assert_eq!(part_a(&EXAMPLE_INPUT_2.parse().unwrap()), 11_687_500);
    }
}
