use clap::Parser;
use std::fmt::Debug;
use std::hash::Hash;
use std::io::Read;
use std::process;
use std::{collections::HashSet, fs::File};

trait Alfabeto {}
trait Estados {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Símbolo {
    Cima,
    Baixo,
    Esquerda,
    Direita,
    Pegar,
    Atirar,
}

impl Alfabeto for Símbolo {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Estado {
    A11,
    A12,
    A13,
    A21,
    A22,
    A23,
    A31,
    A32,
    A33,
    B11,
    B12,
    B13,
    B21,
    B22,
    B23,
    B31,
    B32,
    B33,
    C11,
    C12,
    C13,
    C21,
    C22,
    C23,
    C31,
    C32,
    C33,
    D11,
    D12,
    D13,
    D21,
    D22,
    D23,
    D31,
    D32,
    D33,
}

impl Estados for Estado {}

impl TryFrom<&str> for Símbolo {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_ascii_lowercase().as_ref() {
            "cima" | "c" => Ok(Self::Cima),
            "baixo" | "b" => Ok(Self::Baixo),
            "esquerda" | "e" => Ok(Self::Esquerda),
            "direita" | "d" => Ok(Self::Direita),
            "pegar" | "p" => Ok(Self::Pegar),
            "atirar" | "a" => Ok(Self::Atirar),
            _ => Err(()),
        }
    }
}

struct Labirinto<'a, T, U>
where
    T: Alfabeto + TryFrom<&'a str> + Clone + Copy + Debug,
    U: Estados + Eq + Clone + Copy + Hash + Debug,
{
    transição: &'a dyn Fn(U, T) -> Option<U>,
    inicial: U,
    finais: HashSet<U>,
}

fn transição(estado_atual: Estado, simbolo: Símbolo) -> Option<Estado> {
    use Estado::*;
    use Símbolo::*;
    // A* -> wumpus vivo e tesouro não pego
    // B* -> wumpus vivo e tesouro pego
    // C* -> wumpus morto e tesouro não pego
    // D* -> wumpus morto e tesouro pego
    // buraco  32  wumpus
    // 21  tesouro  23
    // 11  12       buraco

    match (estado_atual, simbolo) {
        (A11, Cima) => Some(A21),
        (A11, Direita) => Some(A12),
        (A11, _) => Some(A11),

        (A12, Cima) => Some(A22),
        (A12, Direita) => Some(A13),
        (A12, Esquerda) => Some(A11),
        (A12, _) => Some(A12),

        (A13, _) => None,

        (A21, Cima) => Some(A31),
        (A21, Direita) => Some(A22),
        (A21, Baixo) => Some(A11),
        (A21, _) => Some(A21),

        (A22, Cima) => Some(A32),
        (A22, Direita) => Some(A23),
        (A22, Esquerda) => Some(A21),
        (A22, Baixo) => Some(A12),
        (A22, Pegar) => Some(B22),
        (A22, _) => Some(A22),

        (A23, Cima) => Some(A33),
        (A23, Esquerda) => Some(A22),
        (A23, Baixo) => Some(A13),
        (A23, _) => Some(A23),

        (A31, _) => None,

        (A32, Direita) => Some(A33),
        (A32, Esquerda) => Some(A31),
        (A32, Baixo) => Some(A22),
        (A32, _) => Some(A32),

        (A33, Atirar) => Some(C33),
        (A33, _) => Some(A33),

        (B11, Cima) => Some(B21),
        (B11, Direita) => Some(B12),
        (B11, _) => Some(B11),

        (B12, Cima) => Some(B22),
        (B12, Direita) => Some(B13),
        (B12, Esquerda) => Some(B11),
        (B12, _) => Some(B12),

        (B13, _) => None,

        (B21, Cima) => Some(B31),
        (B21, Direita) => Some(B22),
        (B21, Baixo) => Some(B11),
        (B21, _) => Some(B21),

        (B22, Cima) => Some(B32),
        (B22, Direita) => Some(B23),
        (B22, Esquerda) => Some(B21),
        (B22, Baixo) => Some(B12),
        (B22, _) => Some(B22),

        (B23, Cima) => Some(B33),
        (B23, Esquerda) => Some(B22),
        (B23, Baixo) => Some(B13),
        (B23, _) => Some(B23),

        (B31, _) => None,

        (B32, Direita) => Some(B33),
        (B32, Esquerda) => Some(B31),
        (B32, Baixo) => Some(B22),
        (B32, _) => Some(B32),

        (B33, Atirar) => Some(D33),
        (B33, _) => Some(B33),

        (C11, Cima) => Some(C21),
        (C11, Direita) => Some(C12),
        (C11, _) => Some(C11),

        (C12, Cima) => Some(C22),
        (C12, Direita) => Some(C13),
        (C12, Esquerda) => Some(C11),
        (C12, _) => Some(C12),

        (C13, _) => None,

        (C21, Cima) => Some(C31),
        (C21, Direita) => Some(C22),
        (C21, Baixo) => Some(C11),
        (C21, _) => Some(C21),

        (C22, Cima) => Some(C32),
        (C22, Direita) => Some(C23),
        (C22, Esquerda) => Some(C21),
        (C22, Baixo) => Some(C12),
        (C22, Pegar) => Some(D22),
        (C22, _) => Some(C22),

        (C23, Cima) => Some(C33),
        (C23, Esquerda) => Some(C22),
        (C23, Baixo) => Some(C13),
        (C23, _) => Some(C23),

        (C31, _) => None,

        (C32, Direita) => Some(C33),
        (C32, Esquerda) => Some(C31),
        (C32, Baixo) => Some(C22),
        (C32, _) => Some(C32),

        (C33, Esquerda) => Some(C32),
        (C33, Baixo) => Some(C23),
        (C33, _) => Some(C33),

        (D11, Cima) => Some(D21),
        (D11, Direita) => Some(D12),
        (D11, _) => Some(D11),

        (D12, Cima) => Some(D22),
        (D12, Direita) => Some(D13),
        (D12, Esquerda) => Some(D11),
        (D12, _) => Some(D12),

        (D13, _) => None,

        (D21, Cima) => Some(D31),
        (D21, Direita) => Some(D22),
        (D21, Baixo) => Some(D11),
        (D21, _) => Some(D21),

        (D22, Cima) => Some(D32),
        (D22, Direita) => Some(D23),
        (D22, Esquerda) => Some(D21),
        (D22, Baixo) => Some(D12),
        (D22, _) => Some(D22),

        (D23, Cima) => Some(D33),
        (D23, Esquerda) => Some(D22),
        (D23, Baixo) => Some(D13),
        (D23, _) => Some(D23),

        (D31, _) => None,

        (D32, Direita) => Some(D33),
        (D32, Esquerda) => Some(D31),
        (D32, Baixo) => Some(D22),
        (D32, _) => Some(D32),

        (D33, Esquerda) => Some(D32),
        (D33, Baixo) => Some(D23),
        (D33, _) => Some(D33),
    }
}

impl<'a, T, U> Labirinto<'a, T, U>
where
    T: Alfabeto + TryFrom<&'a str> + Clone + Copy + Debug,
    U: Estados + Eq + Clone + Copy + Hash + Debug,
{
    fn new(transição: &'a dyn Fn(U, T) -> Option<U>, inicial: U, finais: &[U]) -> Self {
        let finais = finais.iter().fold(HashSet::new(), |mut set, elem| {
            set.insert(*elem);
            set
        });

        Self {
            inicial,
            finais,
            transição,
        }
    }

    fn read<V: AsRef<str>>(&'a self, palavra: &'a [V]) -> LabirintoIter<'a, T, U, V> {
        LabirintoIter {
            palavra,
            estado_atual: self.inicial,
            transição: self.transição,
        }
    }
}

