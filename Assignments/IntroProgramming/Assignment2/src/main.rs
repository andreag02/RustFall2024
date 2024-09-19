fn is_even(n:i32) -> bool{
    n%2 == 0
}

fn main(){
    let nums = [1,2,3,4,5,6,7,8,9,10];

    for &num in nums.iter(){
        if is_even(num){
            println!("{} is even", num);
        }
        else{
            println!("{} is odd", num);
        }

        if num % 3 == 0 && num % 5 == 0{
            println!("FizzBuzz");
        }
        else if num % 3 == 0{
            println!("Fizz");
        }
        else if num % 5 == 0{
            println!("Buzz");
        }
    }

    let mut sum = 0;
    let mut index = 0; 
    while index < nums.len(){
        sum += nums[index];
        index +=1;
    }
    println!("The sum of all numbers in the array is {}", sum);

    let mut max_num = nums[0];
    for &num in nums.iter(){
        if num > max_num{
            max_num = num;
        }
    }
    println!("The largest number in the array is {}", max_num);
}