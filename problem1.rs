fn hitchiker(list: &[i32])->bool{
    match list{
        []=>false,//no 42? false
        [42,..]=>true,//find 42? make true
        [_, rest@..]=> hitchiker(rest),//recurse

    }
}

fn everyother(list:&[i32])->Vec<i32>{
    match list{
        []=>vec![], //check if empty
        [_]=>vec![],//not empty but one element left?
        [_,y,x@..]=>{ //drop first keep second
            let mut result = vec![*y];
            result.extend(everyother(x)); //change list and recurse
            result
        }
    }
}

fn divisible(x: i32, y: i32) -> bool{
    match (y* y > x, x%y==0){
        (true, _) => false, //doesn't divide
        (false, true) => true, //divides
        (false, false) => divisible(x, y +1), //recursion
    }
}

//checks qualifications for prime status
fn prime(x: i32) -> bool{
    match x > 1{
        false => false,
        true => !divisible(x,2),
    }
}

// finds the prime numbers that make up the given int
fn buddies(j: i32, k: i32) -> i32{
    match (prime(k), prime(j-k)){
        (true, true) => k,
        _ => buddies(j,k+1),
    }
}

// prints either the equation of the 2 primes or an error
fn goldbach(n: i32){
    match (n > 2, n % 2){
        (true, 0) => {
            let p = buddies(n,2);
            println!("{} + {} = {}", p, n-p , n);
        }
        _=>print!("Error: input must be an even int > 2"),
    }
}


fn main(){
    println!("{}", hitchiker(&[41,42,43]));
    println!("{}", hitchiker(&[1,2,3]));
    println!("{}", hitchiker(&[]));

    println!("{:?}", everyother(&[1,2,3,4,5,6,7,8,9]));
    println!("{:?}", everyother(&[1,1,2,2,3,3,4,4,5,5]));
    println!("{:?}", everyother(&[1,2,3]));

    goldbach(34);
    goldbach(200);
    goldbach(88);
}

