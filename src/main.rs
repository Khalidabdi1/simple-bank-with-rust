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
        println!("6. Deposit");
        println!("7. Exit");
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
            
        }else if choose==3 {
            
        }else if choose==4 {
            
        }else if choose==5 {
            
        }else if choose==6 {
           
        }else if choose==7 {
            break;
        }
        
    }
    
}


#[derive(Debug)]
struct Account{
    id:i32,
    name:String,
    email:String,
    blance:f64

}

fn create_account(info:&mut HashMap<String,Account>){
     let mut id:i32=0;


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
    io::stdin().read_line(&mut email).unwrap();
    let email=email.trim().to_string();


    let id =info.len() as i32+1;

    let user:Account=Account { 
        id :id,
         name:name.to_string(),
          email:email,
           blance:0.0
    };

 
 
    info.insert(user.id.to_string(), user);

      match info.get(&id.to_string()){
    Some(d)=>{
        println!("the name is : {}",d.name);
        println!("the email is : {}",d.email);
        println!("the blance is {}",d.blance);
         println!("The account has been created successfully 🏦");
    }
    None =>println!("not found")
   }
    



     println!("-------------------");
   

}
