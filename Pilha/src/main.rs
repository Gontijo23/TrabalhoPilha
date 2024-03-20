use std::io::{self, Write};
use std::thread;
use std::time::Duration;

struct Processos{
    pid: u64,
    menSize: u64,
    timeExecution: u64,
}

impl Processos{
    fn new(pid: u64, menSize: u64, timeExecution: u64)-> Processos{
        Processos{
            pid,
            menSize,
            timeExecution,
        }
    }
    
    fn resumo(&self){
        println!("PID {}", self.pid)
    }
    fn executar(&self) {
        println!("Começando a execução do processo PID: {}", self.pid);
        for i in 0..self.timeExecution {
            println!(
                "Progresso: [{}] {}/{} segundos\r",
                "=".repeat((i + 1) as usize),
                i + 1,
                self.timeExecution
            );
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        println!("\nProcesso PID: {} concluído.", self.pid);
    }
}


fn main(){
}
