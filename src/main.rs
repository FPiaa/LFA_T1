use std::io::Read;
use clap::Parser;
use std::process;
use std::{
    collections::HashSet,
    fs::File,
    str::FromStr,
};

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
enum Símbolo {
    Cima,
    Baixo,
    Esquerda,
    Direita,
    Pegar,
}

impl FromStr for Símbolo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_ref() {
            "cima" | "c" => Ok(Self::Cima),
            "baixo" | "b" => Ok(Self::Baixo),
            "esquerda" | "e" => Ok(Self::Esquerda),
            "direita" | "d" => Ok(Self::Direita),
            "pegar" | "p" => Ok(Self::Pegar),
            _ => Err(()),
        }
    }
}

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
enum Estado {
    A1,
    A2,
    A3,
    A4,
    B1,
    B2,
    B3,
    B4,
    C1,
    C2,
    C3,
    C4,
    D1,
    D2,
    D3,
    D4,
    E1,
    E2,
    E3,
    E4,
    F1,
    F2,
    F3,
    F4,
    G1,
    G2,
    G3,
    G4,
    H1,
    H2,
    H3,
    H4,
}

const ALFABETO: [Símbolo; 5] = [
    Símbolo::Cima,
    Símbolo::Baixo,
    Símbolo::Direita,
    Símbolo::Esquerda,
    Símbolo::Pegar,
];

const ESTADOS: [Estado; 32] = [
    Estado::A1,
    Estado::A2,
    Estado::A3,
    Estado::A4,
    Estado::B1,
    Estado::B2,
    Estado::B3,
    Estado::B4,
    Estado::C1,
    Estado::C2,
    Estado::C3,
    Estado::C4,
    Estado::D1,
    Estado::D2,
    Estado::D3,
    Estado::D4,
    Estado::E1,
    Estado::E2,
    Estado::E3,
    Estado::E4,
    Estado::F1,
    Estado::F2,
    Estado::F3,
    Estado::F4,
    Estado::G1,
    Estado::G2,
    Estado::G3,
    Estado::G4,
    Estado::H1,
    Estado::H2,
    Estado::H3,
    Estado::H4,
];

type FunçãoTransição = dyn Fn(Estado, Símbolo) -> Option<Estado>;

struct Labirinto<'a> {
    alfabeto: HashSet<Símbolo>,
    estados: HashSet<Estado>,
    transição: &'a FunçãoTransição,
    inicial: Estado,
    finais: HashSet<Estado>,
}

