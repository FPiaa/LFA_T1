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

        (A2, Cima) => Some(A3),
        (A2, Direita) => Some(B2),
        (A2, Esquerda) => Some(A2),
        (A2, Baixo) => Some(A1),
        (A2, Pegar) => Some(A2),


        (A3, Cima) => Some(A4),
        (A3, Direita) => Some(B3),
        (A3, Esquerda) => Some(A3),
        (A3, Baixo) => Some(A2),
        (A3, Pegar) => Some(A3),


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
        (B3, Pegar) => Some(B3),

        (B4, Cima) => Some(B4),
        (B4, Direita) => Some(C4),
        (B4, Esquerda) => Some(A4),
        (B4, Baixo) => Some(B3),
        (B4, Pegar) => Some(B3),
        
        (C1, Cima) => Some(C2),
        (C1, Direita) => Some(D1),
        (C1, Esquerda) => Some(B1),
        (C1, Baixo) => Some(C1),
        (C1, Pegar) => Some(C1),

        (C2, Cima) => Some(C3),
        (C2, Direita) => Some(D2),
        (C2, Esquerda) => Some(B2),
        (C2, Baixo) => Some(C1),
        (C2, Pegar) => Some(C2),

        (C3, Cima) => Some(C4),
        (C3, Direita) => Some(D3),
        (C3, Esquerda) => Some(B3),
        (C3, Baixo) => Some(C2),
        (C3, Pegar) => Some(C3),

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
        (D2, Baixo) => Some(D2),
        (D2, Pegar) => Some(D2),

        (D3, Cima) => Some(D4),
        (D3, Direita) => Some(D3),
        (D3, Esquerda) => Some(C3),
        (D3, Baixo) => Some(D2),
        (D3, Pegar) => Some(D3),

        (D4, Cima) => Some(D4),
        (D4, Direita) => Some(D4),
        (D4, Esquerda) => Some(C4),
        (D4, Baixo) => Some(D3),
        (D4, Pegar) => Some(D4),

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

        (E3, Cima) => Some(E4),
        (E3, Direita) => Some(F3),
        (E3, Esquerda) => Some(E3),
        (E3, Baixo) => Some(E2),
        (E3, Pegar) => Some(E3),

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

        (G1, Cima) => Some(G1),
        (G1, Direita) => Some(H1),
        (G1, Esquerda) => Some(F4),
        (G1, Baixo) => Some(G1),
        (G1, Pegar) => Some(G1),

        (G2, Cima) => Some(G3),
        (G2, Direita) => Some(H2),
        (G2, Esquerda) => Some(F2),
        (G2, Baixo) => Some(G1),
        (G2, Pegar) => Some(G2),

        (G3, Cima) => Some(G4),
        (G3, Direita) => Some(H3),
        (G3, Esquerda) => Some(F3),
        (G3, Baixo) => Some(G2),
        (G3, Pegar) => Some(G3),

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

        (H4, Cima) => Some(H4),
        (H4, Direita) => Some(H4),
        (H4, Esquerda) => Some(G4),
        (H4, Baixo) => Some(H3),
        (H4, Pegar) => Some(H4),
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
    let palavra = ["esquerda", "baixo", "pegar", "direita", "esquerda", "b", "c","c","c","c","d","d","b","e","e","b","b","b","e","e"];
    println!("Estado inicial => {:?}", machine.inicial);

    if let Some(last) = machine.iter(&palavra).last() {
        if machine.finais.contains(&last) {
            println!("Parabéns você achou o tesouro e saiu da caverna!!! (Palavra aceita)");
        }
        else if last == machine.inicial{
            println!("A sua incursão e você saiu da caverna sem o tesouro, tente novamente!!! (Palavra Rejeitada)");
        }
        else {
            println!("A sua incursão terminou e você ficou preso na caverna!! (Palavra Rejeitada)");
        }
    } else {
        println!("Você não fez nada(Rejeitado por preguiça)");
    }
}
