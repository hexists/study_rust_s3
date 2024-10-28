fn main() {
  for i in 1..=100{
    // [채우기] 문제  
	match (i%3 == 0, i%5 == 0) {
	  (true, true) => println!("FizzBuzz"), 
	  (true, false) => println!("Fizz"), 
	  (false, true) => println!("Buzz"), 
	  (_, _) => println!("{}", i), 
	}
  }
}
