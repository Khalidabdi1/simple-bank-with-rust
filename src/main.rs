use std::collections::HashMap;
use std::io;
use std::io::{Write};

/*
to do :
1. Create account
2. Deposit
3. Withdraw
4. Transfer
5. Show account
6. Exit

*/
fn main() {
       let mut info:HashMap<String,Account>=HashMap::new();

    println!("-------------- Welcome to bank --------------");

    loop{
        println!("1. Create account");
        println!("2. Deposit");
        println!("3. Withdraw");
        println!("4. Transfer");
        println!("5. Show account");
        println!("6. Exit");
        print!("Enter number to start : ");
        io::stdout().flush().unwrap();

        let mut choose:String =String::new();

        io::stdin()
        .read_line(&mut choose).unwrap();

        // change to number and handling error
        let choose:i32=match choose.trim().parse() {
            Ok(x)=>x,
            Err(_)=>{
                println!("Error is not number ");
                continue;

            }
        };


// choose ?
        if choose==1{
            create_account(&mut info);
            
        }else if choose==2 {
            deposit(&mut info);
            
        }else if choose==3 {
            
        }else if choose==4 {
            
        }else if choose==5 {
            show_account(&mut info);
            
        }else if choose==6 {
           break;
        }
        
    }
    
}


#[derive(Debug)]
struct Account{
    id:i32,
    name:String,
    email:String,
    blance:f64,
    password:String

}


// for create
fn create_account(info:&mut HashMap<String,Account>){
     let _id:i32=0;


   println!("-------------------");


    println!("Start to create your account ");

    print!("Write your name : ");
    io::stdout().flush().unwrap();

    let mut  name:String=String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name:String=name.trim().to_string();

    print!("Write your email :");
    io::stdout().flush().unwrap();

    let mut email:String =String::new();

loop{
    io::stdin().read_line(&mut email).unwrap();
    let email=email.trim().to_string();

   if email.contains('@'){

break;
   }else {
      println!("Write your email :");
       println!("your email don't have @")
   }
}



    print!("Write your password :");
    io::stdout().flush().unwrap();

    let mut password :String=" ".to_string();
    io::stdin().read_line(&mut password).expect("not a pssword");
    let password:String=password.trim().to_string();


    let id =info.len() as i32+1;

    let user:Account=Account { 
        id :id,
         name:name.to_string(),
          email:email,
          password:password,
           blance:0.0
    };

 
 
    info.insert(user.id.to_string(), user);
    println!("account is :{:?} ",info);

      match info.get(&id.to_string()){
    Some(d)=>{
        println!("the name is : {}",d.name);
        println!("the email is : {}",d.email);
        println!("the blance is {}",d.blance);
        println!("the password is : {}" ,d.password);
         println!("The account has been created successfully 🏦");
    }
    None =>println!("not found")
   }
    



     println!("-------------------");
   

}



// show account

fn show_account(info:&mut HashMap<String,Account>){

    println!("To view your account information, please enter your account number and password :");

    print!(" please enter your account number : ");
    io::stdout().flush().unwrap();

    let mut account:String=String::new();
    io::stdin().read_line(&mut account).unwrap();

    let account =account.trim().to_string();

    match info.get(&account){
        Some(e) =>{
            println!("account found");
            println!("the account is : {:?}", e);
            check(&e.password,e);

        }
        None =>println!("not found this account")
    }

    // ask for password

    fn check(pass:&String,info:&Account){
        
        let mut password:String=String::new();
        print!("write the password :");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut password).unwrap();

        let password:String=password.trim().to_string();

        if password==*pass {
            println!("Your account information is :");
            println!("your name is {} and your blance is :{}", info.name,info.blance);

        }else {
            println!("the password not correct !")
        }

    }



}



// Deposit

fn deposit(info:&mut HashMap<String,Account>){
    println!("If you wish to make a deposit, please provide the following information.");

    let mut account:String=String::new();
    print!("write your account number : ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut account).unwrap();
    let account:String=account.trim().to_string();

    match info.get(&account) {
        Some(e)=>{
           println!("account found 👍");
           //todo:fix this err
           check(&e.password,&info);

        }
        None =>{
                       println!("account  not found 😔");

        }
    }

    // check password 
    fn check(pass:&String,info: &mut HashMap<String, Account>){
        let mut password:String=String::new();

        let mut count:i32=0;

        loop{
    print!("enter password for the account :");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut password).unwrap();

        let password :String=password.trim().to_string();

        if pass==&password{
            println!("your password is correct 😀");
            start_deposit(info);

            break;
        }else if count==3 {
              println!("You have made 3 incorrect attempts ❌");
              break;
        }
        else {
            println!("your password is not correct 😔 ,Remaining attempts{}",3-count);
            count+=1;
        }
        }
    


    }

    fn start_deposit(info:&mut HashMap<String,Account>){

    }






}