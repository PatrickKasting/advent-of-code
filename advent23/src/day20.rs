use std::collections::VecDeque;

use ahash::AHashMap;
use itertools::Itertools;
use shared::number_theory::least_common_multiple;

type Configuration<'input> = AHashMap<&'input str, (Module<'input>, Vec<&'input str>)>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module<'input> {
    FlipFlop(bool),
    Conjunction(AHashMap<&'input str, Pulse>),
    Broadcaster,
}

type Pulse = bool;
const LOW: Pulse = false;

pub fn first_answer(input: &str) -> String {
    let mut configuration = configuration(input);
    let (total_number_of_low_pulses, total_number_of_high_pulses) =
        total_number_of_low_and_high_pulses(&mut configuration);
    (total_number_of_low_pulses * total_number_of_high_pulses).to_string()
}

/// Returns the fewest number of button presses required to deliver a single low pulse to the module
/// named `rx`.
///
/// # Correctness
///
/// The module `rx` receives pulses from the conjunction `nc`, which is the only source of `rx`.
/// Thus, we need to determine, when `nc` remembers all high pulses from its four sources `fh`,
/// `lk`, `hh`, and `fn`, which are all conjunctions with a single source each. A conjunction with a
/// single source acts as an inverter, so we must find out when the sources of these four modules
/// send low pulses simultaneously. These sources are `gl`, `gk`, `hr`, and `nr`, which are all
/// conjunctions and each can be viewed as the output component of a cluster of components.
///
/// Each cluster contains 12 chained flip-flops, where the first flip-flop receives low pulses from
/// the broadcaster. Because a flip-flop emits a low pulse on every second low pulse received, the
/// chain acts as a 12-bit integer that increases by one on every button press. Some flip-flops in
/// the chain are linked to the output conjunction, meaning that these bits must be `1`
/// simultaneously for the conjunction to send a low pulse.
///
/// The output conjunction sends pulses to some of the flip-flops in the chain. Most of the time,
/// these will be high pulses and because these are ignored by the flip-flops, these pulses don't
/// interfere with the incrementation of the 12-bit integer. However, when the correct bits are `1`,
/// the conjunction sends low pulses, toggling the bits to which it's connected. This effectively
/// increases the 12-bit integer by a number given by the connected bits. Notice that the first
/// flip-flop and the output conjunction are always mutually connected, so after a low pulse from
/// the conjunction, the first flip-flop flips, sending a high pulse to the conjunction, which then
/// goes back to sending high pulses until the relevant bits are `1`.
///
/// Now, consider the cluster to which `gl` belongs: The connections to `gl` yield that
/// `gl` outputs a low pulse when the chain matches the bit pattern `0b1111_xxxx_1x11`. This will
/// happen after `0b1111_0000_1011 == 3851` button presses. Then, the low pulses from `gl` will
/// increase the interger by `0b0000_1111_0101 == 245` causing it to overflow to exactly zero, and
/// then the process will repeat. Thus, `gl` emits low pulses every `3851` button presses.
///
/// Similar analyses for `gk`, `hr`, and `nr` show that they emit low pulses every `4003`, `4027`,
/// and `3847`, respectively. The least common multiple of these four cycle lengths is
/// `238_815_727_638_557`, which is the number of button presses it takes for `gl`, `gk`, `hr`, and
/// `nr` to emit low pulses simultaneously. That is, it's the number of button presses it takes for
/// `rx` to receive a single low pulse.
pub fn second_answer(_input: &str) -> String {
    let cycle_lengths = [
        0b1111_0000_1011,
        0b1111_1010_0011,
        0b1111_1011_1011,
        0b1111_0000_0111,
    ];
    cycle_lengths
        .into_iter()
        .reduce(least_common_multiple)
        .expect("list of cycle lengths should not be empty")
        .to_string()
}

fn total_number_of_low_and_high_pulses(configuration: &mut Configuration) -> (usize, usize) {
    let (number_of_low_pulses, number_of_high_pulses): (Vec<_>, Vec<_>) =
        (0..1000).map(|_| press_button(configuration)).unzip();
    let total_number_of_low_pulses = number_of_low_pulses.into_iter().sum();
    let total_number_of_high_pulses = number_of_high_pulses.into_iter().sum();
    (total_number_of_low_pulses, total_number_of_high_pulses)
}

fn press_button(configuration: &mut Configuration) -> (usize, usize) {
    let (mut number_of_low_pulses, mut number_of_high_pulses) = (0, 0);
    let mut pulses = VecDeque::from([("button", LOW, "broadcaster")]);
    while let Some((source, pulse, destination)) = pulses.pop_front() {
        if pulse {
            number_of_high_pulses += 1;
        } else {
            number_of_low_pulses += 1;
        }

        if let Some((destination_module, output_pulse_destinations)) =
            configuration.get_mut(&destination)
        {
            if let Some(output_pulse) = receive(pulse, destination_module, source) {
                for output_pulse_destination in output_pulse_destinations {
                    pulses.push_back((destination, output_pulse, output_pulse_destination));
                }
            }
        }
    }
    (number_of_low_pulses, number_of_high_pulses)
}

fn receive(pulse: Pulse, to: &mut Module, from: &str) -> Option<Pulse> {
    match to {
        Module::FlipFlop(is_on) => {
            if pulse {
                None
            } else {
                *is_on = !*is_on;
                Some(*is_on)
            }
        }
        Module::Conjunction(most_recent_pulse_from_each_source) => {
            *most_recent_pulse_from_each_source
                .get_mut(from)
                .expect("source should be recognized") = pulse;
            let all_high = most_recent_pulse_from_each_source
                .values()
                .all(|&pulse| pulse);
            Some(!all_high)
        }
        Module::Broadcaster => Some(pulse),
    }
}

fn configuration(str: &str) -> Configuration {
    let mut configuration: Configuration = str.lines().map(module).collect();

    for source in configuration.keys().copied().collect_vec() {
        for destination in configuration[&source].1.clone() {
            if let Some((Module::Conjunction(most_recent_pulse_from_each_source), _)) =
                configuration.get_mut(&destination)
            {
                most_recent_pulse_from_each_source.insert(source, LOW);
            }
        }
    }

    configuration
}

fn module(line: &str) -> (&str, (Module, Vec<&str>)) {
    let (module, destinations) = line
        .split_once(" -> ")
        .expect("every line should contain ' -> '");
    let (name, module) = if module == "broadcaster" {
        ("broadcaster", Module::Broadcaster)
    } else if let Some(name) = module.strip_prefix('%') {
        (name, Module::FlipFlop(LOW))
    } else if let Some(name) = module.strip_prefix('&') {
        (name, Module::Conjunction(AHashMap::new()))
    } else {
        panic!("module should be 'broadcaster', '%', or '&'")
    };
    let destinations = destinations.split(", ").collect_vec();
    (name, (module, destinations))
}

#[cfg(test)]
mod tests {
    use infrastructure::{Input, Puzzle};

    use crate::tests::test_on_input;

    const DAY: usize = 20;

    #[test]
    fn first_examples() {
        test_on_input(DAY, Puzzle::First, Input::Example(0), 32_000_000);
        test_on_input(DAY, Puzzle::First, Input::Example(1), 11_687_500);
    }

    #[test]
    fn first_answer_input() {
        test_on_input(DAY, Puzzle::First, Input::PuzzleInput, 1_020_211_150);
    }

    #[test]
    fn second_answer_input() {
        test_on_input(
            DAY,
            Puzzle::Second,
            Input::PuzzleInput,
            238_815_727_638_557_usize,
        );
    }
}