struct LabirintoIter<'a, T, U, V>
where
    T: Alfabeto + TryFrom<&'a str> + Clone + Copy + Debug,
    U: Estados + Eq + Clone + Copy + Hash + Debug,
    V: AsRef<str>,
{
    palavra: &'a [V],
    estado_atual: U,
    transição: &'a dyn Fn(U, T) -> Option<U>,
}

impl<'a, T, U, V> Iterator for LabirintoIter<'a, T, U, V>
where
    T: Alfabeto + TryFrom<&'a str> + Clone + Copy + Debug,
    U: Estados + Eq + Clone + Copy + Hash + Debug,
    V: AsRef<str>,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        let simbolo_str = self.palavra.first()?;
        let simbolo = T::try_from(simbolo_str.as_ref());

        let simbolo = match simbolo {
            Ok(s) => s,
            Err(_) => {
                eprintln!(
                    "O símbolo \"{}\" não é reconhecido pelo alfabeto (Palavra rejeitada)",
                    simbolo_str.as_ref()
                );
                process::exit(-1);
            }
        };

        self.palavra = &self.palavra[1..];
        let proximo = (self.transição)(self.estado_atual, simbolo);

        let proximo = match proximo {
            Some(p) => p,
            None => {
                println!("δ({:?}, {simbolo:?}) => —", self.estado_atual);
                return None;
            }
        };

        println!("δ({:?}, {simbolo:?}) => {proximo:?}", self.estado_atual);

        self.estado_atual = proximo;
        Some(self.estado_atual)
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(value_parser)]
    filepath: Option<String>,
}