fn transição(estado_atual: Estado, simbolo: Símbolo) -> Option<Estado> {
    use Estado::*;
    use Símbolo::*;

    // estados A* -> E* depois de pegar o tesouro
    // estados B* -> F* depois de pegar o tesouro
    // estados C* -> G* depois de pegar o tesouro
    // estados D* -> H* depois de pegar o tesouro

    //   a b c d
    // 1
    // 2
    // 3
    // 4
    //   e f g h

    match (estado_atual, simbolo) {
        (A1, Cima) => Some(A2),
        (A1, Direita) => Some(B1),
        (A1, Esquerda) => Some(A1),
        (A1, Baixo) => Some(A1),
        (A1, Pegar) => Some(A1),

        (A2, Cima) => Some(A3),
        (A2, Direita) => Some(B2),
        (A2, Esquerda) => Some(A2),
        (A2, Baixo) => Some(A1),
        (A2, Pegar) => Some(A2),

        (A3, Cima) => None,
        (A3, Direita) => None,
        (A3, Esquerda) => None,
        (A3, Baixo) => None,
        (A3, Pegar) => None,

        (A4, Cima) => Some(A4),
        (A4, Direita) => Some(B4),
        (A4, Esquerda) => Some(A4),
        (A4, Baixo) => Some(A3),
        (A4, Pegar) => Some(A4),

        (B1, Cima) => Some(B2),
        (B1, Direita) => Some(C1),
        (B1, Esquerda) => Some(A1),
        (B1, Baixo) => Some(B1),
        (B1, Pegar) => Some(B1),

        (B2, Cima) => Some(B3),
        (B2, Direita) => Some(C2),
        (B2, Esquerda) => Some(A2),
        (B2, Baixo) => Some(B1),
        (B2, Pegar) => Some(B2),

        (B3, Cima) => Some(B4),
        (B3, Direita) => Some(C3),
        (B3, Esquerda) => Some(A3),
        (B3, Baixo) => Some(B2),
        (B3, Pegar) => Some(F3),

        (B4, Cima) => Some(B4),
        (B4, Direita) => Some(C4),
        (B4, Esquerda) => Some(A4),
        (B4, Baixo) => Some(B3),
        (B4, Pegar) => Some(B3),

        (C1, Cima) => None,
        (C1, Direita) => None,
        (C1, Esquerda) => None,
        (C1, Baixo) => None,
        (C1, Pegar) => None,

        (C2, Cima) => Some(C3),
        (C2, Direita) => Some(D2),
        (C2, Esquerda) => Some(B2),
        (C2, Baixo) => Some(C1),
        (C2, Pegar) => Some(C2),

        (C3, Cima) => None,
        (C3, Direita) => None,
        (C3, Esquerda) => None,
        (C3, Baixo) => None,
        (C3, Pegar) => None,

        (C4, Cima) => Some(C4),
        (C4, Direita) => Some(D4),
        (C4, Esquerda) => Some(B4),
        (C4, Baixo) => Some(C3),
        (C4, Pegar) => Some(C4),

        (D1, Cima) => Some(D2),
        (D1, Direita) => Some(D1),
        (D1, Esquerda) => Some(C1),
        (D1, Baixo) => Some(D1),
        (D1, Pegar) => Some(D1),

        (D2, Cima) => Some(D3),
        (D2, Direita) => Some(D2),
        (D2, Esquerda) => Some(C2),
        (D2, Baixo) => Some(D1),
        (D2, Pegar) => Some(D2),

        (D3, Cima) => Some(D4),
        (D3, Direita) => Some(D3),
        (D3, Esquerda) => Some(C3),
        (D3, Baixo) => Some(D2),
        (D3, Pegar) => Some(D3),

        (D4, Cima) => None,
        (D4, Direita) => None,
        (D4, Esquerda) => None,
        (D4, Baixo) => None,
        (D4, Pegar) => None,

        (E1, Cima) => Some(E2),
        (E1, Direita) => Some(F1),
        (E1, Esquerda) => Some(E1),
        (E1, Baixo) => Some(E1),
        (E1, Pegar) => Some(E1),

        (E2, Cima) => Some(E3),
        (E2, Direita) => Some(F2),
        (E2, Esquerda) => Some(E2),
        (E2, Baixo) => Some(E1),
        (E2, Pegar) => Some(E2),

        // Wumpus te pegou na volta
        (E3, Cima) => None,
        (E3, Direita) => None,
        (E3, Esquerda) => None,
        (E3, Baixo) => None,
        (E3, Pegar) => None,

        (E4, Cima) => Some(E4),
        (E4, Direita) => Some(F4),
        (E4, Esquerda) => Some(E4),
        (E4, Baixo) => Some(E3),
        (E4, Pegar) => Some(E4),

        (F1, Cima) => Some(F2),
        (F1, Direita) => Some(G1),
        (F1, Esquerda) => Some(E1),
        (F1, Baixo) => Some(F1),
        (F1, Pegar) => Some(F1),

        (F2, Cima) => Some(F3),
        (F2, Direita) => Some(G2),
        (F2, Esquerda) => Some(E2),
        (F2, Baixo) => Some(F1),
        (F2, Pegar) => Some(F2),

        (F3, Cima) => Some(F4),
        (F3, Direita) => Some(G3),
        (F3, Esquerda) => Some(E3),
        (F3, Baixo) => Some(F2),
        (F3, Pegar) => Some(F3),

        (F4, Cima) => Some(F4),
        (F4, Direita) => Some(G4),
        (F4, Esquerda) => Some(E4),
        (F4, Baixo) => Some(F3),
        (F4, Pegar) => Some(F4),

        (G1, Cima) => None,
        (G1, Direita) => None,
        (G1, Esquerda) => None,
        (G1, Baixo) => None,
        (G1, Pegar) => None,

        (G2, Cima) => Some(G3),
        (G2, Direita) => Some(H2),
        (G2, Esquerda) => Some(F2),
        (G2, Baixo) => Some(G1),
        (G2, Pegar) => Some(G2),

        (G3, Cima) => None,
        (G3, Direita) => None,
        (G3, Esquerda) => None,
        (G3, Baixo) => None,
        (G3, Pegar) => None,

        (G4, Cima) => Some(G4),
        (G4, Direita) => Some(H4),
        (G4, Esquerda) => Some(F4),
        (G4, Baixo) => Some(G3),
        (G4, Pegar) => Some(G4),

        (H1, Cima) => Some(H2),
        (H1, Direita) => Some(H1),
        (H1, Esquerda) => Some(G1),
        (H1, Baixo) => Some(H1),
        (H1, Pegar) => Some(H1),

        (H2, Cima) => Some(H3),
        (H2, Direita) => Some(H2),
        (H2, Esquerda) => Some(G2),
        (H2, Baixo) => Some(H1),
        (H2, Pegar) => Some(H2),

        (H3, Cima) => Some(H4),
        (H3, Direita) => Some(H3),
        (H3, Esquerda) => Some(G3),
        (H3, Baixo) => Some(H2),
        (H3, Pegar) => Some(H3),

        (H4, Cima) => None,
        (H4, Direita) => None,
        (H4, Esquerda) => None,
        (H4, Baixo) => None,
        (H4, Pegar) => None,
    }
}

