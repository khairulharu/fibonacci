fn main() {
    let mut fibonacci_number_n1: i32 = 0;
    let mut fibonacci_number_n2: i32 = 1;

    loop {
        println!("{fibonacci_number_n1}");
        println!("{fibonacci_number_n2}");

        fibonacci_number_n1 += fibonacci_number_n2;

        fibonacci_number_n2 += fibonacci_number_n1;


        if fibonacci_number_n2 > 100 {
            break;
        } 
    }
}
