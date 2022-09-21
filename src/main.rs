use std::{collections::HashSet, str::FromStr};

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
enum Simbolo {
    Cima,
    Baixo,
    Esquerda,
    Direita,
    Pegar,
}

impl FromStr for Simbolo {
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

const ALFABETO: [Simbolo; 5] = [
    Simbolo::Cima,
    Simbolo::Baixo,
    Simbolo::Direita,
    Simbolo::Esquerda,
    Simbolo::Pegar,
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

type FunçãoTransição = dyn Fn(Estado, Simbolo) -> Option<Estado>;

struct Labirinto<'a> {
    alfabeto: HashSet<Simbolo>,
    estados: HashSet<Estado>,
    transição: &'a FunçãoTransição,
    inicial: Estado,
    finais: HashSet<Estado>,
}

fn transição(estado_atual: Estado, simbolo: Simbolo) -> Option<Estado> {
    use Estado::*;
    use Simbolo::*;
    match (estado_atual, simbolo) {
        (A1, Cima) => Some(A2),
        (A1, Direita) => Some(B1),
        (A1, Esquerda) => Some(A1),
        (A1, Baixo) => Some(A1),
        (A1, Pegar) => Some(A1),
        (_, _) => None,
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

    fn iter(&'a self, palavra: &'a [&str]) -> LabirintoIter<'a> {
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
    palavra: &'a [&'a str],
    estado_atual: Estado,
    transição: &'a FunçãoTransição,
    alfabeto: &'a HashSet<Simbolo>,
    estados: &'a HashSet<Estado>
}

impl<'a> Iterator for LabirintoIter<'a> {
    type Item = Estado;

    fn next(&mut self) -> Option<Self::Item> {
        let simbolo = self.palavra.first()?;
        let simbolo = Simbolo::from_str(simbolo).ok()?;
        if !self.alfabeto.contains(&simbolo) {
            panic!("Simbolo do alfabeto é inválido")
        }
        self.palavra = &self.palavra[1..];
        let proximo = (self.transição)(self.estado_atual, simbolo)?;
        if !self.estados.contains(&proximo) {
            panic!("A função de transição gerou um estado inválido")
        }

        println!("δ({:?}, {simbolo:?}) => {proximo:?}", self.estado_atual);
        self.estado_atual = proximo;
        Some(self.estado_atual)
    }
}

fn main() {
    let machine = Labirinto::new();
    let palavra = ["esquerda", "baixo", "pegar", "direita", "esquerda"];
    println!("Estado inicial => {:?}", machine.inicial);

    for _ in machine.iter(&palavra) {}
}
