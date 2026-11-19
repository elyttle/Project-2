fn hitchiker(list: &[i32])->bool{
    match list{
        []=>false,//no 42? false
        [42,..]=>true,//find 42? make true
        [_, rest@..]=> hitchiker(rest),//recurse

    }
}

fn main(){
    println!("{}", hitchiker(&[41,42,43]));
    println!("{}", hitchiker(&[1,2,3]));
    println!("{}", hitchiker(&[]));
}