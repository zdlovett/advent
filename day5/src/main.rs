
use std::env;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::read_to_string;


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


fn get_3_params(counter: usize, program: &Vec<i32>) -> (i32, i32, i32) {
    let mut mode = program[counter]; // read the current instruction

    // we need to sort out the modes for the instructions
    let mut a = program[counter + 1];
    let mut b = program[counter + 2];
    let mut c = program[counter + 3];

    if mode / 10000 == 0 {
        c = program[c as usize];
    } else {
        mode -= 10000;
    }
    if mode / 1000 == 0 { // immediate mode
        b = program[b as usize];
    } else {
        mode -= 1000;
    }
    if mode / 100 == 0 {
        a = program[a as usize];
    } // no further checks of mode after this so we can leave it as is

    (a, b, c)
}

fn get_2_params(counter: usize, program: &Vec<i32>) -> (i32, i32) {
    let mut mode = program[counter]; // read the current instruction

    // we need to sort out the modes for the instructions
    let mut a = program[counter + 1];
    let mut b = program[counter + 2];

    if mode / 1000 == 0 { // immediate mode
        b = program[b as usize];
    } else {
        mode -= 1000;
    }
    if mode / 100 == 0 {
        a = program[a as usize];
    } // no further checks of mode after this so we can leave it as is

    (a, b)
}


fn perform_add(counter: usize, program: &mut Vec<i32>) -> usize {
    let (a, b) = get_2_params(counter, program);
    let store = program[counter + 3];

    program[store as usize] = a + b;

    counter + 4
}


fn perform_mul(counter: usize, program: &mut Vec<i32>) -> usize {
    let (a, b) = get_2_params(counter, program);
    let store = program[counter + 3];

    program[store as usize] = a * b;

    counter + 4
}


fn jump_if_true(counter: usize, program: &mut Vec<i32>) -> usize{
    let (a, b) = get_2_params(counter, program);

    if a != 0 {
        b as usize
    } else {
        counter + 3
    }
}

fn jump_if_false(counter: usize, program: &mut Vec<i32>) -> usize{
    let (a, b) = get_2_params(counter, program);

    if a == 0 {
        b as usize
    } else {
        counter + 3
    }
}

fn less_than(counter: usize, program: &mut Vec<i32>) -> usize {
    let (a, b) = get_2_params(counter, program);
    let store = program[counter + 3] as usize;

    program[store] = (a < b) as i32;

    counter + 4
}

fn equals(counter: usize, program: &mut Vec<i32>) -> usize {
    let (a, b) = get_2_params(counter, program);
    let store = program[counter + 3] as usize;

    program[store] = (a == b) as i32;

    counter + 4
}

fn perform_input<R: io::Read>(counter: usize, program: &mut Vec<i32>, input: &mut R) -> usize {

    let mut buffer = String::new();
    let mut reader = BufReader::new(input);

    reader.read_line(&mut buffer).unwrap();

    let i = program[counter + 1] as usize;
    program[i] = buffer.trim().parse().unwrap();

    let counter = counter + 2;
    counter
}


fn perform_output<W: io::Write>(counter: usize, program: &mut Vec<i32>, output: &mut W) -> usize {
    /*
    Opcode 4 outputs the value of its only parameter. For example, the instruction 4,50 would output the value at address 50.
    */
    let i = program[counter + 1] as usize;
    write!(output, "{}\n", program[i]).unwrap();

    let counter = counter + 2;
    counter
}


fn compute<'a, R: io::Read, W: io::Write>(program: &'a mut Vec<i32>, input: &mut R, output: &mut W) -> &'a mut Vec<i32> {
    // perform all of the computations on a program assuming that the
    // program counter starts at 0

    // Op codes:
    // 1 = add
    // 2 = multiply
    // 3 = read input from stdin
    // 4 = output to stdout
    // 5 = jump if true
    // 6 = jump if false
    // 99 = exit
    // other = panic or something

    let mut counter = 0;

    while counter != 99 {
        // TODO handle immediate mode params
        counter = match program[counter] % 100 {
            1 => perform_add(counter, program),
            2 => perform_mul(counter, program),
            3 => perform_input(counter, program, input),
            4 => perform_output(counter, program, output),
            5 => jump_if_true(counter, program),
            6 => jump_if_false(counter, program),
            7 => less_than(counter, program),
            8 => equals(counter, program),
            99 => break,
            _ => panic!()
        }
    }

    program
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
    let mut input = io::stdin();
    let mut output = io::stdout();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please input a single path to the file containing the input data");
        return
    }

    if let Ok(program) = load_program(&args[1]) {
        // do stuff
        let mut working_program = program.clone();

        compute(&mut working_program, &mut input, &mut output);

    } else {
        println!("failed to load input");
    }
}


