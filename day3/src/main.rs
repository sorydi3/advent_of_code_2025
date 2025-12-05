use utils::read_input;

fn main() {
    let banks = read_input("./day3/input.txt");
    let joltage = banks
        .map(|bank| {
            let bateries = bank.unwrap().chars().collect::<Vec<_>>();
            get_max(&bateries[..])
        })
        .sum::<usize>();

    println!("JOLTAGE: {:?}", joltage);
}

fn get_max<'a>(bateries: &'a [char]) -> usize {
    (0..bateries.len())
        .map(|i| {
            let current = bateries[i];
            bateries
                .iter()
                .skip(i+1)
                .map(move |c| {
                    [current.to_string(), (*c).to_string()]
                        .concat()
                        .parse::<usize>()
                        .unwrap()
                })
                .max()
                .unwrap_or_default()
        })
        .max()
        .unwrap()
}
