fn main() {
    let contents =
        std::fs::read_to_string("day08/input").expect("Something went wrong reading the file");

    //     let contents = r#"
    // nop +0
    // acc +1
    // jmp +4
    // acc +3
    // jmp -3
    // acc -99
    // acc +1
    // jmp -4
    // acc +6
    //         "#
    //     .to_string();

    let instructions = contents
        .trim()
        .split('\n')
        .map(|instruction| instruction.trim())
        .map(|instruction| {
            (
                instruction[0..3].to_string(),
                match instruction[4..].parse::<isize>() {
                    Ok(number) => number,
                    Err(err) => {
                        panic!(
                            "value: ({}), error: ({})",
                            instruction[4..].to_string(),
                            err
                        );
                    }
                },
            )
        })
        .map(Some)
        .collect::<Vec<_>>();

    let mut idx: isize = 0;
    let mut acc = 0;
    let mut instructions_copy = instructions.clone();
    // if you encounter a None, terminate it,
    // because that means you've entered the infinite loop
    while let Some((opr, arg)) = &instructions_copy[idx as usize] {
        let old_idx = idx;
        match &opr[..] {
            "nop" => {
                idx += 1;
            }
            "acc" => {
                acc += arg;
                idx += 1;
            }
            "jmp" => {
                idx += *arg;
            }
            _ => unreachable!(),
        }
        // set the instruction to None to show that we've visited it before
        instructions_copy[old_idx as usize] = None;
    }
    println!("acc = {}", acc);

    // change the Nth nop or jmp you encounter
    // does it result in an infinite loop?
    // increment N and repeat the experiment
    let mut change_nop_jmp_at = 0;
    loop {
        let mut nop_jmp_encounters_remaining = change_nop_jmp_at;
        let mut idx: isize = 0;
        let mut acc = 0;
        let mut instructions_copy = instructions.clone();
        // if you encounter a None, terminate it,
        // because that means you've either entered the infinite loop
        // or you've changed the right instruction to escape
        while let Some(Some((opr, arg))) = instructions_copy.get(idx as usize) {
            let old_idx = idx;
            match &opr[..] {
                "nop" => {
                    // when you've got 0 left, that's Nth nop or jmp encountered
                    if nop_jmp_encounters_remaining == 0 {
                        // treat it as a jmp
                        idx += *arg;
                    } else {
                        idx += 1;
                    }
                    // decrement the number of encounters you've got left
                    nop_jmp_encounters_remaining -= 1;
                }
                "acc" => {
                    acc += arg;
                    idx += 1;
                }
                "jmp" => {
                    // when you've got 0 left, that's Nth nop or jmp encountered
                    if nop_jmp_encounters_remaining == 0 {
                        // treat it as a nop
                        idx += 1;
                    } else {
                        idx += *arg;
                    }
                    nop_jmp_encounters_remaining -= 1;
                }
                _ => unreachable!(),
            }
            // set the instruction to None to show that we've visited it before
            instructions_copy[old_idx as usize] = None;
        }
        if instructions_copy.get(idx as usize).is_none() {
            println!("acc = {}", acc);
            break;
        }
        change_nop_jmp_at += 1;
    }
}