impl<'a> Labirinto<'a> {
    fn new() -> Self {
        Self {
            alfabeto: HashSet::from(ALFABETO),
            estados: HashSet::from(ESTADOS),
            inicial: Estado::A1,
            finais: HashSet::from([Estado::E1]),
            transição: &transição,
        }
    }

    fn read(&'a self, palavra: &'a [String]) -> LabirintoIter<'a> {
        LabirintoIter {
            palavra,
            estado_atual: self.inicial,
            transição: self.transição,
            alfabeto: &self.alfabeto,
            estados: &self.estados,
        }
    }
}

struct LabirintoIter<'a> {
    palavra: &'a [String],
    estado_atual: Estado,
    transição: &'a FunçãoTransição,
    alfabeto: &'a HashSet<Símbolo>,
    estados: &'a HashSet<Estado>,
}

impl<'a> Iterator for LabirintoIter<'a> {
    type Item = Estado;

    fn next(&mut self) -> Option<Self::Item> {
        let simbolo_str = self.palavra.first()?;
        let simbolo = Símbolo::from_str(simbolo_str);

        let simbolo = match simbolo {
            Ok(s) => s,
            Err(_) => {
                eprintln!("O símbolo \"{simbolo_str}\" não é reconhecido pelo alfabeto (Palavra rejeitada)");
                process::exit(-1);
            }
        };

        assert!(self.alfabeto.contains(&simbolo));

        self.palavra = &self.palavra[1..];
        let proximo = (self.transição)(self.estado_atual, simbolo);

        let proximo = match proximo {
            Some(p) => p,
            None => {
                println!("δ({:?}, {simbolo:?}) => —", self.estado_atual);
                return None;
            }
        };

        assert!(self.estados.contains(&proximo));

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
        .split(|c: char| c.is_whitespace() || c == ',' )
        .filter(|c| c != &"")
        .map(|simbolo| simbolo.trim().to_owned())
        .collect()
}

fn main() {
    let máquina = Labirinto::new();
    let args = Args::parse();

    let palavra = if args.filepath.is_some() {
        get_palavra(args.filepath.unwrap())
    } else {
        ["c", "c", "c", "c", "d", "d", "d", "d"]
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

    use crate::{Labirinto, Estado};

    impl<'a> Labirinto<'a> {
        
        fn new_inicial(inicial: Estado) -> Self {
            Self {
                inicial,
                .. Self::new()
            }
        }
    }
    fn split_input(s: &str) -> Vec<String> {
        s.chars().map(|c| c.to_string()).collect()
    }

    fn should_accept(palavra: &str, inicial: Estado) {
        let palavra = split_input(palavra);
        let maquina = Labirinto::new_inicial(inicial);

        let estado_final = maquina.read(&palavra).last().unwrap();
        assert!(maquina.finais.contains(&estado_final));
    }

    fn should_reject(palavra: &str, inicial: Estado) {
        let palavra = split_input(palavra);
        let maquina = Labirinto::new_inicial(inicial);

        let estado_final = maquina.read(&palavra).last().unwrap();
        assert!(!maquina.finais.contains(&estado_final));
    }

    #[test]
    fn aceita1() {
        let palavra = "cdcpbeb";
        should_accept(palavra, Estado::A1);
    }

    #[test]
    fn aceita2() {
        let palavra = "cdddbccbeecceddebpbbe";
        should_accept(palavra, Estado::A1);
    }

    #[test]
    fn aceita3() {
        let palavra = "decbcdcceddebbbcddcbbceecpceddebbddcbbceebceb";
        should_accept(palavra, Estado::A1);
    }

    #[test]
    fn aceita4() {
        let palavra = "ebcebdbcdddcdbbdbceecccdceecedbpbbe";
        should_accept(palavra, Estado::A1);
    }

    #[test]
    fn aceita5() {
        let palavra = "cdcpceecdcdcebbddcdbbbdceebbceebbe";
        should_accept(palavra, Estado::A1);
    }

    #[test]
    fn rejeita_sair_wumpus1() {
        let palavra = "d";
        let inicial = Estado::A1;
        should_reject(palavra, inicial);
        let palavra = "b";
        should_reject(palavra, inicial);
        let palavra = "c";
        should_reject(palavra, inicial);
        let palavra = "e";
        should_reject(palavra, inicial);
        let palavra = "p";
        should_reject(palavra, inicial);
    }

    #[test]
    fn rejeita_sair_wumpus2() {
        let palavra = "d";
        let inicial = Estado::A1;
        should_reject(palavra, inicial);
        let palavra = "b";
        should_reject(palavra, inicial);
        let palavra = "c";
        should_reject(palavra, inicial);
        let palavra = "e";
        should_reject(palavra, inicial);
        let palavra = "p";
        should_reject(palavra, inicial);
    }

    // entrou no quadradinho do wumpus antes de sair
    #[test]
    fn rejeita_entrar_e_sair_wumpus1() {
        let palavra = "cdpbbe";
        let inicial = Estado::A2;
        should_reject(palavra, inicial);

        let palavra = "edpbbe";
        let inicial = Estado::B3;
        should_reject(palavra, inicial);

        let palavra = "bdpbbe";
        let inicial = Estado::A4;
        should_reject(palavra, inicial);
    }

    // Encontrou o tesouro mas aentrou no quadradinho do wumpus

    #[test]
    fn rejeita_entrar_e_sair_wumpus2() {
        let palavra = "cbb";
        let inicial = Estado::E2;
        should_reject(palavra, inicial);

        let palavra = "ebb";
        let inicial = Estado::F3;
        should_reject(palavra, inicial);

        let palavra = "bbb";
        let inicial = Estado::E4;
        should_reject(palavra, inicial);
    }

    #[test]
    fn rejeita_buraco_c1() {
        let palavra = "dcecpbbe";
        let inicial = Estado::B1;
        should_reject(palavra, inicial);

        let palavra = "bcecpbbe";
        let inicial = Estado::C2;
        should_reject(palavra, inicial);

        let palavra = "edceecpbbe";
        let inicial = Estado::D1;
        should_reject(palavra, inicial);
    }

    #[test]
    fn rejeita_buraco_c3() {
        let palavra = "depbbe";
        let inicial = Estado::B3;
        should_reject(palavra, inicial);

        let palavra = "cbecpbbe";
        let inicial = Estado::C2;
        should_reject(palavra, inicial);

        let palavra = "edbeecpbbe";
        let inicial = Estado::D3;
        should_reject(palavra, inicial);
        
        let palavra = "bcebpbbe";
        let inicial = Estado::C4;
        should_reject(palavra, inicial);
    }

    #[test]
    fn rejeita_buraco_d4() {
        let palavra = "cbbeecpbbe";
        let inicial = Estado::D3;
        should_reject(palavra, inicial);
        
        let palavra = "deebpbbe";
        let inicial = Estado::C4;
        should_reject(palavra, inicial);
    }
    
    #[test]
    fn rejeita_buraco_g1() {
        let palavra = "dee";
        let inicial = Estado::F1;
        should_reject(palavra, inicial);

        let palavra = "bceeb";
        let inicial = Estado::G2;
        should_reject(palavra, inicial);

        let palavra = "edceeeb";
        let inicial = Estado::H1;
        should_reject(palavra, inicial);
    }

    #[test]
    fn rejeita_buraco_g3() {
        let palavra = "debbe";
        let inicial = Estado::F3;
        should_reject(palavra, inicial);

        let palavra = "cbeeb";
        let inicial = Estado::G2;
        should_reject(palavra, inicial);

        let palavra = "edbeeeb";
        let inicial = Estado::H3;
        should_reject(palavra, inicial);
        
        let palavra = "bcebbbe";
        let inicial = Estado::G4;
        should_reject(palavra, inicial);
    }

    #[test]
    fn rejeita_buraco_h4() {
        let palavra = "cbbeeeb";
        let inicial = Estado::H3;
        should_reject(palavra, inicial);
        
        let palavra = "deebbbe";
        let inicial = Estado::G4;
        should_reject(palavra, inicial);
    }

}
