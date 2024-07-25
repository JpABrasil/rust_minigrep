use std::fs;//Para Ler Arquivos
use std::error::Error; //Melhorar o feedback de erros

pub struct Config{
    pub query: String,
    pub file_path: String,
}

impl Config{
    pub fn build(args: &[String]) -> Result<Config, &'static str>{ //'static é um tipo de lifetime
        if args.len() < 3{
            return Err("Precisamos de mais argumentos"); //Caso args tenha menos de 3 Strings
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query,file_path})
    }
}

pub fn run(config: Config) -> Result<(),Box<dyn Error>>{ //Função Retorna Result Unit ou Error; Box<dyn Error> é um trait dyn, mais detalhes no capítulo 17
    let contents = fs::read_to_string(config.file_path)?; //? faz a mesma função de panic! e retorna
    //println!("Texto: \n{contents}");
    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
        safe,fast,productive.
        Pick three.";
        assert_eq!(vec!["safe,fast,productive."],search(query,contents));
    }

    pub fn search<'a>(query:&str,contents:&'a str) ->Vec<&'a str>{ //Aqui definimos que os dados retornardos por seach teram o mesmo lifetime de contents
        let mut results = Vec::new();
        for line in contents.lines(){//Metodo lines retorna um iterator
            if line.contains(query){
                results.push(line)
            }
        }
        results
    }
}