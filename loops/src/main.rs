fn main() {
    // Calculate the sum of all even numbers up to and including the number 20
    // Loop
    {
        let mut sum = 0;
        let mut num = 2;
        loop {
            if num > 20 {
                break;
            }
            sum += num;
            num += 2;
        }
        println!("Loop Statement: Sum = {}", sum);
    }

    // while loop
    {
        let mut sum = 0;
        let mut num = 2;
        while num <= 20 {
            sum += num;
            num += 2;
        }
    
        println!("While Statement: Sum = {}", sum);
    }

    // for loop
    {
        let mut sum = 0;
        for i in 0..=20 {
            if i % 2 == 0 {
                sum += i;
            }
        }
        println!("For Statement: Sum = {}", sum)
    }

    // for with step
    {
        let mut sum = 0;
        for i in (0..=20).step_by(2) {
            sum += i;
        }
        println!("For with step: Sum = {}", sum)
    }

    // closure
    {
        let sum = (0..=20).reduce(|acc, el| acc + el).unwrap_or(0);
        println!("Closure statement: sum = {}", sum);
    }

    // short hand
    {
        let sum: i32 = (0..20).step_by(2).sum();
        println!("Idiomatic Rust: sum = {}", sum);
    }

    // loop over a string with ownership
    {
        let mut words = [String::from("This "), String::from("is "), String::from("Sparta")];
        let mut sentence = String::new();
        for s in &mut words {
            *s = s.to_uppercase();
        }
        for s in &words {
            sentence.push_str(s);
        }
        println!("the sentence is {:?}", sentence);
        println!("the words were is {:?}", words);
    }
}
