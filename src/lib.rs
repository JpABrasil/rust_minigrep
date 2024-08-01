use std::fs;//Para Ler Arquivos
use std::error::Error; //Melhorar o feedback de erros
use std::vec;
use std::env;
pub struct Config{
    pub query: String,
    pub file_path: String,
    pub ignore_case:bool
}

impl Config{//Usamos trait como um parametro aqui
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str>{ //'static é um tipo de lifetime
        args.next();
        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };
        let file_path= match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a file path")
        };
       /* if args.len() < 3{
            return Err("Precisamos de mais argumentos"); //Caso args tenha menos de 3 Strings
        }*/

       /*let query = args[1].clone();
        let file_path = args[2].clone();*/

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {query,file_path,ignore_case})
    }
}

pub fn run(config: Config) -> Result<(),Box<dyn Error>>{ //Função Retorna Result Unit ou Error; Box<dyn Error> é um trait dyn, mais detalhes no capítulo 17
    let contents = fs::read_to_string(config.file_path)?; //? faz a mesma função de panic! e retorna
    //println!("Texto: \n{contents}");
    let results = if config.ignore_case{
        search_case_insensitive(&config.query,&contents)
    } else{
        search(&config.query,&contents)
    };
    for line in results {
        println!("{line}")
    }
    Ok(())
}

pub fn search<'a>(query:&str,contents:&'a str) ->Vec<&'a str>{ //Aqui definimos que os dados retornardos por seach teram o mesmo lifetime de contents
    contents
        .lines()
        .filter(|line|line.contains(query)) //Filtra o iterator
        .collect()
    /*let mut results = Vec::new();
    for line in contents.lines(){//Metodo lines retorna um iterator
        if line.contains(query){
            results.push(line)
        }
    }
    results*/
}

pub fn search_case_insensitive<'a>(query:&str, contents:&'a str) ->Vec<&'a str>{
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust
safe,fast,productive.
Duct tape.";
        assert_eq!(vec!["safe,fast,productive."],search(query,contents));
    }

    #[test]
    fn case_sensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
safe,fast,productive.
Trust me.";
        assert_eq!(
            vec!["Rust:","Trust me."],
            search_case_insensitive(query,contents)
        );
    }

}