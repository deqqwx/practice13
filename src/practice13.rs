fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        return None; // Невозможно
    }

    let target = total / n;
    let mut moves = 0;
    let mut balance = 0;

    for &shipment in shipments {
        balance += shipment as i32 - target as i32;
        moves += balance.abs();
    }

    Some(moves as usize)
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut shipments = vec![4; n]; // середнє число 4
    shipments[0] += 4; // зміна для демонстрації
    shipments[1] -= 2; // зберігаючи загальну кількість
    shipments
}
#[test]
fn main() {
    let shipments = gen_shipments(5);
    println!("{:?}", shipments); // Показує згенеровані вантажі

    match count_permutation(&shipments) {
        Some(moves) => println!("Мінімальна кількість переносу: {}", moves),
        None => println!("Рівномірний розподіл неможливий"),
    }
}