#[cfg(test)]
mod tests {
    // TODO: add tests for the other functions that are defined above

    use std::io;
    use super::compute;

    #[test]
    fn test_compute(){
        let mut input = io::stdin();
        let mut output = io::stdout();

        // 1,0,0,0,99 becomes 2,0,0,0,99 (1 + 1 = 2).
        let mut a: Vec<i32> = vec![1, 0, 0, 0, 99];
        let b: Vec<i32> = vec![2, 0, 0, 0, 99];
        assert_eq!(compute(&mut a, &mut input, &mut output), &b);

        // 2,3,0,3,99 becomes 2,3,0,6,99 (3 * 2 = 6).
        let mut a: Vec<i32> = vec![2,3,0,3,99];
        let b: Vec<i32> = vec![2,3,0,6,99];
        assert_eq!(compute(&mut a, &mut input, &mut output), &b);

        // 2,4,4,5,99,0 becomes 2,4,4,5,99,9801 (99 * 99 = 9801).
        let mut a: Vec<i32> = vec![2,4,4,5,99,0];
        let b: Vec<i32> = vec![2,4,4,5,99,9801];
        assert_eq!(compute(&mut a, &mut input, &mut output), &b);

        // 1,1,1,4,99,5,6,0,99 becomes 30,1,1,4,2,5,6,0,99.
        let mut a: Vec<i32> = vec![1,1,1,4,99,5,6,0,99];
        let b: Vec<i32> = vec![30,1,1,4,2,5,6,0,99];
        assert_eq!(compute(&mut a, &mut input, &mut output), &b);
    }

    #[test]
    fn test_io(){
        let input = String::from("123\n");
        let mut output = Vec::<u8>::new();

        compute(&mut vec![3, 0, 4, 0, 99], &mut input.as_bytes(), &mut output);
        let input: i32 = input.trim().parse().unwrap();
        let output: i32 = String::from_utf8(output).unwrap().trim().parse().unwrap();
        assert_eq!(input, output)
    }

    #[test]
    fn test_modes() {
        let mut input = io::stdin();
        let mut output = io::stdout();

        let mut a: Vec<i32> = vec![1101,100,-1,4,0];
        let b: Vec<i32> = vec![1101,100,-1,4,99];
        assert_eq!(compute(&mut a, &mut input, &mut output), &b);

        let mut a: Vec<i32> = vec![101,100,5,4,0,-1];
        let b: Vec<i32> = vec![101,100,5,4,99,-1];
        assert_eq!(compute(&mut a, &mut input, &mut output), &b);

        let mut a: Vec<i32> = vec![1001,5,-1,4,0,100];
        let b: Vec<i32> = vec![1001,5,-1,4,99,100];
        assert_eq!(compute(&mut a, &mut input, &mut output), &b);

        let mut a: Vec<i32> = vec![1002,5,11,4,0,9];
        let b: Vec<i32> = vec![1002,5,11,4,99,9];
        assert_eq!(compute(&mut a, &mut input, &mut output), &b);
    }

    #[test]
    fn test_equal_to(){
        let input = String::from("8\n");
        let mut output = Vec::<u8>::new();
        compute(&mut vec![3,9,8,9,10,9,4,9,99,-1,8], &mut input.as_bytes(), &mut output);
        let output: i32 = String::from_utf8(output).unwrap().trim().parse().unwrap();
        assert_eq!(output, 1);

        let input = String::from("1\n");
        let mut output = Vec::<u8>::new();
        compute(&mut vec![3,9,8,9,10,9,4,9,99,-1,8], &mut input.as_bytes(), &mut output);
        let output: i32 = String::from_utf8(output).unwrap().trim().parse().unwrap();
        assert_eq!(output, 0);


        let input = String::from("8\n");
        let mut output = Vec::<u8>::new();
        compute(&mut vec![3,3,1108,-1,8,3,4,3,99], &mut input.as_bytes(), &mut output);
        let output: i32 = String::from_utf8(output).unwrap().trim().parse().unwrap();
        assert_eq!(output, 1);

        let input = String::from("1\n");
        let mut output = Vec::<u8>::new();
        compute(&mut vec![3,3,1108,-1,8,3,4,3,99], &mut input.as_bytes(), &mut output);
        let output: i32 = String::from_utf8(output).unwrap().trim().parse().unwrap();
        assert_eq!(output, 0);
    }

