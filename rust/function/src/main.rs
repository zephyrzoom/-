fn main() {
    println!("Hello, world!");
    // 函数名用小写字母组成下划线分割
    another_function(5, 15);
    let y = 6;
    let x = {
        let y = 6;
        y+1
    };
    println!("the value of x is {}", x);

    let x = five();
    println!("the value of x is {}", x);

    let x = plus_one(3);
    println!("the value of x is {}", x);

    let number = 12;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition {
        "5"
    } else {
        "6"
    };

    println!("The value of number is: {}", number);

    loop {
        println!("loop again!");
        break
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    assert_eq!(result, 20);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn another_function(x: i32, y: i32) {
    println!("the value of x is {}", x);
    println!("the value of y is {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}