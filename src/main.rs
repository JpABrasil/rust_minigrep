//Globally Search a Regular Expression and Print
use std::env; //Para Ler Argumentos
use std::process;//Fechar processos
use grep::Config; //Importando definições criadas no arquivo lib.rs
fn main() {
    let args: Vec<String> = env::args().collect(); //Collect transforma o iterator args() em um vetor
    let config = Config::build(&args).unwrap_or_else(|err| { //unwrap_or_else retorna o Ok caso não haja erro e podemos personalizar o que sairá no erro
        //println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    //println!("Buscando: {}",config.query);
    //println!("No arquivo: {}",config.file_path);
    if let Err(e) = grep::run(config){ //Usamos if let no lugar de unwrap_or_else porque queremos apenas identificar caso aja erro, já que em caso de sucesso nada será retornardo pela função
        //println!("Application error: {e}");
        process::exit(1)
    }

}

/*Função e Struct
//Struct para receber a query e o camiinho para o arquivo
struct Config{
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config{
    let query = args[1].clone(); //Usamos clone porque o struct Config utiliza String sem & o que tomaria o ownership
    let file_path = args[2].clone(); //Usar clone piora um pouco em performance e memória
    Config { query , file_path }
}

 */
