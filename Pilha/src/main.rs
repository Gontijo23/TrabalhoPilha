use std::io::{self, Write};
use std::thread;
use std::time::Duration;

struct Processos{
    pid: i32,
    menSize: i32,
    timeExecution: i32,
    totalMen:i32,
    totalTime:i32,
}

impl Processos{
    fn new(pid: i32, menSize: i32, timeExecution: i32, totalMen:i32, totalTime:i32,)-> Processos{
        Processos{
            pid,
            menSize,
            timeExecution,
            totalMen,
            totalTime,
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

struct Pilha{
    numProcessos:i32,
    vecProcesso: Vec<Processos>,
    proxProcesso:i32,
    totalMen:i32,
    totalTime:i32,
}

impl Pilha{
    fn new() -> Pilha{
        Pilha{
            vecProcesso: Vec:: new(),
            proxProcesso: 0,
            numProcessos: 0,
            totalMen: 0,
            totalTime: 0,
        }
    }

    fn push(&mut self, menSize: i32, timeExecution: i32, totalMen:i32, totalTime: i32) {
        let processo = Processos {
            pid: self.proxProcesso,
            menSize,
            timeExecution,
            totalMen,
            totalTime
        };
        self.vecProcesso.push(processo);
        self.proxProcesso += 1;
        self.totalMen += menSize;
        self.totalTime += timeExecution;
    }
}


fn main(){
}

