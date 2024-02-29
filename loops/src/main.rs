fn main() {
    println!("Hello, world!");

    // forever();
    counter();
    loop_labels();
    loop_impl_of_while();
    while_loop();
    prac_for_loop();
}


fn forever(){
    loop {
        println!("runs forever!");
    }
}


fn counter(){
    let mut track:  u32 =  0;
    let count =  loop {
        track += 1;
        if track >  300{
            break (track + 40);
        }

    };

    println!("final value: {} ",  count);
}

fn loop_labels(){
    let mut count =  0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining =  10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2{
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}")
}

fn loop_impl_of_while(){
    let mut count =  1;

    loop{
        if count > 5{
            break;
        }

        count += 1;
    }

    println!("loop_impl_while prints {}",  count);
}


fn while_loop(){
    let mut countdown =  10;

    while countdown > 0 {
        countdown -= 1;
    }

    println!("LIFT OFF!")
}

fn prac_for_loop(){
    for number in 1..5 {
        println!("for {number}")
    }
}