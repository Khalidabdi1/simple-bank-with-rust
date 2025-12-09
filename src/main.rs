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
            withdraw_account(&mut info);
        }else if choose==4 {
            transfer(&mut info)
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

   

    let  email:String=loop{

     print!("Write your email :");
    io::stdout().flush().unwrap();

    let mut e:String =String::new();
   
    io::stdin().read_line(&mut e).unwrap();

    let e=e.trim().to_string();

   if e.contains('@'){
   break e;


   }else {
      println!("Write your email :");
       println!("your email don't have @")
   }
};



    print!("Write your password :");
    io::stdout().flush().unwrap();

    let mut password :String=" ".to_string();
    io::stdin().read_line(&mut password).expect("not a pssword");
    let password:String=password.trim().to_string();


    let id =info.len() as i32+1;

    let user:Account=Account { 
        id :id,
         name:name.to_string(),
          email:email.clone(),
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
            // println!("the account is : {:?}", e);
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

    match info.get_mut(&account) {
        Some(e)=>{
           println!("account found 👍");
           //todo:fix this err
           if check(&e.password){
            start_deposit(e);
           };

        }
        None =>{
                       println!("account  not found 😔");

        }
    }

 

   






}


   // check password 
    fn check(real_password:&String)->bool{

        let mut count:i32=0;

        loop{
                    let mut password:String=String::new();

    print!("enter password for the account :");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut password).unwrap();

        let password :String=password.trim().to_string();

        if real_password==&password{
            println!("your password is correct 😀");
            return  true;

           
        }else if count==3 {
              println!("You have made 3 incorrect attempts ❌");
            return  false;
        }
        else {
            println!("your password is not correct 😔 ,Remaining attempts{}",3-count);
            count+=1;
        }
        }
    


    }



     fn start_deposit(info:&mut Account){
        println!("your account is {}",info.id);

        print!("Enter the amount you want to add to your account :");
        io::stdout().flush().unwrap();


        let mut amount:String=String::new();
        io::stdin().read_line(&mut amount).unwrap();

        let amount:f64= match amount.trim().parse(){
            Ok(e)=>e,
            Err(r)=>{
println!("error {}",r);
return;
            }
        };


        info.blance+=amount;

        println!("✅ Amount added successfully!");
         println!("💰 New balance: {}",info.blance);


        



    }



    fn withdraw_account(info:&mut HashMap<String,Account>){
        println!("start Withdraw from your account :");

        let mut account :String=String::new();
        print!("Enter your account number :");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut account).unwrap();
        let account:String =account.trim().to_string();

        match info.get_mut(&account) {
            Some(e)=>{
               if check(&e.password){
                    start_withdraw(e);
               }
            },
            None =>println!("not found this account {}",account)
        }


    }

    //todo fix error here 
    fn start_withdraw(info:&mut Account){
        let mut amount:String =String::new();
         print!("Enter the amount you want to withdraw :");
         io::stdout().flush().unwrap();

         io::stdin().read_line(&mut amount).unwrap();

         let amount:f64=amount.trim().parse().unwrap();

        if info.blance>amount{
            info.blance-=amount;
            println!("This amount was successfully withdrawn : {}",amount);
            println!("Your new balance : {}",info.blance);


        }
    }


fn transfer(info:&mut HashMap<String,Account>){
    let mut account:String=String::new();
    println!("Enter your bank account number :");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut account).unwrap();

    

    match info.get_mut(&account){
        Some(e)=>{
            if  check(&e.password){
                transfer_account(e);
            }
            
             
        },
        None=> println!("Account not found !")
    }
}

//todo:fix error here:
fn transfer_account(info:&mut HashMap<String,Account>){

    let mut account:String=String::new();


    print!("Enter the account you want to transfer to :");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut account).unwrap();

    let account:String=account.trim().to_string();

    match info.get(&account){
        Some(e)=>{
            println!("{}",e);
        }
        None=>println!("this account not found")
    }

}