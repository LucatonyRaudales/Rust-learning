use std::string;

fn main() {
    let res = mostraData( "tony".to_string(), "dsa".to_string());
    print!("the data returned is active: {}, name: {} {} and number: {}", res.activo, res.first_name, res.last_name, res.numeros)
}

fn mostraData(first_name: String, last_name: String) -> Boleto{
    Boleto{
        first_name,
        last_name,
        numeros: 5,
        activo:true
    }
}

fn filtrarUnVectorDeEnteros(){
    let vs = vec![0, 1, 2, 3, 4];
    let sublist: Vec<i32> = vs.into_iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", sublist);
}


struct Boleto{
    first_name: String,
    last_name: String,
    numeros: u32,
    activo: bool
}