    #[test]
    fn test_less_than(){
        let input = String::from("10\n");
        let mut output = Vec::<u8>::new();
        compute(&mut vec![3,9,7,9,10,9,4,9,99,-1,8], &mut input.as_bytes(), &mut output);
        let output: i32 = String::from_utf8(output).unwrap().trim().parse().unwrap();
        assert_eq!(output, 0);

        let input = String::from("8\n");
        let mut output = Vec::<u8>::new();
        compute(&mut vec![3,9,7,9,10,9,4,9,99,-1,8], &mut input.as_bytes(), &mut output);
        let output: i32 = String::from_utf8(output).unwrap().trim().parse().unwrap();
        assert_eq!(output, 0);

        let input = String::from("1\n");
        let mut output = Vec::<u8>::new();
        compute(&mut vec![3,9,7,9,10,9,4,9,99,-1,8], &mut input.as_bytes(), &mut output);
        let output: i32 = String::from_utf8(output).unwrap().trim().parse().unwrap();
        assert_eq!(output, 1);

        let input = String::from("10\n");
        let mut output = Vec::<u8>::new();
        compute(&mut vec![3,3,1107,-1,8,3,4,3,99], &mut input.as_bytes(), &mut output);
        let output: i32 = String::from_utf8(output).unwrap().trim().parse().unwrap();
        assert_eq!(output, 0);

        let input = String::from("8\n");
        let mut output = Vec::<u8>::new();
        compute(&mut vec![3,3,1107,-1,8,3,4,3,99], &mut input.as_bytes(), &mut output);
        let output: i32 = String::from_utf8(output).unwrap().trim().parse().unwrap();
        assert_eq!(output, 0);

        let input = String::from("1\n");
        let mut output = Vec::<u8>::new();
        compute(&mut vec![3,3,1107,-1,8,3,4,3,99], &mut input.as_bytes(), &mut output);
        let output: i32 = String::from_utf8(output).unwrap().trim().parse().unwrap();
        assert_eq!(output, 1);
    }

    #[test]
    fn test_jmp(){
        let input = String::from("0\n");
        let mut output = Vec::<u8>::new();
        compute(&mut vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &mut input.as_bytes(), &mut output);
        let output: i32 = String::from_utf8(output).unwrap().trim().parse().unwrap();
        assert_eq!(output, 0);

        let input = String::from("-1\n");
        let mut output = Vec::<u8>::new();
        compute(&mut vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &mut input.as_bytes(), &mut output);
        let output: i32 = String::from_utf8(output).unwrap().trim().parse().unwrap();
        assert_eq!(output, 1);

        let input = String::from("10\n");
        let mut output = Vec::<u8>::new();
        compute(&mut vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9], &mut input.as_bytes(), &mut output);
        let output: i32 = String::from_utf8(output).unwrap().trim().parse().unwrap();
        assert_eq!(output, 1);


        let input = String::from("0\n");
        let mut output = Vec::<u8>::new();
        compute(&mut vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &mut input.as_bytes(), &mut output);
        let output: i32 = String::from_utf8(output).unwrap().trim().parse().unwrap();
        assert_eq!(output, 0);

        let input = String::from("-1\n");
        let mut output = Vec::<u8>::new();
        compute(&mut vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &mut input.as_bytes(), &mut output);
        let output: i32 = String::from_utf8(output).unwrap().trim().parse().unwrap();
        assert_eq!(output, 1);

        let input = String::from("10\n");
        let mut output = Vec::<u8>::new();
        compute(&mut vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1], &mut input.as_bytes(), &mut output);
        let output: i32 = String::from_utf8(output).unwrap().trim().parse().unwrap();
        assert_eq!(output, 1);
    }
}