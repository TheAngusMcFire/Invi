
struct SomeStruct
{
    test1 : u16,
    test2 : u64,
    sting : String
}

fn main() 
{
    let tst = SomeStruct {test1 : 25, test2 : 23, sting : String::from( "lol test")};

    test("this is a test", &tst);
}

fn test(msg : &str, stru : &SomeStruct)
{
    println!("Hello, world! {} {} {} {}",msg, stru.test1, stru.test2, stru.sting );
}