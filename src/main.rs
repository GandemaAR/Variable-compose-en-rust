fn main() {
    //Arrays
    //Are a fixe size  collection of eléments of a same type
    let array: [i32; 6] = [1, 2, 3, 4, 5, 6];
    println!("Number array: {:?}", array);

    let motorbike: [&str; 5] = ["Sirus", "Mio", "Nano", "XoneR", "Jupiter"];
    println!("Motorbike éléments:{:?} ", motorbike);
    //println!("Motorbike éléments:");
    //for element in motorbike {print!(" {} ", element);}

    //Tuples
    let human = ("Ibrahim", 15, true);
    println!("Human tuple: {:?}", human);
}
