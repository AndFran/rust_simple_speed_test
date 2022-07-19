use std::time::Instant;  

fn main() {                                                              
    let start = Instant::now();                                          
    let mut x: i64 = 0;                                                  
    loop {                                                               
        if x < 200_000_000                                               
        {                                                                
            x += 1;                                                      
        } else {                                                         
            break;                                                       
        }                                                                
    }                                                                    
    let duration = start.elapsed();                                      
    println!("Finished in {:?} with total: {}", duration, x);            
}                                                                        

