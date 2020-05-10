
use std::fs::read_to_string;
use std::env;

fn load_program(path: &String) -> Result<Vec<i32>, String> {
    // handle the various parsing issues that may come up in one place

    let contents = read_to_string(path).map_err(|err| err.to_string())?;

    // now that we have the contents we need to split on ","
    let mut result = Vec::<i32>::new();
    for num in contents.split(",") {
        result.push(
            num.parse::<i32>().map_err(|err| err.to_string())?
        );
    }

    Ok(result)
}

fn patch(program: &mut Vec<i32>, noun: i32, verb: i32) -> &mut Vec<i32> {
    // patch the values to match our execution state
    // replace position 1 with the value 12 and replace position 2 with the value 2.
    program[1] = noun;
    program[2] = verb;

    program
}

fn perform_add(counter: usize, program: &mut Vec<i32>) {
    let a = program[counter + 1];
    let b = program[counter + 2];
    let store = program[counter + 3];

    program[store as usize] = program[a as usize] + program[b as usize];
}

fn perform_mul(counter: usize, program: &mut Vec<i32>){
    let a = program[counter + 1];
    let b = program[counter + 2];
    let store = program[counter + 3];

    program[store as usize] = program[a as usize] * program[b as usize];
}


fn compute(program: &mut Vec<i32>) -> &mut Vec<i32> {
    // perform all of the computations on a program assuming that the
    // program counter starts at 0

    // Op codes:
    // 1 = add
    // 2 = multiply
    // 99 = exit
    // other = panic or something

    let mut counter = 0;

    while counter != 99 {
        match program[counter] {
            1 => perform_add(counter, program),
            2 => perform_mul(counter, program),
            99 => break,
            _ => panic!()
        }
        counter += 4;
    }

    program
}

// TODO make this output a optional or result since the target might not be able to be found
fn search(target: i32, program: Vec<i32>) -> Option<(i32, i32)>{
    // given the target number and input program search through nouns and verbs until you find the result

    let mut output: Option<(i32, i32)> = None;

    for noun in 0..99 {
        for verb in 0..99 {
            let mut working_copy = program.clone();

            patch(&mut working_copy, noun, verb);
            compute(&mut working_copy);

            if working_copy[0] == target {
                output = Some((noun, verb));
            }
        }
    }
    output
}


fn main() {
    /*
    this program must:
    accept a single command line arg that is the path to a file
    read the input from a file
    split all of the numbers on the commas
    convert the numbers to ints that we can work with
    put all of those ints into a datatype that makes sense for this
    patch the input to match the prompt
    perform the calculation
    return the result
    */
    let target = 19690720;

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please input a single path to the file containing the input data");
        return
    }

    if let Ok(program) = load_program(&args[1]) {
        // do stuff
        let mut working_program = program.clone();

        patch(&mut working_program, 12, 2);
        compute(&mut working_program);

        println!("part one answer: {}", &working_program[0]);

        if let Some((noun, verb)) = search(target, program){
            let computed = 100 * noun + verb;
            println!("part two answer: noun:{}, verb:{}, computed:{}", noun, verb, computed);
        } else {
            println!("No input could be found that matches the target {}", target);
        }
    } else {
        println!("failed to load input");
    }
}


#[cfg(test)]
mod tests {
    // TODO: add tests for the other functions that are defined above

    use super::compute;

    #[test]
    fn test_compute(){
        // 1,0,0,0,99 becomes 2,0,0,0,99 (1 + 1 = 2).
        let mut a: Vec<i32> = vec![1, 0, 0, 0, 99];
        let b: Vec<i32> = vec![2, 0, 0, 0, 99];
        assert_eq!(compute(&mut a), &b);

        // 2,3,0,3,99 becomes 2,3,0,6,99 (3 * 2 = 6).
        let mut a: Vec<i32> = vec![2,3,0,3,99];
        let b: Vec<i32> = vec![2,3,0,6,99];
        assert_eq!(compute(&mut a), &b);

        // 2,4,4,5,99,0 becomes 2,4,4,5,99,9801 (99 * 99 = 9801).
        let mut a: Vec<i32> = vec![2,4,4,5,99,0];
        let b: Vec<i32> = vec![2,4,4,5,99,9801];
        assert_eq!(compute(&mut a), &b);

        // 1,1,1,4,99,5,6,0,99 becomes 30,1,1,4,2,5,6,0,99.
        let mut a: Vec<i32> = vec![1,1,1,4,99,5,6,0,99];
        let b: Vec<i32> = vec![30,1,1,4,2,5,6,0,99];
        assert_eq!(compute(&mut a), &b);
    }
}