fn get_palavra(filepath: String) -> Vec<String> {
    let file = File::open(filepath);
    let mut file = match file {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Erro para abrir o arquivo {:?}", e);
            process::exit(e.raw_os_error().unwrap_or(-1));
        }
    };
    let mut palavra = String::new();
    let _ = file.read_to_string(&mut palavra);
    palavra
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|c| c != &"")
        .map(|simbolo| simbolo.trim().to_owned())
        .collect()
}

fn main() {
    let máquina = Labirinto::new(&transição, Estado::A11, &[Estado::B11, Estado::D11]);
    let args = Args::parse();

    let palavra = if args.filepath.is_some() {
        get_palavra(args.filepath.unwrap())
    } else {
        ["c", "d", "p", "b", "e"]
            .iter()
            .map(|&c| c.to_owned())
            .collect()
    };

    println!("Estado inicial => {:?}", máquina.inicial);

    if let Some(last) = máquina.read(&palavra).last() {
        if máquina.finais.contains(&last) {
            println!("Parabéns você achou o tesouro e saiu da caverna!!! (Palavra aceita)");
        } else if last == máquina.inicial {
            println!("A sua incursão e você saiu da caverna sem o tesouro, tente novamente!!! (Palavra Rejeitada)");
        } else {
            println!("A sua incursão terminou e você ficou preso na caverna!! (Palavra Rejeitada)");
        }
    } else {
        println!("Você não fez nada(Rejeitado por preguiça)");
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{Estado, Labirinto};
    fn split_input(s: &str) -> Vec<String> {
        s.chars().map(|c| c.to_string()).collect()
    }

    fn should_accept(palavra: &str, inicial: Estado) {
        let palavra = split_input(palavra);

        let máquina = Labirinto::new(&transição, inicial, &[Estado::B11, Estado::D11]);

        let estado_final = máquina.read(&palavra).last().unwrap();
        assert!(máquina.finais.contains(&estado_final));
    }

    #[test]
    fn test_aceita2_mapa_acd() {
        let inicial = Estado::A11;
        let palavra = "dccdpebcadpacbdpacecpabcbddpaeeepadebcbdedcbcdedcbepcbdecdedbcbebcedbedecb";
        should_accept(palavra, inicial);
    }

    #[test]
    fn test_todas_menos_buracos_ab() {
        let inicial = Estado::A11;
        let palavra = "pabedbpacepabcdedbcddpaeccbpccpabddpaeepaedbbpacbepabecb";
        should_accept(palavra, inicial);
    }

    #[test]
    fn test_todas_menos_buracos_abd() {
        let inicial = Estado::A11;
        let palavra = "cdpcdpebcaecpadbdpacebpacbdebbpaceepadebpabecbde";
        should_accept(palavra, inicial);
    }
}
