fn main() {
    let mut x = 5;
    println!("{x}");    
    x = 6;


    const CONST_VAR: i32 = 1213223;

    let s = 12;
    let s = s + 1;

    println!("s: {s}");

    {
        let s    = s * 2;
        println!("s: {s}");
    }

    println!("s: {s}");

    let st = "salam";
    let st = st.len();

    // let mut st = "salam";
    // st = st.len();

    let d_n = 15;
    let h_n = 0xf;
    let o_n = 0o17;
    let b_n = 0b1111;
    println!("{d_n} {h_n} {o_n} {b_n}");

    let mut sample_u8: u8 = 255;

    // sample_u8 += 20;

    let sample_f64 = 2.4;
    let sample_f32: f32 = 2.1;

    let t = true;
    let f: bool = false;

    let tup: (i64, f32, bool) = (100, 2.3, false);
    let (t1, t2, t3) = tup;

    println!("{} {}", t2, tup.1);    

    let a = [1,1,2,3,4,5,6];

    let a2: [i32; 2] = [1,1];
    let a3 = [false; 5];
    println!("{}", a[2]);
    // println!("{}", a[10]);

    sample_function();
    sample_function2(12);

    println!("{}",  sample_function3(13));

    let x = 12;
    // comment1
    let y = {
        if x == 12 { // comment2
            5
        } else {
            6
        }
    };
    println!("y: {y}");


    let x = 12;
    if x == 12 {

    } else if x == 13 {

    } else {

    }

    let y = 15;
    
    let z = if y == 12 {false} else {true};

    let mut x = 0;
    let mut y = 0;
    'main_loop: loop {
        println!("x: {x}");
        x += 1;

        loop {
            println!("y : {y}");
            y += 1;

            if y % 10 == 0 {
                if y == 1000 {
                    break 'main_loop;       
                }

                break;
            }
        }
    }

    let mut counter = 0;
    let x = loop {
        counter += 1;
        if counter == 10 {      
            break counter;
        }
    };
    println!("{x}");        

    let mut n = 10;
    while n != 0 {
        n -= 1;
        println!("{n}");
    }

    for i in [1,2,4,4] {
        println!("{i}");
    }

    for j in (0..=10).rev() {
        println!("{j}");
    }       

}

fn sample_function() {
    println!("sample_function");
}

fn sample_function2(x: usize) {
    println!("{}", x);
}

fn sample_function3(x: usize) -> usize {
    println!("{}", x);

    2 * x
}