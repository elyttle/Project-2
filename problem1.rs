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

fn main(){
    println!("{}", hitchiker(&[41,42,43]));
    println!("{}", hitchiker(&[1,2,3]));
    println!("{}", hitchiker(&[]));

    println!("{:?}", everyother(&[1,2,3,4,5,6,7,8,9]));
    println!("{:?}", everyother(&[1,1,2,2,3,3,4,4,5,5];));
    println!("{:?}", everyother(&[1,2,3];));
}

