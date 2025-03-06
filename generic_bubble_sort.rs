/// Fonction générique pour le tri par bulle, compatible avec les flottants (`PartialOrd`)
fn bubble_sort<T: PartialOrd>(arr: &mut Vec<T>) {
    let n = arr.len();
    let mut swapped;

    for i in 0..n {
        swapped = false;

        for j in 0..(n - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

fn main() {
    // Tri d'entiers
    let mut numbers = vec![64, 34, 25, 12, 22, 11, 90];
    bubble_sort(&mut numbers);
    println!("Entiers triés : {:?}", numbers);

    // Tri de nombres flottants
    let mut floats = vec![3.2, 1.5, 4.7, 2.9, 0.8];
    bubble_sort(&mut floats);
    println!("Flottants triés : {:?}", floats);

    // Tri de chaînes de caractères
    let mut words = vec!["pomme", "banane", "cerise", "abricot"];
    bubble_sort(&mut words);
    println!("Mots triés : {:?}", words);
}

// Appel de la fonction main pour forcer l'affichage sous Jupyter par exemple
//main();
